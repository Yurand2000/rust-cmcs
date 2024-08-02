use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::discrete_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = DDS_LE)]
pub struct Model { }

#[wasm_bindgen(js_name = DDS_LE_Params)]
pub struct Params {
    max_time: f32,
    initial_population: f32,
    birth_rate: f32,
    carrying_capacity: f32,
    max_population_display: f32,
}

#[wasm_bindgen(js_class = DDS_LE)]
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

        let birth_model = params.to_model();

        let simulation = Simulation::new(birth_model)
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
            Simulation::new(model.clone())
                .time_limit(chart.x_range().end)
                .max_steps(MAX_RENDER_STEPS)
                .phase_graph_slope(),
            &RED
        ))?;

        // draw phase graph
        chart.draw_series(LineSeries::new(
            Simulation::new(model)
                .time_limit(chart.x_range().end)
                .max_steps(MAX_RENDER_STEPS)
                .phase_graph_lines(),
            &BLACK
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = DDS_LE_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { max_time: 1f32, initial_population: 1f32,
            birth_rate: 1f32, carrying_capacity: 20f32, max_population_display: 0f32 }
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_population(mut self, initial_population: f32) -> Self {
        self.initial_population = initial_population;
        self
    }

    pub fn birth_rate(mut self, birth_rate: f32) -> Self {
        self.birth_rate = birth_rate;
        self
    }

    pub fn carrying_capacity(mut self, carrying_capacity: f32) -> Self {
        self.carrying_capacity = carrying_capacity;
        self
    }

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
        self
    }

    fn predict_max_population_size(&self) -> f32 {
        let equilibrium = self.carrying_capacity * (1f32 - 1f32 / self.birth_rate);

        f32::max(equilibrium, self.initial_population)
    }

    fn to_model(self) -> LogisticEquation {
        LogisticEquation::new(
            self.initial_population,
            self.birth_rate,
            self.carrying_capacity
        )
    }
}