class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const init_pop = document.getElementById("init_pop");
const offsprings = document.getElementById("offsprings");
const repr_rate = document.getElementById("repr_rate");
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
	init_pop.addEventListener("input", updatePlot);
	offsprings.addEventListener("input", updatePlot);
	repr_rate.addEventListener("input", updatePlot);
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
        .initial_population(Number(init_pop.value))
        .offsprings_per_individual(Number(offsprings.value))
        .reproduction_period(Number(repr_rate.value));
    chart = Model.draw(canvas, params);
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Initial Pop (N(0)): ${init_pop.value}, ` + 
        `Offsprings (λ): ${offsprings.value}, ` + 
        `Reproduction Period (σ): ${repr_rate.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
