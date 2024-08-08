class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");
const solver = document.getElementById("solver");

const init_prey_pop = document.getElementById("init_prey_pop");
const init_predator_pop = document.getElementById("init_predator_pop");
const prey_birth_rate = document.getElementById("prey_birth_rate");
const predator_death_rate = document.getElementById("predator_death_rate");
const hunting_meetings = document.getElementById("hunting_meetings");
const hunt_offsprings = document.getElementById("hunt_offsprings");
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
	init_prey_pop.addEventListener("input", updatePlot);
	init_predator_pop.addEventListener("input", updatePlot);
	prey_birth_rate.addEventListener("input", updatePlot);
    predator_death_rate.addEventListener("input", updatePlot);
	hunting_meetings.addEventListener("input", updatePlot);
	hunt_offsprings.addEventListener("input", updatePlot);
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
        .initial_prey_population(Number(init_prey_pop.value))
        .initial_predator_population(Number(init_predator_pop.value))
        .prey_birth_rate(Number(prey_birth_rate.value))
        .predator_death_rate(Number(predator_death_rate.value))
        .hunting_meetings(Number(hunting_meetings.value))
        .hunt_offsprings(Number(hunt_offsprings.value));
    chart = Model.draw(canvas, chosen_solver, params);
    canvas_text.innerHTML = `Max Time (t): ${max_time.value}, ` +
        `Initial Prey Pop (F(0)): ${init_prey_pop.value}, ` + 
        `Initial Predator Pop (M(0)): ${init_predator_pop.value}, ` + 
        `Prey Birth Rate (r): ${prey_birth_rate.value}, ` + 
        `Predator Death Rate (s): ${predator_death_rate.value}, ` + 
        `Hunting Meetings (a): ${hunting_meetings.value}, ` +
        `Hunt Offsprings (b): ${hunt_offsprings.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
