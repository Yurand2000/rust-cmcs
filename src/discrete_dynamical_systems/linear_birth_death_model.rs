use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;

#[wasm_bindgen]
pub struct ILinearBirthDeathModel { }

#[wasm_bindgen]
pub struct ILinearBirthDeathModelParams {
    max_time: f32,
    initial_population: f32,
    birth_rate: f32,
    death_rate: f32,
    max_population_display: f32,
}

pub enum GraphType {
    Function,
    PhaseGraph
}

#[wasm_bindgen]
impl ILinearBirthDeathModel {
    pub fn draw(canvas: HtmlCanvasElement, typ: String, params: ILinearBirthDeathModelParams) -> Result<(), JsValue> {
        match GraphType::from_string(typ) {
            Some(GraphType::Function) => 
                draw_generic(Self::draw_function)(canvas, params),
            Some(GraphType::PhaseGraph) =>
                draw_generic(Self::draw_phase_graph)(canvas, params),
            None =>
                Err(format!("Graph type not supported").into())
        }
    }

    fn draw_function(canvas: HtmlCanvasElement, params: ILinearBirthDeathModelParams) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
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
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(params.max_time as usize)
            .y_labels(10)
            .draw()?;

        chart.draw_series(LineSeries::new(
            LimitedSimulation::wrap(
                LinearBirthDeathModel::new(
                    params.initial_population,
                    params.birth_rate,
                    params.death_rate
                ).map(|(x, y)| (x, y as u32)),
                chart.x_range().end,
            ),
            &RED
        ))?;
    
        Ok(())
    }
    
    fn draw_phase_graph(canvas: HtmlCanvasElement, params: ILinearBirthDeathModelParams) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;

        let max_population_display =
            if params.max_population_display == 0f32 {
                params.predict_max_population_size()
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

        let birth_model = LinearBirthDeathModel::new(
            params.initial_population,
            params.birth_rate,
            params.death_rate
        );

        // draw bisector
        chart.draw_series(LineSeries::new(
            [(0f32, 0f32), (chart.x_range().end, chart.y_range().end)],
            &BLACK
        ))?;

        // draw ratio
        chart.draw_series(LineSeries::new(
            LimitedSimulation::wrap(
                PhaseGraphSlope::new(birth_model.clone()),
                chart.x_range().end
            ),
            &RED
        ))?;

        // draw phase graph
        chart.draw_series(LineSeries::new(
            LimitedSimulation::wrap(
                PhaseGraphLines::new(birth_model),                
                chart.x_range().end
            ),
            &BLACK
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen]
impl ILinearBirthDeathModelParams {
    pub fn builder() -> Self {
        Self { max_time: 1f32, initial_population: 1f32,
            birth_rate: 1f32, death_rate: 0.1f32, max_population_display: 0f32 }
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

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
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
}

impl GraphType {
    fn from_string(value: String) -> Option<Self> {
        match value.as_str() {
            "normal" => Some(GraphType::Function),
            "phase" => Some(GraphType::PhaseGraph),
            _ => None
        }
    }
}

#[derive(Clone)]
struct LinearBirthDeathModel {
    initial_value: f32,
    grow_factor: f32,

    last_step: Option<(f32, f32)>
}

impl LinearBirthDeathModel {
    pub fn new(initial_population: f32, birth_rate: f32, death_rate: f32) -> Self {
        Self {
            initial_value: initial_population,
            grow_factor: birth_rate - death_rate,
            last_step: None,
        }
    }
}

impl Iterator for LinearBirthDeathModel {
    type Item = (f32, f32);
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((time, value)) = self.last_step {
            let next_time = time + 1f32;
            let next_value = value * self.grow_factor;
            if next_time != time || next_value != value {
                self.last_step = Some((next_time, next_value));
                self.last_step.clone()
            } else {
                None
            }
        } else {
            let next_step = (0f32, self.initial_value);
            self.last_step = Some(next_step);
            self.last_step.clone()
        }
    }
}