class Model {}
class Params {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const init_female_pop = document.getElementById("init_female_pop");
const init_male_pop = document.getElementById("init_male_pop");
const birth_rate = document.getElementById("birth_rate");
const male_death_rate = document.getElementById("male_death_rate");
const carrying_cap = document.getElementById("carrying_cap");
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
	init_female_pop.addEventListener("input", updatePlot);
	init_male_pop.addEventListener("input", updatePlot);
	birth_rate.addEventListener("input", updatePlot);
    male_death_rate.addEventListener("input", updatePlot);
	carrying_cap.addEventListener("input", updatePlot);
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
        .initial_female_population(Number(init_female_pop.value))
        .initial_male_population(Number(init_male_pop.value))
        .birth_rate(Number(birth_rate.value))
        .male_death_rate(Number(male_death_rate.value))
        .carrying_capacity(Number(carrying_cap.value));
    chart = Model.draw(canvas, params);
    canvas_text.innerHTML = `Max Time ($ t $): ${max_time.value}, ` +
        `Initial Female Pop ($ F(0) $): ${init_female_pop.value}, ` + 
        `Initial Male Pop ($ M(0) $): ${init_male_pop.value},<br/>` + 
        `Birth Rate ($ r $): ${birth_rate.value}, ` + 
        `Male Death Rate ($ s $): ${male_death_rate.value}, ` + 
        `Carrying Capacity ($ K $): ${carrying_cap.value}`;
    MathJax.typeset();
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;	
}
