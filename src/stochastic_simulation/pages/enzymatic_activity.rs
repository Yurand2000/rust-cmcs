use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::ODESolver;
use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

#[wasm_bindgen(js_name = SSA_EA)]
pub struct Model { }

#[wasm_bindgen(js_name = SSA_EA_Params)]
#[derive(Default)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_enzyme: u32,
    initial_reactant: u32,
    binding_rate: f32,
    unbinding_rate: f32,
    catalysis_rate: f32,
    seed: u64,
}

#[wasm_bindgen(js_class = SSA_EA)]
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
        let max_population_display = params.initial_reactant as f32;
        let model = params.to_ode_model();

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

        let simulation = model.into_iter();
        let simulation = Simulation::new(simulation)
            .time_limit(chart.x_range().end);

        let (_, reactant, bound_reactant, product) = EnzymaticActivity::species();

        // reactant quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &reactant).unwrap().1;

                (x, quantity)
            }),
            &GREEN
        ))?
        .label("S")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));

        // product quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &product).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // bound reactant quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &bound_reactant).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("ES")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }

    fn draw_ssa(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let max_time = params.max_time;
        let max_population_display = params.initial_reactant;
        let model = params.to_ssa_model();

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

        let simulation = Simulation::new(model)
            .fix_point(max_time + 1f32)
            .time_limit(max_time);

        let (_, reactant, bound_reactant, product) = EnzymaticActivity::species();

        // reactant quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &reactant).unwrap().1;

                (x, quantity)
            }),
            &GREEN
        ))?
        .label("S")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));

        // product quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &product).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // bound reactant quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &bound_reactant).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("ES")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = SSA_EA_Params)]
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

    pub fn initial_enzyme(mut self, initial_enzyme: u32) -> Self {
        self.initial_enzyme = initial_enzyme;
        self
    }

    pub fn initial_reactant(mut self, initial_reactant: u32) -> Self {
        self.initial_reactant = initial_reactant;
        self
    }

    pub fn binding_rate(mut self, binding_rate: f32) -> Self {
        self.binding_rate = binding_rate;
        self
    }

    pub fn unbinding_rate(mut self, unbinding_rate: f32) -> Self {
        self.unbinding_rate = unbinding_rate;
        self
    }

    pub fn catalysis_rate(mut self, catalysis_rate: f32) -> Self {
        self.catalysis_rate = catalysis_rate;
        self
    }

    pub fn ssa_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn to_ode_model(self) -> ODESimulation {
        EnzymaticActivity::make_ode(
            self.initial_enzyme,
            self.initial_reactant,
            self.binding_rate,
            self.unbinding_rate,
            self.catalysis_rate,
            self.solver,
            self.max_time
        )
    }

    fn to_ssa_model(self) -> StochasticSimulation {
        EnzymaticActivity::make_ssa(
            self.initial_enzyme,
            self.initial_reactant,
            self.binding_rate,
            self.unbinding_rate,
            self.catalysis_rate,
            self.seed,
        )
    }
}