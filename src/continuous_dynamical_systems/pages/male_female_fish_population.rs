use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::continuous_dynamical_systems::prelude::*;

#[wasm_bindgen(js_name = CDS_SLE_MFFP)]
pub struct Model { }

#[wasm_bindgen(js_name = CDS_SLE_MFFP_Params)]
#[derive(Default)]
pub struct Params {
    solver: ODESolver,
    max_time: f32,
    initial_female_pop: f32,
    initial_male_pop: f32,
    birth_rate: f32,
    carrying_capacity: f32,
    male_death_rate: f32,
}

#[wasm_bindgen(js_class = CDS_SLE_MFFP)]
impl Model {
    pub fn draw(canvas: HtmlCanvasElement, params: Params) -> Result<(), JsValue> {
        draw_generic(Self::draw_function)(canvas, params)
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

        let simulation = time.iter().zip(population.iter()).map(|(time, res)| (*time, (res[0], res[1])));
        let simulation = Simulation::new(simulation)
            .time_limit(chart.x_range().end);

        // female population
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| (x, pops.0 as u32)),
            &BLUE
        ))?
        .label("F(t)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        // male population
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| (x, pops.1 as u32)),
            &RED
        ))?
        .label("M(t)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));

        // total population
        chart.draw_series(LineSeries::new(
            simulation.clone().simulation_map(|(x, pops)| (x, (pops.0 + pops.1) as u32)),
            &GREEN
        ))?
        .label("F(t) + M(t)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], GREEN));
    
        // draw legend
        chart.configure_series_labels()
            .background_style(WHITE)
            .draw()?;
    
        Ok(())
    }
}

#[wasm_bindgen(js_class = CDS_SLE_MFFP_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn solver(mut self, solver: String) -> Self {
        self.solver = ODESolver::from_string(solver).unwrap();
        self
    }

    pub fn max_time(mut self, max_time: f32) -> Self {
        self.max_time = max_time;
        self
    }
    
    pub fn initial_female_population(mut self, initial_female_pop: f32) -> Self {
        self.initial_female_pop = initial_female_pop;
        self
    }
    
    pub fn initial_male_population(mut self, initial_male_pop: f32) -> Self {
        self.initial_male_pop = initial_male_pop;
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

    pub fn male_death_rate(mut self, male_death_rate: f32) -> Self {
        self.male_death_rate = male_death_rate;
        self
    }

    fn predict_max_population_size(&self) -> f32 {
        f32::max(self.initial_female_pop + self.initial_male_pop, self.carrying_capacity)
    }

    fn to_model(self) -> MaleFemaleFishPopulation {
        MaleFemaleFishPopulation::new(
            (self.initial_female_pop, self.initial_male_pop),
            self.birth_rate,
            self.carrying_capacity,
            self.male_death_rate,
            self.max_time,
        )
    }
}