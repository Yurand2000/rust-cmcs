use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = CDS_SLE_SIR_V)]
pub struct Model { }

#[wasm_bindgen(js_name = CDS_SLE_SIR_V_Params)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_susceptible_pop: f32,
    initial_infected_pop: f32,
    initial_recovered_pop: f32,
    infection_coefficient: f32,
    recovery_coefficient: f32,
    birth_rate: f32,
    vaccination_coefficient: f32,
    max_population_display: f32,
}

#[wasm_bindgen(js_class = CDS_SLE_SIR_V)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
    }

    fn draw_function(canvas: HtmlCanvasElement, params: Params) -> MyDrawResult<()> {
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;
    
        let x_axis_range = 0f32..params.max_time;
        let y_axis_range = 0f32..1f32;
    
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

        let simulation = time.iter().zip(population.iter())
            .map(|(time, res)| (*time, (res[0], res[1], res[2])));
        let simulation = Simulation::new(simulation)
            .time_limit(chart.x_range().end);

        // susceptible population
        chart.draw_series(LineSeries::new(
            simulation.clone().map(|(x, pops)| (x, pops.0)),
            &GREEN
        ))?;

        // infected population
        chart.draw_series(LineSeries::new(
            simulation.clone().map(|(x, pops)| (x, pops.1)),
            &RED
        ))?;

        // recovered population
        chart.draw_series(LineSeries::new(
            simulation.clone().map(|(x, pops)| (x, pops.2)),
            &BLUE
        ))?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = CDS_SLE_SIR_V_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { solver: ODESolver::RK4, max_time: 1f32, max_population_display: 0f32, initial_susceptible_pop: 0f32, initial_infected_pop: 0f32, initial_recovered_pop: 0f32,
            infection_coefficient: 0f32, recovery_coefficient: 0f32, birth_rate: 0f32, vaccination_coefficient: 0f32 }
    }

    pub fn solver(mut self, solver: String) -> Self {
        self.solver = ODESolver::from_string(solver).unwrap();
        self
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_susceptible_population(mut self, initial_susceptible_pop: f32) -> Self {
        self.initial_susceptible_pop = initial_susceptible_pop;
        self
    }

    pub fn initial_infected_population(mut self, initial_infected_pop: f32) -> Self {
        self.initial_infected_pop = initial_infected_pop;
        self
    }

    pub fn initial_recovered_population(mut self, initial_recovered_pop: f32) -> Self {
        self.initial_recovered_pop = initial_recovered_pop;
        self
    }

    pub fn infection_coefficient(mut self, infection_coefficient: f32) -> Self {
        self.infection_coefficient = infection_coefficient;
        self
    }

    pub fn recovery_coefficient(mut self, recovery_coefficient: f32) -> Self {
        self.recovery_coefficient = recovery_coefficient;
        self
    }

    pub fn birth_rate(mut self, birth_rate: f32) -> Self {
        self.birth_rate = birth_rate;
        self
    }

    pub fn vaccination_coefficient(mut self, vaccination_coefficient: f32) -> Self {
        self.vaccination_coefficient = vaccination_coefficient;
        self
    }

    pub fn max_population_display(mut self, max_population_display: f32) -> Self {
        self.max_population_display = max_population_display;
        self
    }

    fn to_model(self) -> SIRModel {
        SIRModel::new(
            (self.initial_susceptible_pop, self.initial_infected_pop, self.initial_recovered_pop),
            self.infection_coefficient,
            self.recovery_coefficient,
            self.birth_rate,
            self.vaccination_coefficient,
            self.max_time,
        )
    }
}