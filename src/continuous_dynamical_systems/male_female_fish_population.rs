use ode_solvers::*;

pub struct MaleFemaleFishPopulation {
    pub initial_state: Vector2<f32>,
    pub ode: MaleFemaleFishPopulationODE,
    pub max_time: f32,
}

#[derive(Clone)]
pub struct MaleFemaleFishPopulationODE {
    birth_rate: f32,
    carrying_capacity: f32,
    male_death_rate: f32
}

impl MaleFemaleFishPopulation {
    pub fn new(initial_population: (f32, f32), birth_rate: f32, carrying_capacity: f32, male_death_rate: f32, max_time: f32) -> Self {
        Self {
            initial_state: [initial_population.0, initial_population.1].into(),
            ode: MaleFemaleFishPopulationODE { birth_rate, carrying_capacity, male_death_rate },
            max_time,
        }
    }
}

impl ode_solvers::System<f32, Vector2<f32>> for MaleFemaleFishPopulationODE {
    fn system(&self, _: f32, y: &Vector2<f32>, dy: &mut Vector2<f32>) {
        dy[0] = self.birth_rate * y[0] * (1f32 - (y[0] + y[1]) / self.carrying_capacity);
        dy[1] = self.birth_rate * y[0] * (1f32 - (y[0] + y[1]) / self.carrying_capacity) - self.male_death_rate * y[1];
    }
}