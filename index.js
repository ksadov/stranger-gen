import wasmInit, { Metadata } from "./pkg/stranger_gen.js";
//import Metadata from "./pkg/stranger_gen.js";

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/stranger_gen_bg.wasm");    
    var canvas = document.getElementById("strangerCanvas");    
    var nav_l =  document.getElementById("nav_l");
    var nav_r =  document.getElementById("nav_r");

    var class_f = document.getElementById("class_f");
    var disposition_f = document.getElementById("disposition_f");
    var height_f = document.getElementById("height_f");
    var length_f = document.getElementById("length_f");
    var weight_f = document.getElementById("weight_f");
    var size_variance_f = document.getElementById("size_variance_f");
    var iq_f = document.getElementById("iq_f");
    var core_temp_f = document.getElementById("core_temp_f");
    var stability_f = document.getElementById("stability_f");
    var prevalence_f = document.getElementById("prevalence_f");
    var constancy_f = document.getElementById("constancy_f");
    var longevity_f = document.getElementById("longevity_f");
    var no_appearing_f = document.getElementById("no_appearing_f");
    var vision_f = document.getElementById("vision_f");
    var language_family_f = document.getElementById("language_family_f");

    var switcher = document.getElementById("switcher");
    
    var height = rustWasm.height();
    canvas.height = height;
    var width = rustWasm.width();
    canvas.width = width;

    var ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;

    var metric = true;

    var m;
    
    var pixelsPtr; var pixelArray;

    function c_to_f(c) {
	return (round_1((c * 9/5) + 32));
    }

    function kg_to_lb(kg) {
	return (round_1(2.205 * kg));
    }

    function m_to_ft(m) {
	return (round_1(3.281 * m));
    }
    
    function assign_measurements(meta) {
	if (metric) {
	    height_f.innerHTML = round_1(meta.height) + " m";
	    length_f.innerHTML = round_1(meta.length) + " m";
	    weight_f.innerHTML = round_1(meta.weight) + " kg";
	    core_temp_f.innerHTML = round_1(meta.core_temp) + "°C";
	}

	else {
	    height_f.innerHTML = m_to_ft(meta.height) + " ft";
	    length_f.innerHTML = m_to_ft(meta.length) + " ft";
	    weight_f.innerHTML = kg_to_lb(meta.weight) + " lb";
	    core_temp_f.innerHTML = c_to_f(meta.core_temp) + "°F";
	}
    }

    function round_1(n) {
	return (Math.round(n * 10) / 10);
    }
    
    function assign_metadata(meta) {
	class_f.innerHTML = meta.class_t;
	disposition_f.innerHTML = meta.disposition;
	size_variance_f.innerHTML = meta.size_variance + "%";
	iq_f.innerHTML = meta.iq;
	stability_f.innerHTML = meta.stability + "%";
	prevalence_f.innerHTML = meta.prevalence + "%";
	constancy_f.innerHTML = meta.constancy + "%";
	longevity_f.innerHTML = round_1(meta.longevity) + " years";
	no_appearing_f.innerHTML = meta.no_appearing;
	vision_f.innerHTML = meta.vision;
	language_family_f.innerHTML = meta.language_family;
	assign_measurements(meta);
    }
    
    function generate() {
	var unrendered = true;
	while (unrendered) {
	    try {
		pixelsPtr = rustWasm.render_stranger();
		unrendered = false;
	    }
	    catch (err) {}
	    pixelArray = new Uint8ClampedArray(rustWasm.memory.buffer,
					       pixelsPtr, 4 * width * height);
	    ctx.putImageData(new ImageData(pixelArray, width, height), 0, 0);
	}
	m = new Metadata();
	assign_metadata(m);
    }

    generate();

    nav_l.addEventListener('click', function() {
	generate();
    }, false);

    nav_r.addEventListener('click', function() {
	generate();
    }, false);

    switcher.addEventListener('click', function() {
	metric = !metric;
	if (metric) { switcher.innerHTML = "imperial"; }
	else { switcher.innerHTML = "metric"; }
	assign_metadata(m);
    }, false);
}    


runWasm();
