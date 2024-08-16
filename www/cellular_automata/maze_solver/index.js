class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const maze = document.getElementById("maze");
const step = document.getElementById("step");

let chart = null;
let model = null;

/** Main entry point */
export function main() {
    setupUI();
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmModel, WasmParams) {
    Model = WasmModel;
    Params = WasmParams;
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
    window.addEventListener("resize", setupCanvas);
    
	maze.addEventListener("input", updateModel);
	step.addEventListener("input", updatePlot);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
    const aspectRatio = canvas.width / canvas.height;
    var size = canvas.parentNode.offsetWidth * 0.8;
    if (size < 600)
        size = 600;
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size;
    canvas.height = size / aspectRatio;
    updateModel();
}

function updateModel() {
    step.value = 0;
    model = Model.build(
        Params.builder()
            .maze(maze.value)
    );

    updatePlot();
}

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = "Rendering...";
    const start = performance.now();

    chart = model.draw(canvas, step.value);
    canvas_text.innerHTML = `Current Step: ${step.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
