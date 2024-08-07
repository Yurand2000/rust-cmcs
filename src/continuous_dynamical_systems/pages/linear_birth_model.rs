use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = CDS_LBM)]
pub struct Model { }

#[wasm_bindgen(js_name = CDS_LBM_Params)]
pub struct Params {
    max_time: f32,
    initial_population: f32,
    offsprings_per_individual: f32,
    reproduction_period: f32,
    max_population_display: f32,
}

#[wasm_bindgen(js_class = CDS_LBM)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
    }

    fn draw_function(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let x_axis_range = 0f32..params.max_time;

        let max_population_display =
            if params.max_population_display == 0f32 {
                params.predict_max_population_size() * 2f32
            } else {
                params.max_population_display
            } as u32;

        let y_axis_range = 0..max_population_display;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(params.max_time as usize)
            .y_labels(10)
            .draw()?;

        let birth_model = params.to_model();

        let simulation = Simulation::new(birth_model)
            .simulation_map(|(x, y)| (x, y as u32))
            .time_limit(chart.x_range().end);

        chart.draw_series(LineSeries::new(
            simulation,
            &RED
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = CDS_LBM_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { max_time: 1f32, initial_population: 1f32,
            offsprings_per_individual: 1f32, reproduction_period: 1f32, max_population_display: 0f32 }
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_population(mut self, initial_population: f32) -> Self {
        self.initial_population = initial_population;
        self
    }

    pub fn offsprings_per_individual(mut self, offsprings_per_individual: f32) -> Self {
        self.offsprings_per_individual = offsprings_per_individual;
        self
    }

    pub fn reproduction_period(mut self, reproduction_period: f32) -> Self {
        self.reproduction_period = reproduction_period;
        self
    }

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
        self
    }

    fn predict_max_population_size(&self) -> f32 {
        let rate = self.offsprings_per_individual / self.reproduction_period;
        self.initial_population * f32::exp(rate * self.max_time)
    }

    fn to_model(self) -> LinearBirthModel {
        let birth_rate = self.offsprings_per_individual / self.reproduction_period;

        LinearBirthModel::new(
            self.initial_population,
            birth_rate,
        )
    }
}