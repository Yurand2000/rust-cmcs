class Model {}
class Params {}

const canvas = document.createElement("canvas");
const image = document.getElementById("image");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const maze = document.getElementById("maze");
const step = document.getElementById("step");

let chart = null;
let model = null;

/** Main entry point */
export function main() {
    setupUI();
    updateModel();
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
    
	maze.addEventListener("input", updateModelAndDraw);
	step.addEventListener("input", updatePlot);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
    const aspectRatio = image.width / image.height;
    var size = image.parentNode.offsetWidth * 0.8;
    if (size < 400)
        size = 400;
    image.style.width = size + "px";
    image.style.height = size / aspectRatio + "px";
    image.width = size;
    image.height = size / aspectRatio;
    updatePlot();
}

function updateModel() {
    step.value = 0;
    model = Model.build(
        Params.builder()
            .maze(maze.value)
    );

    step.max = model.max_step();
}

function updateModelAndDraw() {
    updateModel();
    updatePlot();
}

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = "Rendering...";
    const start = performance.now();

    chart = model.draw(canvas, step.value);
    image.src = canvas.toDataURL("image/png");
    canvas_text.innerHTML = `Current Step: ${step.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;

}
