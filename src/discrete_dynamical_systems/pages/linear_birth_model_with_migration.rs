use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::discrete_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = DDS_LBMM)]
pub struct Model { }

#[wasm_bindgen(js_name = DDS_LBMM_Params)]
pub struct Params {
    max_time: f32,
    initial_population: f32,
    birth_death_rate: f32,
    migration_coefficient: f32,
    max_population_display: f32,
}

#[wasm_bindgen(js_class = DDS_LBMM)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, typ: String, params: Params) -> Result<(), JsValue> {
        match GraphType::from_string(typ) {
            Some(GraphType::Function) => 
                draw_generic(Self::draw_function)(canvas, params),
            Some(GraphType::PhaseGraph) =>
                draw_generic(Self::draw_phase_graph)(canvas, params),
            None =>
                Err(format!("Graph type not supported").into())
        }
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

        let model = params.to_model();

        let simulation = Simulation::new(model)
            .map(|(x, y)| (x, y as u32))
            .time_limit(chart.x_range().end);

        chart.draw_series(LineSeries::new(
            simulation,
            &RED
        ))?;
    
        Ok(())
    }
    
    fn draw_phase_graph(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        const MAX_RENDER_STEPS: usize = 20000;

        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;

        let max_population_display =
            if params.max_population_display == 0f32 {
                params.predict_max_population_size() * 1.5f32
            } else {
                params.max_population_display
            };

        let x_axis_range = 0f32..max_population_display;
        let y_axis_range = 0f32..max_population_display;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("N(t)")
            .y_desc("N(t+1)")
            .x_labels(10)
            .y_labels(10)
            .draw()?;

        let model = params.to_model();

        // draw bisector
        chart.draw_series(LineSeries::new(
            [(0f32, 0f32), (chart.x_range().end, chart.y_range().end)],
            &BLACK
        ))?;

        // draw ratio
        chart.draw_series(LineSeries::new(
            Simulation::phase_graph_slope(model.clone())
                .time_limit(chart.x_range().end)
                .max_steps(MAX_RENDER_STEPS),
            &RED
        ))?;

        // draw phase graph
        chart.draw_series(LineSeries::new(
            Simulation::phase_graph_lines(model.clone())
                .time_limit(chart.x_range().end)
                .max_steps(MAX_RENDER_STEPS),
            &BLACK
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = DDS_LBMM_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { max_time: 1f32, initial_population: 1f32,
            birth_death_rate: 1f32, migration_coefficient: 0f32, max_population_display: 0f32 }
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_population(mut self, initial_population: f32) -> Self {
        self.initial_population = initial_population;
        self
    }

    pub fn birth_death_rate(mut self, birth_death_rate: f32) -> Self {
        self.birth_death_rate = birth_death_rate;
        self
    }

    pub fn migration_coefficient(mut self, migration_coefficient: f32) -> Self {
        self.migration_coefficient = migration_coefficient;
        self
    }

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
        self
    }

    fn predict_max_population_size(&self) -> f32 {
        if self.birth_death_rate > 1f32 {
            f32::powf(self.birth_death_rate, self.max_time) * self.initial_population +
                (0..self.max_time as u32).into_iter()
                .map(|i| f32::powf(self.birth_death_rate, i as f32) * self.migration_coefficient).sum::<f32>()
        } else if self.birth_death_rate == 1f32 {
            self.initial_population + self.max_time * self.migration_coefficient
        } else {
            //migration equilibrium value
            let equilibrium = self.migration_coefficient / (1f32 - self.birth_death_rate);

            f32::max(equilibrium, self.initial_population)
        }
    }

    fn to_model(self) -> LinearBirthModel {
        LinearBirthModel::new(
            self.initial_population,
            self.birth_death_rate,
            self.migration_coefficient,
            1f32,
            200f32,
        )
    }
}