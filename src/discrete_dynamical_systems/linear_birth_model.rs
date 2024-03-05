use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;

pub mod prelude { }

#[wasm_bindgen]
pub struct ILinearBirthModel {

}

#[wasm_bindgen]
pub struct ILinearBirthModelParams {
    max_time: f32,
    time_step: f32,
    initial_population: f32,
    offsprings_per_individual: f32,
    reproduction_period: f32,
    max_population_display: f32,
}

#[wasm_bindgen]
impl ILinearBirthModel {
    pub fn draw(canvas: HtmlCanvasElement, params: ILinearBirthModelParams) -> Result<(), JsValue> {
        draw_generic(Self::draw_inner)(canvas, params)
    }

    fn draw_inner(canvas: HtmlCanvasElement, params: ILinearBirthModelParams) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        let font: FontDesc = ("sans-serif", 20.0).into();
        area.fill(&WHITE)?;
    
        let x_axis_range = 0f32..params.max_time;

        let max_population_display =
            if params.max_population_display == 0f32 {
                params.predict_max_population_size()
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
            .x_labels(params.max_time as usize)
            .y_labels(10)
            .draw()?;

        chart.draw_series(LineSeries::new(
            LimitedSimulation::wrap(
                LinearBirthModel::new(
                    params.initial_population,
                    params.time_step,
                    params.offsprings_per_individual,
                    params.reproduction_period
                ).map(|(x, y)| (x, y as u32)),
                params.max_time,
            ),
            &RED
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen]
impl ILinearBirthModelParams {
    pub fn builder() -> Self {
        Self { max_time: 1f32, time_step: 1f32, initial_population: 1f32,
            offsprings_per_individual: 1f32, reproduction_period: 1f32, max_population_display: 0f32 }
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn time_step(mut self, time_step: f32) -> Self {
        self.time_step = time_step;
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
        let rate = self.offsprings_per_individual * self.time_step / self.reproduction_period + 1f32;
        f32::powf(rate, self.max_time / self.time_step) * self.initial_population
    }
}


struct LinearBirthModel {
    initial_value: f32,
    grow_factor: f32,
    step_size: f32,

    last_step: Option<(f32, f32)>
}

impl LinearBirthModel {
    pub fn new(initial_population: f32, time_step: f32, offsprings_per_individual: f32, reproduction_period: f32) -> Self {
        Self {
            initial_value: initial_population,
            grow_factor: 1f32 + offsprings_per_individual * time_step / reproduction_period,
            step_size: time_step,
            last_step: None,
        }
    }
}

impl Iterator for LinearBirthModel {
    type Item = (f32, f32);
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((time, value)) = self.last_step {
            let next_time = time + self.step_size;
            let next_value = value * self.grow_factor;
            self.last_step = Some((next_time, next_value));
            self.last_step.clone()
        } else {
            let next_step = (0f32, self.initial_value);
            self.last_step = Some(next_step);
            self.last_step.clone()
        }
    }
}