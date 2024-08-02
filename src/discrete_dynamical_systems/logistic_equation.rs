#[derive(Clone)]
pub struct LogisticEquation {
    initial_population: f32,
    birth_rate: f32,
    carrying_capacity: f32,

    last_state: Option<(f32, f32)>
}

impl LogisticEquation {
    pub fn new(initial_population: f32, birth_rate: f32, carrying_capacity: f32) -> Self {
        LogisticEquation {
            initial_population,
            birth_rate,
            carrying_capacity,
            last_state: None,
        }
    }
}

impl Iterator for LogisticEquation {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.last_state {
            Some((time, population)) => {
                let next_time = time + 1f32;
                let next_population = self.birth_rate * population * (1f32 - population / self.carrying_capacity);
                
                self.last_state = Some((next_time, next_population));
                self.last_state.clone()
            },
            None => {
                self.last_state = Some((0f32, self.initial_population));
                self.last_state.clone()
            },
        }
    }
}