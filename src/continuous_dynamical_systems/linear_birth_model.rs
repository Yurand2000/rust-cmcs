#[derive(Clone)]
pub struct LinearBirthModel {
    initial_population: f32,
    birth_rate: f32,

    last_step: Option<f32>
}

impl LinearBirthModel {
    pub fn new(initial_population: f32, birth_rate: f32) -> Self {
        Self { initial_population, birth_rate, last_step: None }
    }
}

impl Iterator for LinearBirthModel {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.last_step {
            Some(time) => {
                let next_time = time + 1f32;
                let next_population = self.initial_population * f32::exp(self.birth_rate * next_time);
                self.last_step = Some(next_time);
                Some((next_time, next_population))
            },
            None => {
                self.last_step = Some(0f32);
                Some((0f32, self.initial_population))
            },
        }
    }
}