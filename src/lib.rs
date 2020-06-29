use wasm_bindgen::prelude::*;
use cubic_spline::{Spline, SplineOpts};
use bresenham::Bresenham;
use rand::Rng;
use std::cmp::min;
extern crate console_error_panic_hook;
use std::panic;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

const STRANGER_START: usize = 20;
const STRANGER_END: usize = WIDTH - 20;

const MAX_HEIGHT: usize = 80;
const MIN_HEIGHT: usize = HEIGHT - 100;

const ANCHOR_1_MIN_X: usize = STRANGER_START + 80;
const ANCHOR_1_MAX_X: usize = WIDTH / 2 - 50;
const ANCHOR_2_MIN_X: usize = WIDTH / 2 + 50;
const ANCHOR_2_MAX_X: usize = STRANGER_END - 50;

const MAX_THICC: usize = 35;
const MIN_THICC: usize = 15;

const EYE_START_MIN: usize = STRANGER_START + 20;
const EYE_START_MAX: usize = STRANGER_START + 60;
const EYE_END_MIN: usize = STRANGER_START + 70;
const EYE_END_MAX: usize = STRANGER_START + 100;

const MIN_EYE_WIDTH: usize = 4;

const MIN_LEG_WIDTH: usize = 3;
const MAX_LEG_WIDTH: usize = 8;
    
const EYE_LEG_DISTANCE: usize = 30;
const NEAR_FAR_LEG_DISTANCE_RATIO: usize = 2;
const MIN_FORE_HIND_LEG_DISTANCE: usize = 100;
const MAX_FORE_HIND_LEG_DISTANCE: usize = 180;

const FAR_FOOT_HEIGHT: usize = HEIGHT - 30;
const NEAR_FOOT_HEIGHT: usize = HEIGHT - 25;

const MAX_IRIS_RADIUS: usize = EYE_END_MIN - EYE_START_MAX + 2;
const MIN_IRIS_RADIUS: usize = 4;

const MIN_STRIPE_WIDTH: usize = 25;
const MAX_STRIPE_WIDTH: usize = 35;

const MAX_STRIPE_START: usize = (STRANGER_END - STRANGER_START) * 4 / 5;

const MAX_GRADIENT_START: usize = MAX_STRIPE_START;

type Color = (u8, u8, u8, u8);

struct Palette {
    body: Color,
    sclera: Color,
    iris: Color,
    stripe_outline: Color,
    stripe: Color,
}

const BLACK: Color = (0, 0, 0, 255);

type DrawMode = dyn Fn(usize, usize, Color) -> bool;

struct Canvas {
    pixels: [[Color; WIDTH]; HEIGHT]
}

struct StrangerParams {
    palette: Palette,
    core_anchors: [(usize, usize); 4],
    dorsal_anchors: [(usize, usize); 4],
    ventral_anchors: [(usize, usize); 4],
    eye_start_x: usize,
    eye_end_x: usize,
    iris_radius: usize,
    leg_width: usize,
    fore_hind_dist: usize,
    stripes: Option<(usize, usize)>,
    gradient: Option<usize>
}

const LAYER_OVER: &DrawMode = &(|_x, _y, _c| true);

const LAYER_UNDER: &DrawMode = &(|_x, _y, c| c.3 < 255);

/*
fn dither_1(x: usize, y: usize, _c: Color) -> bool {
   (x - y)%4 == 0 && (x - y)%4 == 0) 
}
 */

const dither_1: &DrawMode = &(|x, y, _c|  (x - y)%4 == 0 && (x - y)%4 == 0);

#[wasm_bindgen]  
pub fn width() -> usize { WIDTH }

#[wasm_bindgen]  
pub fn height() -> usize { HEIGHT }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/*    
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
*/

#[wasm_bindgen]  
pub fn render_stranger() ->  *const u8 {
    let bg_color = (100, 100, 100, 100);
    let mut canvas = Canvas::new();
    let sp = StrangerParams::new();    
    let core = make_spline(sp.core_anchors);
    let dorsal = make_spline(sp.dorsal_anchors);
    let ventral = make_spline(sp.ventral_anchors);
    &mut canvas.draw_spline(&dorsal);
    &mut canvas.draw_spline(&ventral);
    &mut canvas.flood_fill(sp.core_anchors[2].0,
			   sp.core_anchors[2].1,
			   sp.palette.body,
			   &vec![bg_color],
			   LAYER_OVER
    );

    if let Some(start) = sp.gradient {
	
    }

    if let Some((start, width)) = sp.stripes {
	let mut fill_points = Vec::new();
	for (i, &(x, y)) in core.iter().skip(start).step_by(width).enumerate(){
	    &mut canvas.draw_perpendicular_line(&core, 
						x, y, sp.palette.stripe_outline,
						sp.palette.body);
	    if i % 2 == 0 {
		fill_points.push((x + 1, y_at_x(&core, x + 1)));
	    }
	}
	for (x, y) in fill_points {
	   
	    &mut canvas.flood_fill(x,
				   y,
				   sp.palette.stripe,
				   &vec![sp.palette.body],
				   LAYER_OVER

	    );
	}
    }
    
    let eye_mid_x = (sp.eye_start_x + sp.eye_end_x) / 2;
    let max_eye_width = min
			 (y_at_x(&dorsal, eye_mid_x) - y_at_x(&core, eye_mid_x),
			  y_at_x(&core, eye_mid_x) - y_at_x(&ventral, eye_mid_x))
	* 3 / 5;
    let (top_eye_anchors, bottom_eye_anchors) =
	generate_eye_anchors((sp.eye_start_x, y_at_x(
	    &core, sp.eye_start_x)),
			     (sp.eye_end_x, y_at_x(&core, sp.
						   eye_end_x)), max_eye_width);
    let top_eye = make_spline(top_eye_anchors);
    let bottom_eye = make_spline(bottom_eye_anchors);
    &mut canvas.draw_spline(&top_eye);
    &mut canvas.draw_spline(&bottom_eye);
let eye_mid_y = (y_at_x(&top_eye, eye_mid_x) + y_at_x(&bottom_eye, eye_mid_x)) / 2;

    &mut canvas.flood_fill(eye_mid_x, eye_mid_y,
			   sp.palette.sclera,
			   &vec![sp.palette.body, sp.palette.stripe,
				 sp.palette.stripe_outline],
			   LAYER_OVER
    );
    
    
    &mut canvas.draw_iris((eye_mid_x, eye_mid_y),
			  sp.iris_radius,
			  sp.palette.iris);
    
    let leg_start_x = sp.eye_end_x + EYE_LEG_DISTANCE;
    &mut canvas.draw_leg(leg_start_x, &core, FAR_FOOT_HEIGHT, sp.palette.body,
			 sp.leg_width, LAYER_UNDER);

    
    let leg2_start_x = leg_start_x + NEAR_FAR_LEG_DISTANCE_RATIO * sp.leg_width;
    &mut canvas.draw_leg(leg2_start_x, &dorsal, NEAR_FOOT_HEIGHT, sp.palette.body,
			 sp.leg_width, LAYER_OVER);

    let leg3_start_x = leg_start_x + sp.fore_hind_dist;
    &mut canvas.draw_leg(leg3_start_x, &dorsal, FAR_FOOT_HEIGHT, sp.palette.body,
			 sp.leg_width, LAYER_UNDER);
    
    let leg4_start_x = leg3_start_x + NEAR_FAR_LEG_DISTANCE_RATIO * sp.leg_width;
    &mut canvas.draw_leg(leg4_start_x, &dorsal, NEAR_FOOT_HEIGHT, sp.palette.body,
			 sp.leg_width, LAYER_OVER);
    
    canvas.raw_pixels()
}

fn make_spline(upoints: [(usize, usize); 4]) -> Vec<(usize, usize)> {
    let opts: SplineOpts = Default::default();
    let mut points = vec![(0.0, 0.0); 4];
    for i in 0..4 {
	points[i] = (upoints[i].0 as f64, upoints[i].1 as f64);
    }
    let spline_points = Spline::from_tuples(&points, &opts);
    let spline_points_int : Vec<(isize, isize)> = spline_points.into_iter().map(|(x, y)| (x as isize, y as isize)).collect();
    let mut final_spline: Vec<(usize, usize)> = Vec::new();
    for i in 0..spline_points_int.len() - 1 {
	for (x, y) in Bresenham::new(spline_points_int[i], spline_points_int[i+1]) {
	    final_spline.push((x as usize, y as usize));
	}
    }
    final_spline  
}

fn y_at_x (spline: &[(usize, usize)], x0: usize) -> usize {
    let mut result = 0;
    for (x, y) in spline.iter() {
	if *x == x0 { result = *y }
    }
    result
}

impl StrangerParams {
    fn new() -> StrangerParams {
	let mut rng = rand::thread_rng();

	let palette = generate_palette();
	
	let x0 = STRANGER_START;
	let y0 = rng.gen_range(MAX_HEIGHT, MIN_HEIGHT);
	let x1 = rng.gen_range(ANCHOR_1_MIN_X, ANCHOR_1_MAX_X);
	let c_y1 = rng.gen_range(MAX_HEIGHT, MIN_HEIGHT);
	let x2 = rng.gen_range(ANCHOR_2_MIN_X, ANCHOR_2_MAX_X);
	let c_y2 = rng.gen_range(MAX_HEIGHT, MIN_HEIGHT);
	let x3 = STRANGER_END;
	let y3 = rng.gen_range(10, 140);

	let d_y1 = c_y1 + rng.gen_range(MIN_THICC, MAX_THICC);
	let d_y2 = c_y2 + rng.gen_range(MIN_THICC, MAX_THICC);

	let v_y1 = c_y1 - rng.gen_range(MIN_THICC, MAX_THICC);
	let v_y2 = c_y2 - rng.gen_range(MIN_THICC, MAX_THICC);

	let core_anchors = [(x0, y0), (x1, c_y1), (x2, c_y2), (x3, y3)];
	let dorsal_anchors = [(x0, y0), (x1, d_y1), (x2, d_y2), (x3, y3)];
	let ventral_anchors = [(x0, y0), (x1, v_y1), (x2, v_y2), (x3, y3)];

	let eye_start_x = rng.gen_range(EYE_START_MIN, EYE_START_MAX);
	let eye_end_x = rng.gen_range(EYE_END_MIN, EYE_END_MAX);

	let iris_radius = rng.gen_range(MIN_IRIS_RADIUS, MAX_IRIS_RADIUS);

	let leg_width = rng.gen_range(MIN_LEG_WIDTH, MAX_LEG_WIDTH);

	let fore_hind_dist = rng.gen_range(MIN_FORE_HIND_LEG_DISTANCE,
					   MAX_FORE_HIND_LEG_DISTANCE);
	let stripes =
	    if rng.gen_range(0, 4) > 0	{
		let width = rng.gen_range(MIN_STRIPE_WIDTH, MAX_STRIPE_WIDTH);
		let start = if rng.gen_range(0, 3) > 0 {
		    rng.gen_range(width, MAX_STRIPE_START)
		}
		else { width };
		
		Some((start, width))
	    } else { None };

	let gradient =
	    if rng.gen_range(1, 4) > 0 {
		Some(rng.gen_range(0, MAX_STRIPE_START))
	    } else { None };

	StrangerParams {
	    palette,
	    core_anchors,   
	    dorsal_anchors,
	    ventral_anchors,
	    eye_start_x,
	    eye_end_x,
	    iris_radius,
	    leg_width,
	    fore_hind_dist,
	    stripes,
	    gradient
	}	
    }
}

fn get_slope(spline: &Vec<(usize, usize)>, x: usize) -> f32 {
    let sample_dist = 5;
    let y_prev = y_at_x(spline, x - sample_dist);
    let y_post = y_at_x(spline, x + sample_dist);
    /*
    console_log!("y_prev = {}, y_post = {}", y_prev, y_post);
    console_log!("result = {}",  (y_post as f32 - y_prev as f32 )/ (sample_dist * 2) as f32);
*/
    (y_post as f32 - y_prev as f32 )/ (sample_dist * 2) as f32
}

fn scale_value(c: Color, scale: u8) -> Color {
    ((c.0 * scale) / 100, (c.1 * scale) / 100, (c.2 * scale) / 100, c.3)
}

fn generate_palette() -> Palette { 
    let mut rng = rand::thread_rng();
    let body =  (rng.gen_range(0, 0xFF), rng.gen_range(0, 0xFF),
			rng.gen_range(0, 0xFF), 0xFF);
    let split_dist = rng.gen_range(0, 0xFF / 2);
    let sclera = if rng.gen_range(0, 3) > 0 { (0xFF, 0xFF, 0xFF, 0xFF) }
    else { (0xFF - (body.0 - split_dist),
	    0xFF - (body.1 - split_dist),
	    0xFF - (body.2 - split_dist),
	    0xFF) };
    let iris = (0xFF - (body.0 + split_dist),
		   0xFF - (body.1 + split_dist),
		   0xFF - (body.2 + split_dist),
		   0xFF);
    let stripe = (0xFF - iris.0,
		  0xFF - iris.1,
		  0xFF - iris.2,
		  0xFF);
    let stripe_outline = scale_value(stripe, 50);
    
    Palette { body, sclera, iris, stripe, stripe_outline }
}

fn generate_eye_anchors(eye_start: (usize, usize), eye_end: (usize, usize), max_width: usize) ->
    ( [(usize, usize); 4], [(usize, usize); 4] ) {
	let mut rng = rand::thread_rng();
	let x = (eye_start.0 + eye_end.0) / 2;
	let y_mid = (eye_start.1 + eye_end.1) / 2;
	let y_top = y_mid + rng.gen_range(MIN_EYE_WIDTH, max_width);
	let y_bottom = y_mid - rng.gen_range(MIN_EYE_WIDTH, max_width);
	([eye_start, (x, y_top), eye_end, eye_end],
	 [eye_start, (x, y_bottom), eye_end, eye_end])
    }

fn perpendicular_line_coefficients(core_ptr: &Vec<(usize, usize)>,
				   x: usize, y: usize) -> Option<(f32, f32)> {
    let slope = get_slope(core_ptr, x);
    let m = -(1.0 / slope);
    if m.is_infinite() { None }
    else {
	Some((m, y as f32 - (m * x as f32)))
    }
}

impl Canvas {
    			      
    fn new() -> Canvas {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	Canvas {
	    pixels: [[(100, 100, 100, 100); WIDTH]; HEIGHT]
	}
    }
    
    fn raw_pixels(&self) -> *const u8 {
	&self.pixels[0][0].0 as *const u8
    }

    fn mark_pixel(&mut self, x: usize, y: usize, color: Color, dm: &DrawMode) {
	if dm(x, y, self.pixels[y][x]) {
	    self.pixels[y][x] = color;
	}	
    }

    fn draw_spline(&mut self, spline: &Vec<(usize, usize)>) {
	for (x, y) in spline.iter() {
	    self.pixels[*y][*x] = BLACK;
	}
    }

    fn draw_leg(&mut self,
		leg_start_x: usize,
		dorsal: &Vec<(usize, usize)>,
		foot: usize,
		color: (u8, u8, u8, u8),
		leg_width: usize,
		dm: &DrawMode) {
	let leg_start_y = y_at_x(dorsal, leg_start_x);
	let leg_end_x = leg_start_x + leg_width;
	let leg_end_y = y_at_x(dorsal, leg_end_x);
	self.draw_vertical_line(leg_start_x, leg_start_y, foot, BLACK, dm);
	self.draw_vertical_line(leg_end_x, leg_end_y, foot, BLACK, dm);
	self.draw_horizontal_line(foot + 1, leg_start_x + 1, leg_end_x - 1, BLACK, dm);
	let max_anchor = min (leg_start_y, leg_end_y);
	self.fill_rect((leg_start_x + 1, max_anchor - 1), (leg_end_x - 1, foot), color, dm);
    }
    
    fn draw_vertical_line(&mut self, x: usize, start_y: usize, end_y: usize,
			  color: Color, dm: &DrawMode) {
	for y in start_y..(end_y + 1) {
	    self.mark_pixel(x, y, color, dm);
	}
    }

    fn draw_horizontal_line(&mut self,
			    y: usize,
			    start_x: usize,
			    end_x: usize,
			    color: Color,
			    dm: &DrawMode) {
	for x in start_x..(end_x + 1) {
	    self.mark_pixel(x, y, color, dm);
	}
    }

    fn can_fill(&self, x: usize, y: usize, fill_over_colors: &[Color]) -> bool {
	x < STRANGER_END && y < HEIGHT
	    && x > STRANGER_START && y > 0
	    && fill_over_colors.contains(&self.pixels[y][x])
    }
    
    fn flood_fill(&mut self, x0: usize, y0: usize, color: Color,
		  fill_over_colors: &[Color], dm: &DrawMode) {
	let mut stack = vec![(x0, y0)];
	
	while stack.len() > 0 {
	    match stack.pop() {
		Some((x, y)) => {
		    //self.pixels[y][x] = color;
		    self.mark_pixel(x, y, color, dm);
		    if self.can_fill(x - 1, y, &fill_over_colors) {
			stack.push(( x - 1, y ));
		    }
		
		    if self.can_fill(x + 1, y, &fill_over_colors) {
			stack.push(( x + 1, y ));
		    }
		
		    if self.can_fill(x, y - 1, &fill_over_colors) {
			stack.push(( x, y - 1 ));
		    }
		
		    if self.can_fill(x, y + 1, &fill_over_colors) {
			stack.push(( x, y + 1 ));
		    }
		}
		None => {}	
	    }
	}
    }

    fn fill_rect(&mut self, top_left: (usize, usize),
		 bottom_right: (usize, usize), color: Color,
		 dm: &DrawMode) {
	for x in top_left.0..bottom_right.0 + 1 {
	    for y in top_left.1..bottom_right.1 + 1 {
		self.mark_pixel(x, y, color, dm);
	    }		
	}
    }

    fn draw_iris(&mut self, midpoint: (usize, usize), radius: usize, color: Color) {
	//let bg_color = Rc::new(self.pixels[midpoint.1][midpoint.1]);
	let bg_color = self.pixels[midpoint.1][midpoint.0];
	//fn colorBG(x: usize, y: usize, c: Color) {c == bg_color}
	for x in (midpoint.0 - radius)..(midpoint.0 + radius) {
	    for y in (midpoint.1 - radius)..(midpoint.1 + radius) {
		if (x - midpoint.0).pow(2) + (y - midpoint.1).pow(2) < radius.pow(2) {
		    self.mark_pixel(x, y, color, &(move |x, y, c| c == bg_color));
		    //self.mark_pixel(x, y, color, &(colorBG as DrawMode));
		}
	    }
	}
    }

    fn draw_perpendicular_line(&mut self, core_ptr: &Vec<(usize, usize)>,
			       x: usize, y: usize, line_color: Color, body_color: Color) {
	match perpendicular_line_coefficients(core_ptr, x, y) {
	    None => {
		self.draw_vertical_line(x, 0, HEIGHT - 1, line_color,
					&(move |x, y, c| c == body_color));
	    }
	    Some((m, b)) => {
		let b = y as f32 - (m * x as f32);
		//console_log!("slope: {}, m: {}, x: {}, b: {}", slope, m, x, b);
		let x0 = (0 as f32 - b)/m;
		let x1 = (HEIGHT as f32 - b)/m;
		//console_log!("x0: {}, x1: {}", x0, x1);
		for (x, y) in Bresenham::new((x0 as isize, 0 as isize),
					     (x1 as isize, HEIGHT as isize)) {
		    //console_log!("x: {}, y: {}", x, y);
		    if x > 0 && x < WIDTH as isize {
			self.mark_pixel(x as usize, y as usize, line_color,
					&(move |x, y, c| c == body_color));
		    }
		}
	    }
	}
    }     
}
