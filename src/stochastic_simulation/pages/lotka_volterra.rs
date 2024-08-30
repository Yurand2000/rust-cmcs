use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::ODESolver;
use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

#[wasm_bindgen(js_name = SSA_LV)]
pub struct Model { }

#[wasm_bindgen(js_name = SSA_LV_Params)]
#[derive(Default)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_prey_pop: u32,
    initial_predator_pop: u32,
    prey_birth_rate: f32,
    predator_death_rate: f32,
    hunting_meetings: f32,
    hunt_offsprings: u32,
    seed: u64,
}

#[wasm_bindgen(js_class = SSA_LV)]
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

        let (prey, predator) = LotkaVolterra::species();

        let max_population_display = simulation.clone()
            .map(|(_, data)| data.iter()
                .map(|(_, q)| (*q) as f32).reduce(f32::max).unwrap_or(0f32)
            )
            .reduce(f32::max).unwrap_or(0f32) * 1.5f32;

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

        // prey quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &prey).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("V (Preys)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));

        // predator quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &predator).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P (Predators)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));
    
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

        let (prey, predator) = LotkaVolterra::species();

        let max_population_display = (simulation.clone()
            .map(|(_, data)| data.iter()
                .map(|(_, q)| (*q) as f32).reduce(f32::max).unwrap_or(0f32)
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

        // prey quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &prey).unwrap().1;

                (x, quantity)
            }),
            &RED
        ))?
        .label("V (Preys)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));

        // predator quantity
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| {
                let quantity = pops.iter()
                    .find(|(mol, _)| mol == &predator).unwrap().1;

                (x, quantity)
            }),
            &BLUE
        ))?
        .label("P (Predators)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = SSA_LV_Params)]
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

    pub fn initial_prey_population(mut self, initial_prey_pop: u32) -> Self {
        self.initial_prey_pop = initial_prey_pop;
        self
    }
    
    pub fn initial_predator_population(mut self, initial_predator_pop: u32) -> Self {
        self.initial_predator_pop = initial_predator_pop;
        self
    }

    pub fn prey_birth_rate(mut self, prey_birth_rate: f32) -> Self {
        self.prey_birth_rate = prey_birth_rate;
        self
    }

    pub fn predator_death_rate(mut self, predator_death_rate: f32) -> Self {
        self.predator_death_rate = predator_death_rate;
        self
    }

    pub fn hunting_meetings(mut self, hunting_meetings: f32) -> Self {
        self.hunting_meetings = hunting_meetings;
        self
    }

    pub fn hunt_offsprings(mut self, hunt_offsprings: u32) -> Self {
        self.hunt_offsprings = hunt_offsprings;
        self
    }

    pub fn ssa_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn to_ode_model(self) -> ODESimulation {
        LotkaVolterra::make_ode(
            self.initial_prey_pop,
            self.initial_predator_pop,
            self.prey_birth_rate,
            self.predator_death_rate,
            self.hunting_meetings,
            self.hunt_offsprings,
            self.solver,
            self.max_time
        )
    }

    fn to_ssa_model(self) -> StochasticSimulation {
        LotkaVolterra::make_ssa(
            self.initial_prey_pop,
            self.initial_predator_pop,
            self.prey_birth_rate,
            self.predator_death_rate,
            self.hunting_meetings,
            self.hunt_offsprings,
            self.seed,
        )
    }
}