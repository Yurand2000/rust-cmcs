class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");
const plot_type = document.getElementById("plot_type");

const init_pop = document.getElementById("init_pop");
const offsprings = document.getElementById("offsprings");
const repr_rate = document.getElementById("repr_rate");
const step_size = document.getElementById("step_size");
const max_time = document.getElementById("max_time");

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
    plot_type.addEventListener("change", updatePlot);
	init_pop.addEventListener("input", updatePlot);
	offsprings.addEventListener("input", updatePlot);
	repr_rate.addEventListener("input", updatePlot);
	step_size.addEventListener("input", updatePlot);
	max_time.addEventListener("input", updatePlot);
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
        .time_step(Number(step_size.value))
        .initial_population(Number(init_pop.value))
        .offsprings_per_individual(Number(offsprings.value))
        .reproduction_period(Number(repr_rate.value));
    chart = Model.draw(canvas, plot_type.value, params);
    canvas_text.innerHTML = `Max Time ($ t $): ${max_time.value}, ` +
        `Time Step ($ \\Delta t $): ${step_size.value}, ` + 
        `Initial Pop ($ N(0) $): ${init_pop.value}<br/>` + 
        `Offsprings ($ \\lambda $): ${offsprings.value}, ` + 
        `Reproduction Period ($ \\sigma $): ${repr_rate.value}`;
    MathJax.typeset();
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
