class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");
const solver = document.getElementById("solver");

const init_susceptible_pop = document.getElementById("init_susceptible_pop");
const init_infected_pop = document.getElementById("init_infected_pop");
const infection_coefficient = document.getElementById("infection_coefficient");
const recovery_coefficient = document.getElementById("recovery_coefficient");
const birth_rate = document.getElementById("birth_rate");
const vaccination_coefficient = document.getElementById("vaccination_coefficient");
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
	init_susceptible_pop.addEventListener("input", updatePlot);
	init_infected_pop.addEventListener("input", updatePlot);
    infection_coefficient.addEventListener("input", updatePlot);
	recovery_coefficient.addEventListener("input", updatePlot);
    birth_rate.addEventListener("input", updatePlot);
    vaccination_coefficient.addEventListener("input", updatePlot);
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
    var susceptible = Number(init_susceptible_pop.value);
    var infected = Number(init_infected_pop.value);
    if (susceptible + infected > 1) {
        infected = 1 - susceptible;
        init_infected_pop.value = String(infected);
    }
    var recovered = (1 - susceptible - infected);
    var params = Params.builder()
        .solver(String(solver.value))
        .max_time(Number(max_time.value))
        .initial_susceptible_population(Number(susceptible))
        .initial_infected_population(Number(infected))
        .initial_recovered_population(recovered)
        .infection_coefficient(Number(infection_coefficient.value))
        .recovery_coefficient(Number(recovery_coefficient.value))
        .birth_rate(Number(birth_rate.value))
        .vaccination_coefficient(Number(vaccination_coefficient.value));
    chart = Model.draw(canvas, params);
    canvas_text.innerHTML = `Max Time ($ t $): ${max_time.value}, ` +
        `Initial Susceptible Pop ($ S(0) $): ${susceptible.toFixed(2)}, ` + 
        `Initial Infected Pop ($ I(0) $): ${infected.toFixed(2)}, ` + 
        `Initial Recovered Pop ($ R(0) $): ${recovered.toFixed(2)}<br/>` + 
        `Infection Coefficient ($ \\beta $): ${infection_coefficient.value}, ` + 
        `Recovery Coefficient ($ \\gamma $): ${recovery_coefficient.value}, ` +
        `Birth Rate ($ \\mu $): ${birth_rate.value}, ` +
        `Vaccination Coefficient ($ p $): ${vaccination_coefficient.value}`;
    MathJax.typeset();
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
