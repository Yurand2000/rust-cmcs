#[derive(Clone)]
pub struct LogisticEquation {
    initial_population: f32,
    birth_rate: f32,
    carrying_capacity: f32,

    last_state: Option<f32>
}

impl LogisticEquation {
    pub fn new(initial_population: f32, birth_rate: f32, carrying_capacity: f32) -> Self {
        Self {
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
            Some(time) => {
                let next_time = time + 1f32;

                let num = self.carrying_capacity;
                let exp = f32::exp(-self.birth_rate * next_time);
                let den = 1f32 + (self.carrying_capacity / self.initial_population - 1f32) * exp;
                let next_population = num / den;

                self.last_state = Some(next_time);
                Some((next_time, next_population))
            },
            None => {
                self.last_state = Some(0f32);
                Some((0f32, self.initial_population))
            },
        }
    }
}