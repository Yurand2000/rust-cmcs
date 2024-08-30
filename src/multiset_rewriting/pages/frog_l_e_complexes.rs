use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::multiset_rewriting::prelude::*;

#[wasm_bindgen(js_name = MSR_FLE)]
pub struct Model { }

#[wasm_bindgen(js_name = MSR_FLE_Params)]
#[derive(Default)]
pub struct Params {
    max_time: f32,
    initial_lessonae_pop: u32,
    initial_hybrid_pop: u32,
    initial_ridibundus_pop: u32,
    carrying_capacity: u32,
    selection_strength: f32,
    simulation_seed: u64,
}

#[wasm_bindgen(js_class = MSR_FLE)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
    }

    fn draw_function(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let max_time = params.max_time;
        let x_axis_range = 0f32..params.max_time;
        let y_axis_range = 0..(params.carrying_capacity as f32 * 2f32 / 3f32) as u32;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(40u32)
            .y_label_area_size(60u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(10)
            .y_labels(10)
            .draw()?;

        let model = params.to_model();
        let simulation = Simulation::new(model)
            .time_limit(max_time)
            .cache();

        let [ll, lyl, lr, lyr, lrd, lyrd, rr, ryr, rdrd, rydrd, rdr, rydr, ryrd] = FrogLEComplexes::adults_objects();

        // lessonae population
        chart.draw_series(LineSeries::new(
            simulation.clone()
                .map(|(time, state)| (time, state.get(&ll) + state.get(&lyl))),
            &GREEN
        ))?
        .label("Lessonae")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));

        // hybrids population
        chart.draw_series(LineSeries::new(
            simulation.clone()
                .map(|(time, state)| (time, state.get(&lr) + state.get(&lyr) + state.get(&lrd) + state.get(&lyrd))),
            &BLUE
        ))?
        .label("Esculentus (Hybrids)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // ridibundus population
        chart.draw_series(LineSeries::new(
            simulation.clone()
                .map(|(time, state)| (time,
                    state.get(&rr) + state.get(&ryr) + state.get(&rdrd) + state.get(&rydrd)
                    + state.get(&rdr) + state.get(&rydr) + state.get(&ryrd))
                ),
            &RED
        ))?
        .label("Ridibundus")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = MSR_FLE_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_lessonae_pop(mut self, initial_lessonae_pop: u32) -> Self {
        self.initial_lessonae_pop = initial_lessonae_pop;
        self
    }

    pub fn initial_hybrid_pop(mut self, initial_hybrid_pop: u32) -> Self {
        self.initial_hybrid_pop = initial_hybrid_pop;
        self
    }

    pub fn initial_ridibundus_pop(mut self, initial_ridibundus_pop: u32) -> Self {
        self.initial_ridibundus_pop = initial_ridibundus_pop;
        self
    }
    
    pub fn carrying_capacity(mut self, carrying_capacity: u32) -> Self {
        self.carrying_capacity = carrying_capacity;
        self
    }
    
    pub fn selection_strength(mut self, selection_strength: f32) -> Self {
        self.selection_strength = selection_strength;
        self
    }
    
    pub fn simulation_seed(mut self, simulation_seed: u64) -> Self {
        self.simulation_seed = simulation_seed;
        self
    }

    fn to_model(self) -> MinimalProbabilisticPSystem {
        FrogLEComplexes::build_model(
            (self.initial_lessonae_pop, self.initial_hybrid_pop, self.initial_ridibundus_pop),
            self.selection_strength,
            self.carrying_capacity,
            self.simulation_seed,
        )
    }
}