use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::discrete_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = DDS_LBDM)]
pub struct Model { }

#[wasm_bindgen(js_name = DDS_LBDM_Params)]
#[derive(Default)]
pub struct Params {
    max_time: f32,
    initial_population: f32,
    birth_rate: f32,
    death_rate: f32,
}

#[wasm_bindgen(js_class = DDS_LBDM)]
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
    
        let max_population_display =
            (params.predict_max_population_size() * 1.5f32) as u32;

        let x_axis_range = 0f32..params.max_time;
        let y_axis_range = 0..max_population_display;
    
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
            .y_label_formatter(&|x| format!("{:e}", x))
            .draw()?;

        let model = params.to_model();

        let simulation = Simulation::new(model)
            .simulation_map(|(x, y)| (x, y as u32))
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
            params.predict_max_population_size() * 1.5f32;

        let max_time = params.max_time;
        let x_axis_range = 0f32..max_population_display;
        let y_axis_range = 0f32..max_population_display;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(40u32)
            .y_label_area_size(60u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("N(t)")
            .y_desc("N(t+1)")
            .x_labels(10)
            .y_labels(10)
            .x_label_formatter(&|x| format!("{:e}", x))
            .y_label_formatter(&|x| format!("{:e}", x))
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
                .time_limit(max_time)
                .max_steps(MAX_RENDER_STEPS)
                .phase_graph_slope(),
            &RED
        ))?;

        // draw phase graph
        chart.draw_series(LineSeries::new(
            Simulation::new(model)
                .time_limit(max_time)
                .max_steps(MAX_RENDER_STEPS)
                .phase_graph_lines(),
            &BLACK
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = DDS_LBDM_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default() }
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

    pub fn death_rate(mut self, death_rate: f32) -> Self {
        self.death_rate = death_rate;
        self
    }

    fn predict_max_population_size(&self) -> f32 {
        let rate = self.birth_rate - self.death_rate;
        if rate > 1f32 {
            f32::powf(rate, self.max_time) * self.initial_population
        } else {
            self.initial_population
        }
    }
    
    fn to_model(self) -> LinearBirthModel {
        let birth_rate = self.birth_rate - self.death_rate;

        LinearBirthModel::new(
            self.initial_population,
            birth_rate,
            0f32, //no migration
            1f32,
        )
    }
}