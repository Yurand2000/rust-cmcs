class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");
const solver = document.getElementById("solver");

const init_enzyme = document.getElementById("init_enzyme");
const init_reactant = document.getElementById("init_reactant");
const seed = document.getElementById("seed");
const binding_coeff = document.getElementById("binding_coeff");
const unbinding_coeff = document.getElementById("unbinding_coeff");
const catalysis_coeff = document.getElementById("catalysis_coeff");
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
    solver.addEventListener("change", updatePlot);
    
	init_enzyme.addEventListener("input", updatePlot);
	init_reactant.addEventListener("input", updatePlot);
	seed.addEventListener("input", updatePlot);
	binding_coeff.addEventListener("input", updatePlot);
	unbinding_coeff.addEventListener("input", updatePlot);
	catalysis_coeff.addEventListener("input", updatePlot);
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

    var params = params
        .max_time(Number(max_time.value))
        .initial_enzyme(Number(init_enzyme.value))
        .initial_reactant(Number(init_reactant.value))
        .binding_rate(Number(binding_coeff.value))
        .unbinding_rate(Number(unbinding_coeff.value))
        .catalysis_rate(Number(catalysis_coeff.value));
    chart = Model.draw(canvas, chosen_solver, params);
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Initial Enzyme (E(0)): ${init_enzyme.value}, ` + 
        `Initial Reactant (S(0)): ${init_reactant.value}, ` + 
        `Binding Coefficient (b): ${binding_coeff.value}, ` + 
        `Unbinding Coefficient (ub): ${unbinding_coeff.value}, ` + 
        `Catalysis Coefficient (c): ${catalysis_coeff.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
