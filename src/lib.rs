use wasm_bindgen::prelude::*;
use cubic_spline::{Spline, SplineOpts};
use bresenham::Bresenham;
use rand::Rng;
use std::cmp::min;

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

const MIN_EYE_WIDTH: usize = 0;

const MIN_LEG_WIDTH: usize = 3;
const MAX_LEG_WIDTH: usize = 8;
    
const EYE_LEG_DISTANCE: usize = 30;
const NEAR_FAR_LEG_DISTANCE_RATIO: usize = 2;
const MIN_FORE_HIND_LEG_DISTANCE: usize = 100;
const MAX_FORE_HIND_LEG_DISTANCE: usize = 180;

const FAR_FOOT_HEIGHT: usize = HEIGHT - 30;
const NEAR_FOOT_HEIGHT: usize = HEIGHT - 25;

const MAX_IRIS_RADIUS: usize = EYE_END_MIN - EYE_START_MAX - 2;
const MIN_IRIS_RADIUS: usize = 4;

type Color = (u8, u8, u8, u8);

const BLACK: Color = (0, 0, 0, 255);

#[derive(Copy, Clone)]
enum DrawMode {
    LayerOver,
    LayerUnder,
    PreserveAlpha
}

struct Canvas {
    pixels: [[Color; WIDTH]; HEIGHT]
}

struct StrangerParams {
    core_anchors: [(usize, usize); 4],
    dorsal_anchors: [(usize, usize); 4],
    ventral_anchors: [(usize, usize); 4],
    color: Color,
    eye_start_x: usize,
    eye_end_x: usize,
    iris_radius: usize,
    leg_width: usize,
    fore_hind_dist: usize
/*
    cap: Option<usize>,
    socks: Option<usize>,
    stripes: Option<usize>,
    tears: Option<usize>
*/
}

#[wasm_bindgen]  
pub fn width() -> usize { WIDTH }

#[wasm_bindgen]  
pub fn height() -> usize { HEIGHT }

#[wasm_bindgen]  
pub fn render_stranger() ->  *const u8 {
    let mut canvas = Canvas::new();
    let sp = StrangerParams::new();
    
    let test_fill = sp.color;
    
    let core = make_spline(sp.core_anchors);
    let dorsal = make_spline(sp.dorsal_anchors);
    let ventral = make_spline(sp.ventral_anchors);
    &mut canvas.draw_spline(&dorsal);
    &mut canvas.draw_spline(&ventral);
    
    let eye_mid_x = (sp.eye_start_x + sp.eye_end_x) / 2;
    let max_eye_width = min
			 (y_at_x(&dorsal, eye_mid_x) - y_at_x(&core, eye_mid_x),
			  y_at_x(&core, eye_mid_x) - y_at_x(&ventral, eye_mid_x))
	* 3 / 4;
    let (top_eye_anchors, bottom_eye_anchors) =
			  generate_eye_anchors((sp.eye_start_x, y_at_x(&core, sp.eye_start_x)),
						(sp.eye_end_x, y_at_x(&core, sp.eye_end_x)),
								   max_eye_width);
    let top_eye = make_spline(top_eye_anchors);
    let bottom_eye = make_spline(bottom_eye_anchors);
   
    &mut canvas.draw_spline(&top_eye);
    &mut canvas.draw_spline(&bottom_eye);
    
    &mut canvas.flood_fill(sp.core_anchors[2].0, sp.core_anchors[2].1, test_fill);

    let leg_start_x = sp.eye_end_x + EYE_LEG_DISTANCE;
    &mut canvas.draw_leg(leg_start_x, &dorsal, FAR_FOOT_HEIGHT, test_fill,
			 sp.leg_width, DrawMode::LayerUnder);

    
    let leg2_start_x = leg_start_x + NEAR_FAR_LEG_DISTANCE_RATIO * sp.leg_width;
    &mut canvas.draw_leg(leg2_start_x, &dorsal, NEAR_FOOT_HEIGHT, test_fill,
			 sp.leg_width, DrawMode::LayerOver);

    let leg3_start_x = leg_start_x + sp.fore_hind_dist;
    &mut canvas.draw_leg(leg3_start_x, &dorsal, FAR_FOOT_HEIGHT, test_fill,
			 sp.leg_width, DrawMode::LayerUnder);
    
    let leg4_start_x = leg3_start_x + NEAR_FAR_LEG_DISTANCE_RATIO * sp.leg_width;
    &mut canvas.draw_leg(leg4_start_x, &dorsal, NEAR_FOOT_HEIGHT, test_fill,
			 sp.leg_width, DrawMode::LayerOver);

    let eye_mid_y = y_at_x(&core, eye_mid_x);
    &mut canvas.draw_iris((eye_mid_x, eye_mid_y), sp.iris_radius, BLACK);
    
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

fn y_at_x (spline: &Vec<(usize, usize)>, x0: usize) -> usize {
    let mut result = 0;
    for (x, y) in spline.iter() {
	if *x == x0 { result = *y }
    }
    result
}

impl StrangerParams {
    fn new() -> StrangerParams {
	let mut rng = rand::thread_rng();

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

	let color = (rng.gen_range(0, 255), rng.gen_range(0, 255),
		     rng.gen_range(0, 255), 255);

	let eye_start_x = rng.gen_range(EYE_START_MIN, EYE_START_MAX);
	let eye_end_x = rng.gen_range(EYE_END_MIN, EYE_END_MAX);

	let iris_radius = rng.gen_range(MIN_IRIS_RADIUS, MAX_IRIS_RADIUS);

	let leg_width = rng.gen_range(MIN_LEG_WIDTH, MAX_LEG_WIDTH);

	let fore_hind_dist = rng.gen_range(MIN_FORE_HIND_LEG_DISTANCE,
					   MAX_FORE_HIND_LEG_DISTANCE);
	StrangerParams {
	    core_anchors: core_anchors,	    
	    dorsal_anchors: dorsal_anchors,
	    ventral_anchors: ventral_anchors,
	    color: color,
	    eye_start_x: eye_start_x,
	    eye_end_x: eye_end_x,
	    iris_radius: iris_radius,
	    leg_width: leg_width,
	    fore_hind_dist: fore_hind_dist
	 }	
    }
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

impl Canvas {
    			      
    fn new() -> Canvas {
	//utils::set_panic_hook();
	Canvas {
	    pixels: [[(100, 100, 100, 100); WIDTH]; HEIGHT]
	}
    }
    
    fn raw_pixels(&self) -> *const u8 {
	&self.pixels[0][0].0 as *const u8
    }

    fn mark_pixel(&mut self, x: usize, y: usize, color: Color, dm: DrawMode) {
	match dm {
	    DrawMode::LayerOver => { self.pixels[y][x] = color; }
	    DrawMode::LayerUnder => {
		if self.pixels[y][x].3 < 255 {
		    self.pixels[y][x] = color; }
	    }
	    DrawMode::PreserveAlpha => {
		if self.pixels[y][x].3 == 255 {
		    self.pixels[y][x] = color;
		}
	    }
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
		dm: DrawMode) {
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
			  color: Color, dm: DrawMode) {
	for y in start_y..(end_y + 1) {
	    self.mark_pixel(x, y, color, dm);
	}
    }

    fn draw_horizontal_line(&mut self,
			    y: usize,
			    start_x: usize,
			    end_x: usize,
			    color: Color,
			    dm: DrawMode) {
	for x in start_x..(end_x + 1) {
	    self.mark_pixel(x, y, color, dm);
	}
    }

    fn can_fill(&self, x: usize, y: usize, start_color: Color) -> bool {
	x < STRANGER_END && y < HEIGHT
	    && x > STRANGER_START && y > 0
	    && self.pixels[y][x] == start_color
	    && self.pixels[y][x] != BLACK
    }
    
    fn flood_fill(&mut self, x0: usize, y0: usize, color: Color) {
	let mut stack = vec![(x0, y0)];
	let start_color = self.pixels[y0][x0];
	
	while stack.len() > 0 {
	    match stack.pop() {
		Some((x, y)) => {
		    self.pixels[y][x] = color;
		    if self.can_fill(x - 1, y, start_color) {
			stack.push(( x - 1, y ));
		    }
		
		    if self.can_fill(x + 1, y, start_color) {
			stack.push(( x + 1, y ));
		    }
		
		    if self.can_fill(x, y - 1, start_color) {
			stack.push(( x, y - 1 ));
		    }
		
		    if self.can_fill(x, y + 1, start_color) {
			stack.push(( x, y + 1 ));
		    }
		}
		None => {}	
	    }
	}
    }

    fn fill_rect(&mut self, top_left: (usize, usize),
		 bottom_right: (usize, usize), color: Color,
		 dm: DrawMode) {
	for x in top_left.0..bottom_right.0 + 1 {
	    for y in top_left.1..bottom_right.1 + 1 {
		self.mark_pixel(x, y, color, dm);
	    }		
	}
    }

    fn draw_iris(&mut self, midpoint: (usize, usize), radius: usize, color: Color) {
	for x in (midpoint.0 - radius)..(midpoint.0 + radius) {
	    for y in (midpoint.1 - radius)..(midpoint.1 + radius) {
		if (x - midpoint.0).pow(2) + (y - midpoint.1).pow(2) < radius.pow(2) {
		    self.mark_pixel(x, y, color, DrawMode::LayerUnder);
		}
	    }
	}
    }
    
}

    
