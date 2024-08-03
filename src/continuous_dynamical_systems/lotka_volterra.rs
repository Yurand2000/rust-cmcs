use ode_solvers::*;

pub struct LotkaVolterra {
    pub initial_state: Vector2<f32>,
    pub ode: LotkaVolterraODE,
    pub max_time: f32,
}

#[derive(Clone)]
pub struct LotkaVolterraODE {
    prey_birth_rate: f32,
    predator_death_rate: f32,
    hunting_meetings: f32,
    hunt_offsprings: f32,
}

impl LotkaVolterra {
    pub fn new(initial_population: (f32, f32), prey_birth_rate: f32, predator_death_rate: f32, hunting_meetings: f32, hunt_offsprings: f32, max_time: f32) -> Self {
        Self {
            initial_state: [initial_population.0, initial_population.1].into(),
            ode: LotkaVolterraODE { prey_birth_rate, predator_death_rate, hunting_meetings, hunt_offsprings },
            max_time,
        }
    }
}

impl ode_solvers::System<f32, Vector2<f32>> for LotkaVolterraODE {
    fn system(&self, _: f32, y: &Vector2<f32>, dy: &mut Vector2<f32>) {
        dy[0] = self.prey_birth_rate * y[0] - self.hunting_meetings * y[0] * y[1];
        dy[1] = -self.predator_death_rate * y[1] + self.hunting_meetings * self.hunt_offsprings * y[0] * y[1];
    }
}