use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::discrete_event_simulation::customer_queue::CustomerQueueState;
use crate::prelude::*;
use crate::discrete_event_simulation::prelude::*;

#[wasm_bindgen(js_name = DES_CQ)]
pub struct Model { }

#[wasm_bindgen(js_name = DES_CQ_Params)]
#[derive(Default)]
pub struct Params {
    max_time: f32,
    customer_arrival_lambda: f32,
    customer_served_mean: f32,
    customer_served_std_dev: f32,
    simulation_seed: u64,
}

#[wasm_bindgen(js_class = DES_CQ)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
    }

    fn draw_function(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let max_time = params.max_time;

        let model = params.to_model();
        let simulation = Simulation::new(model)
            .time_limit(max_time)
            .cache();

        let max_queue = simulation.clone()
            .map(|(_, state)| state.queue_length)
            .max().unwrap_or(0);

        let x_axis_range = 0f32..max_time;
        let y_axis_range = 0..max_queue;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(10)
            .y_labels(10)
            .draw()?;

        chart.draw_series(LineSeries::new(
            simulation.clone()
                .map(|(time, state)| (time, state.queue_length)),
            &BLUE
        ))?;

        chart.draw_series(LineSeries::new(
            simulation.clone()
                .map(|(time, state)| (time, (state.operator_available as u32))),
            &GREEN
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = DES_CQ_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn customer_arrival_lambda(mut self, customer_arrival_lambda: f32) -> Self {
        self.customer_arrival_lambda = customer_arrival_lambda;
        self
    }

    pub fn customer_served_mean(mut self, customer_served_mean: f32) -> Self {
        self.customer_served_mean = customer_served_mean;
        self
    }

    pub fn customer_served_std_dev(mut self, customer_served_std_dev: f32) -> Self {
        self.customer_served_std_dev = customer_served_std_dev;
        self
    }
    
    pub fn simulation_seed(mut self, simulation_seed: u64) -> Self {
        self.simulation_seed = simulation_seed;
        self
    }

    fn to_model(self) -> DiscreteEventSimulation<CustomerQueueState> {
        CustomerQueue::build_des(
            self.customer_arrival_lambda,
            self.customer_served_mean,
            self.customer_served_std_dev,
            self.simulation_seed
        )
    }
}