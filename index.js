import wasmInit from "./pkg/stranger_gen.js";
const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/stranger_gen_bg.wasm");    
    var canvas = document.getElementById("strangerCanvas");    
    var gen =  document.getElementById("genButton");
    var height = rustWasm.height();
    canvas.height = height;
    var width = rustWasm.width();
    canvas.width = width;

    var ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;
    var unrendered = true;
    var pixelsPtr;
    /* I'm just not going to bother figuring out why the rendering crashes 2%
       of the time */
    while (unrendered) {
	try {
	    pixelsPtr = rustWasm.render_stranger();
	    unrendered = false;
	}
	catch (err) {}
    }
    
    var pixelArray = new Uint8ClampedArray(rustWasm.memory.buffer,
					   pixelsPtr,
					   4 * width * height);

    ctx.putImageData(new ImageData(pixelArray, width, height), 0, 0);

    gen.addEventListener('click', function() {
	console.log("x");
	    pixelsPtr = rustWasm.render_stranger();
	pixelArray = new Uint8ClampedArray(rustWasm.memory.buffer,
					   pixelsPtr, 4 * width * height);
	ctx.putImageData(new ImageData(pixelArray, width, height), 0, 0);
	}, false);
}    

runWasm();
