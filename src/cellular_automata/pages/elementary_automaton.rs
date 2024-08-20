use plotters::prelude::*;
use rand::{Rng, SeedableRng};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::prelude::*;
use crate::cellular_automata::prelude::{*, elementary::*};

#[wasm_bindgen(js_name = CA_ELEM)]
pub struct Model { }

#[wasm_bindgen(js_name = CA_ELEM_Params)]
#[derive(Default)]
pub struct Params {
    max_time: u32,
    resolution: u32,
    boundary: BoundaryCondition,
    rule: u8,
    initial_state: StartingState,
    seed: u64,
}

#[wasm_bindgen(js_class = CA_ELEM)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        // run the simulation
        let max_time = u32::min(params.max_time, params.resolution);
        let resolution = params.resolution;
        let model = params.to_model();
        let simulation = Simulation::new(model)
            .time_limit(max_time);

        // create the image
        let mut image = image::RgbaImage::new(resolution, resolution);

        image.fill(255);
        for ((_, lattice), row) in simulation.zip(image.rows_mut()) {
            for (active, cell) in lattice.into_iter().zip(row) {
                let color = if active { YELLOW } else { BLUE };                
                cell.0 = [color.0, color.1, color.2, 255];
            }
        }

        // fill the canvas
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&image),
            resolution, 
            resolution
        )?;

        canvas.set_width(resolution);
        canvas.set_height(resolution);

        let context: CanvasRenderingContext2d = canvas.get_context("2d")?.ok_or(format!(""))?.dyn_into()?;
        context.put_image_data(&image, 0f64, 0f64)?;

        Ok(())
    }
}

#[wasm_bindgen(js_class = CA_ELEM_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn max_time(mut self, max_time: u32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn boundary(mut self, boundary: String) -> Self {
        self.boundary = BoundaryCondition::from_str(&boundary).unwrap();
        self
    }

    pub fn rule(mut self, rule: u8) -> Self {
        self.rule = rule;
        self
    }

    pub fn initial_state(mut self, initial_state: String) -> Self {
        self.initial_state = StartingState::from_str(&initial_state).unwrap();
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn to_model(self) -> ElementaryAutomaton {
        let initial_state =
            match self.initial_state {
                StartingState::SingleCell => {
                    let mut state = Lattice::empty(self.resolution as usize);
                    state.set(self.resolution as usize / 2, true);

                    state
                },
                StartingState::Random => {
                    let mut state = Lattice::empty(self.resolution as usize);
                    let distribution = rand::distributions::Bernoulli::new(0.5).unwrap();
                    let mut rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
                    for idx in 0..(self.resolution as usize) {
                        state.set(idx, rng.sample(distribution));
                    }

                    state
                },
                StartingState::Full => {
                    Lattice::full(self.resolution as usize)
                },
                StartingState::Empty => {
                    Lattice::empty(self.resolution as usize)
                },
            };

        ElementaryAutomaton::new(
            initial_state,
            self.boundary,
            self.rule
        )
    }
}