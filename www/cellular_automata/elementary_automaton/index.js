class Model {}
class Params {}

const image = document.getElementById("image");
const canvas = document.createElement("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const resolution = document.getElementById("resolution");
const boundary_condition = document.getElementById("boundary_condition");
const rule_field = document.getElementById("rule");
const start_state = document.getElementById("start_state");
const max_time = document.getElementById("max_time");
const seed = document.getElementById("seed");

let chart = null;

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
    
	resolution.addEventListener("input", updatePlot);
	boundary_condition.addEventListener("input", updatePlot);
	rule_field.addEventListener("input", updatePlot);
    start_state.addEventListener("input", updatePlot);
	max_time.addEventListener("input", updatePlot);
	seed.addEventListener("input", updatePlot);
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

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = "Rendering...";
    const start = performance.now();

    var rule = Math.max(0, Math.min(255, Number(rule_field.value)));
    var params = Params.builder()
        .max_time(Number(max_time.value))
        .resolution(Number(resolution.value))
        .boundary(boundary_condition.value)
        .initial_state(start_state.value)
        .seed(seed.value)
        .rule(rule);
    chart = Model.draw(canvas, params);
    image.src = canvas.toDataURL("image/png");
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Grid Size: ${resolution.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
