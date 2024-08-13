class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const init_lessonae = document.getElementById("init_lessonae");
const init_hybrid = document.getElementById("init_hybrid");
const init_ridibundus = document.getElementById("init_ridibundus");
const carrying_capacity = document.getElementById("carrying_capacity");
const selection_strength = document.getElementById("selection_strength");
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
    
	init_lessonae.addEventListener("input", updatePlot);
	init_hybrid.addEventListener("input", updatePlot);
	init_ridibundus.addEventListener("input", updatePlot);
	carrying_capacity.addEventListener("input", updatePlot);
	selection_strength.addEventListener("input", updatePlot);
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
        .initial_lessonae_pop(Number(init_lessonae.value))
        .initial_hybrid_pop(Number(init_hybrid.value))
        .initial_ridibundus_pop(Number(init_ridibundus.value))
        .carrying_capacity(Number(carrying_capacity.value))
        .selection_strength(Number(selection_strength.value))
        .simulation_seed(seed.value);
    chart = Model.draw(canvas, params);
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Init Lessonae: ${init_lessonae.value}, ` + 
        `Init Hybrids: ${init_hybrid.value}, ` + 
        `Init Ridibundus: ${init_ridibundus.value}, ` + 
        `Carrying Capacity: ${carrying_capacity.value}, ` + 
        `Selection Strength: ${selection_strength.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
