class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");
const solver = document.getElementById("solver");

const init_g1_pop = document.getElementById("init_g1_pop");
const init_g2_pop = document.getElementById("init_g2_pop");
const init_g3_pop = document.getElementById("init_g3_pop");
const seed = document.getElementById("seed");
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
	solver.addEventListener("input", updatePlot);
	seed.addEventListener("input", updatePlot);
	init_g1_pop.addEventListener("input", updatePlot);
	init_g2_pop.addEventListener("input", updatePlot);
	init_g3_pop.addEventListener("input", updatePlot);
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
    
    var params = Params.builder();
    var chosen_solver = "";
    if (solver.value == "ssa") {
        chosen_solver = "ssa";
        params = params.ssa_seed(seed.value);
    }
    else {
        chosen_solver = "ode";
        params = params.solver(solver.value);
    }

    params = params
        .max_time(Number(max_time.value))
        .initial_state(Number(init_g1_pop.value), Number(init_g2_pop.value), Number(init_g3_pop.value))
        .production_rates(10, 10000, 10)
        .binding_rates(10, 0.1, 10)
        .unbinding_rates(2, 20, 20)
        .decay_rates(1, 100, 1);
    chart = Model.draw(canvas, chosen_solver, params);
    canvas_text.innerHTML = `Max Time ($ t $): ${max_time.value}, ` +
        `Initial $ \\ce{g1} $: ${init_g1_pop.value}, ` + 
        `Initial $ \\ce{g2} $: ${init_g2_pop.value}, ` + 
        `Initial $ \\ce{g3} $: ${init_g3_pop.value}`;
    MathJax.typeset();
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
