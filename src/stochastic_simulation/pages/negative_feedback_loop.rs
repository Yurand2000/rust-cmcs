use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::ODESolver;
use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

#[wasm_bindgen(js_name = SSA_NFL)]
pub struct Model { }

#[wasm_bindgen(js_name = SSA_NFL_Params)]
#[derive(Default)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_state: (u32, u32, u32),
    production_rates: (f32, f32, f32),
    binding_rates: (f32, f32, f32),
    unbinding_rates: (f32, f32, f32),
    decay_rates: (f32, f32, f32),
    seed: u64,
}

#[wasm_bindgen(js_class = SSA_NFL)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, algorithm: String, params: Params) -> Result<(), JsValue> {
        match algorithm.as_str() {
            "ode" => draw_generic(Self::draw_ode)(canvas, params),
            "ssa" => draw_generic(Self::draw_ssa)(canvas, params),
            _ => Err(format!("Algorithm {algorithm} not supported").into()),
        }
    }

    fn draw_ode(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let max_time = params.max_time;
        let model = params.to_ode_model();

        let simulation = model.into_iter();
        let simulation = Simulation::new(simulation)
            .time_limit(max_time);

        let (_, (p1, p2, p3), _) = NegativeFeedbackLoop::species();

        let max_population_display = simulation.clone()
            .map(|(_, data)| data.iter()
                .filter(|(m, _)| m == &p1 || m == &p2 || m == &p3)
                .map(|(_, q)| (*q) as f32)
                .reduce(f32::max).unwrap_or(0f32)
            )
            .reduce(f32::max).unwrap_or(0f32) * 1.15f32;

        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let x_axis_range = 0f32..max_time;
        let y_axis_range = 0f32..max_population_display;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(max_time as usize)
            .y_labels(10)
            .draw()?;

        // p1 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p1).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("P1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));

        // p2 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p2).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // p3 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p3).unwrap().1;

                (x, quantity)
            }),
            &GREEN
        ))?
        .label("P3")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }

    fn draw_ssa(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let max_time = params.max_time;
        let model = params.to_ssa_model();

        let simulation = Simulation::new(model)
            .fix_point(max_time + 1f32)
            .time_limit(max_time);

        let (_, (p1, p2, p3), _) = NegativeFeedbackLoop::species();

        let max_population_display = (simulation.clone()
            .map(|(_, data)| data.iter()
                .filter(|(m, _)| m == &p1 || m == &p2 || m == &p3)
                .map(|(_, q)| (*q) as f32)
                .reduce(f32::max).unwrap_or(0f32)
            )
            .reduce(f32::max).unwrap_or(0f32) * 1.5f32) as u32;

        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let x_axis_range = 0f32..max_time;
        let y_axis_range = 0..max_population_display;
    
        let mut chart = ChartBuilder::on(&area)
            .margin(20u32)
            .x_label_area_size(30u32)
            .y_label_area_size(30u32)
            .build_cartesian_2d(x_axis_range, y_axis_range)?;
    
        chart.configure_mesh()
            .x_desc("t")
            .y_desc("N(t)")
            .x_labels(max_time as usize)
            .y_labels(10)
            .draw()?;

        // p1 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p1).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("P1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));

        // p2 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p2).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // p3 quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &p3).unwrap().1;

                (x, quantity)
            }),
            &GREEN
        ))?
        .label("P3")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = SSA_NFL_Params)]
impl Params {
    pub fn builder() -> Self { Default::default() }

    pub fn solver(mut self, solver: String) -> Self {
        self.solver = ODESolver::from_string(solver).unwrap();
        self
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn initial_state(mut self, a: u32, b: u32, c: u32) -> Self {
        self.initial_state = (a, b, c);
        self
    }

    pub fn production_rates(mut self, a: f32, b: f32, c: f32) -> Self {
        self.production_rates = (a, b, c);
        self
    }

    pub fn binding_rates(mut self, a: f32, b: f32, c: f32) -> Self {
        self.binding_rates = (a, b, c);
        self
    }

    pub fn unbinding_rates(mut self, a: f32, b: f32, c: f32) -> Self {
        self.unbinding_rates = (a, b, c);
        self
    }

    pub fn decay_rates(mut self, a: f32, b: f32, c: f32) -> Self {
        self.decay_rates = (a, b, c);
        self
    }

    pub fn ssa_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn to_ode_model(self) -> ODESimulation {
        NegativeFeedbackLoop::make_ode(
            self.initial_state,
            self.production_rates,
            self.binding_rates,
            self.unbinding_rates,
            self.decay_rates,
            self.solver,
            self.max_time
        )
    }

    fn to_ssa_model(self) -> StochasticSimulation {
        NegativeFeedbackLoop::make_ssa(
            self.initial_state,
            self.production_rates,
            self.binding_rates,
            self.unbinding_rates,
            self.decay_rates,
            self.seed,
        )
    }
}