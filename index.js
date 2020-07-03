import wasmInit from "./pkg/stranger_gen.js";

const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/stranger_gen_bg.wasm");    
    var canvas = document.getElementById("strangerCanvas");    
    var nav_l =  document.getElementById("nav_l");
    var nav_r =  document.getElementById("nav_r");
    var height = rustWasm.height();
    canvas.height = height;
    var width = rustWasm.width();
    canvas.width = width;

    var ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;

    var pixelsPtr; var pixelArray;
    
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
    }

    generate();

    nav_l.addEventListener('click', function() {
	generate();
    }, false);

    nav_r.addEventListener('click', function() {
	generate();
    }, false);
}    


runWasm();
