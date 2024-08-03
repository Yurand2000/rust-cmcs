use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::continuous_dynamical_systems::lotka_volterra::LotkaVolterra;
use crate::prelude::*;
use crate::continuous_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = CDS_SLE_LV)]
pub struct Model { }

#[wasm_bindgen(js_name = CDS_SLE_LV_Params)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_prey_pop: f32,
    initial_predator_pop: f32,
    prey_birth_rate: f32,
    predator_death_rate: f32,
    hunting_meetings: f32,
    hunt_offsprings: f32,
    max_population_display: f32,
}

#[wasm_bindgen(js_class = CDS_SLE_LV)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
    }

    fn draw_function(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let max_time = params.max_time;
        let solver = params.solver;
        let model = params.to_model();
        let step_size = 0.01f32;

        use ode_solvers::*;
        let results =
            match solver {
                ODESolver::DOP853 => {
                    let mut stepper = Dop853::new(model.ode, 0f32, model.max_time, step_size, model.initial_state, 1.0e-2, 1.0e-6);
                    stepper.integrate()?;
                    stepper.results().to_owned()
                },
                ODESolver::DOPRI5 => {
                    let mut stepper = Dopri5::new(model.ode, 0f32, model.max_time, step_size, model.initial_state, 1.0e-2, 1.0e-6);
                    stepper.integrate()?;
                    stepper.results().to_owned()
                },
                ODESolver::RK4 => {
                    let mut stepper = Rk4::new(model.ode, 0f32, model.initial_state, model.max_time, step_size);
                    stepper.integrate()?;
                    stepper.results().to_owned()
                },
            };

        let (time, population) = results.get();

        let max_population_display = (f32::max(
            population.iter().map(|res| res[0]).reduce(f32::max).unwrap(),
            population.iter().map(|res| res[1]).reduce(f32::max).unwrap(),
        ) * 1.5f32) as u32;

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

        let simulation = time.iter().zip(population.iter()).map(|(time, res)| (*time, (res[0], res[1])));
        let simulation = Simulation::new(simulation)
            .time_limit(chart.x_range().end);

        // prey population
        chart.draw_series(LineSeries::new(
            simulation.clone().map(|(x, pops)| (x, pops.0 as u32)),
            &BLUE
        ))?;

        // predator population
        chart.draw_series(LineSeries::new(
            simulation.clone().map(|(x, pops)| (x, pops.1 as u32)),
            &RED
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = CDS_SLE_LV_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { solver: ODESolver::RK4, max_time: 1f32, initial_prey_pop: 1f32, initial_predator_pop: 1f32, prey_birth_rate: 1f32, 
            predator_death_rate: 1f32, hunting_meetings: 0f32, hunt_offsprings: 1f32, max_population_display: 0f32 }
    }

    pub fn solver(mut self, solver: String) -> Self {
        self.solver = ODESolver::from_string(solver).unwrap();
        self
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_prey_population(mut self, initial_prey_pop: f32) -> Self {
        self.initial_prey_pop = initial_prey_pop;
        self
    }
    
    pub fn initial_predator_population(mut self, initial_predator_pop: f32) -> Self {
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

    pub fn hunt_offsprings(mut self, hunt_offsprings: f32) -> Self {
        self.hunt_offsprings = hunt_offsprings;
        self
    }

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
        self
    }

    fn to_model(self) -> LotkaVolterra {
        LotkaVolterra::new(
            (self.initial_prey_pop, self.initial_predator_pop),
            self.prey_birth_rate,
            self.predator_death_rate,
            self.hunting_meetings,
            self.hunt_offsprings,
            self.max_time,
        )
    }
}