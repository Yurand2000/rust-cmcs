use ode_solvers::*;

pub struct SIRModel {
    pub initial_state: Vector3<f32>,
    pub ode: SIRModelODE,
    pub max_time: f32,
}

#[derive(Clone)]
pub struct SIRModelODE {
    infection_coefficient: f32,
    recovery_coefficient: f32,
    birth_rate: f32,
    vaccination_rate: f32,
}

impl SIRModel {
    pub fn new(initial_population: (f32, f32, f32), infection_coefficient: f32, recovery_coefficient: f32, birth_rate: f32, vaccination_rate: f32, max_time: f32) -> Self {
        Self {
            initial_state: [initial_population.0, initial_population.1, initial_population.2].into(),
            ode: SIRModelODE { infection_coefficient, recovery_coefficient, birth_rate, vaccination_rate },
            max_time
        }
    }
}

impl ode_solvers::System<f32, Vector3<f32>> for SIRModelODE {
    fn system(&self, _: f32, y: &Vector3<f32>, dy: &mut Vector3<f32>) {
        let (s, i, r) = (y[0], y[1], y[2]);

        let ds = (1f32 - self.vaccination_rate) * self.birth_rate - self.infection_coefficient * s * i - self.birth_rate * s;
        let di = self.infection_coefficient * s * i - self.recovery_coefficient * i - self.birth_rate * i;
        let dr = self.vaccination_rate * self.birth_rate + self.recovery_coefficient * i - self.birth_rate * r;

        dy[0] = ds; dy[1] = di; dy[2] = dr;
    }
}