class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const lambda_param = document.getElementById("lambda_param");
const mean_param = document.getElementById("mean_param");
const std_dev_param = document.getElementById("std_dev_param");
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
    
	lambda_param.addEventListener("input", updatePlot);
	mean_param.addEventListener("input", updatePlot);
	std_dev_param.addEventListener("input", updatePlot);
	max_time.addEventListener("input", updatePlot);
	seed.addEventListener("input", updatePlot);
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
    updatePlot();
}

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = "Rendering...";
    const start = performance.now();

    var params = Params.builder()
        .max_time(Number(max_time.value))
        .customer_arrival_lambda(Number(lambda_param.value))
        .customer_served_mean(Number(mean_param.value))
        .customer_served_std_dev(Number(std_dev_param.value))
        .simulation_seed(seed.value);
    chart = Model.draw(canvas, params);
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Arrival Rate: ${lambda_param.value}, ` + 
        `Service Mean Time: ${mean_param.value}, ` + 
        `Service Time StdDev: ${std_dev_param.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
