class Model {}
class Params {}

const button_play = `<svg class="w-6 h-6 text-gray-800" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
  <path fill-rule="evenodd" d="M8.6 5.2A1 1 0 0 0 7 6v12a1 1 0 0 0 1.6.8l8-6a1 1 0 0 0 0-1.6l-8-6Z" clip-rule="evenodd"/>
</svg>`;
const button_pause = `<svg class="w-6 h-6 text-gray-800" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
  <path fill-rule="evenodd" d="M8 5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H8Zm7 0a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-1Z" clip-rule="evenodd"/>
</svg>`;

const canvas = document.createElement("canvas");
const image = document.getElementById("image");
const status = document.getElementById("status");
const canvas_text = document.getElementById("canvas_text");

const maze = document.getElementById("maze");
const step = document.getElementById("step");

const rewind = document.getElementById("rewind");
const play_pause = document.getElementById("play_pause");

let chart = null;
let model = null;
var playing = false;
var anim_speed = 75;

/** Main entry point */
export function main() {
    setupUI();
    updateModel();
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
    
	maze.addEventListener("input", updateModelAndDraw);
	step.addEventListener("input", updateStep);

    rewind.addEventListener("click", rewindFn);
    play_pause.addEventListener("click", playPauseFn);
}

function rewindFn() {
    step.value = 0;
    updatePlayPause(false);
    updatePlot();
}

function playPauseFn() {
    updatePlayPause(!playing);
}

function updatePlayPauseRender() {
    if (playing) {
        play_pause.innerHTML = button_pause;
        playAnimation();
    } else {
        play_pause.innerHTML = button_play;
    }
}

function playAnimation() {
    if (!playing) {
        return;
    }

    step.value = Number(step.value) + 1;
    updatePlot();

    if (Number(step.value) < Number(step.max)) {
        setTimeout(playAnimation, anim_speed);
    } else {
        updatePlayPause(false);
    }
}

function updatePlayPause(value) {
    playing = value;
    updatePlayPauseRender();
}

function updateStep() {
    updatePlayPause(false);
    updatePlot();
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

function updateModel() {
    step.value = 0;
    model = Model.build(
        Params.builder()
            .maze(maze.value)
    );

    step.max = model.max_step();
}

function updateModelAndDraw() {
    updatePlayPause(false);
    updateModel();
    updatePlot();
}

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = "Rendering...";
    const start = performance.now();

    chart = model.draw(canvas, step.value);
    image.src = canvas.toDataURL("image/png");
    canvas_text.innerHTML = `Current Step: ${step.value}`;
    const end = performance.now();
    status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
}
