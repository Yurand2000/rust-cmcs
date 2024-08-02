#[derive(Clone)]
pub struct LinearBirthModel {
    initial_population: f32,
    grow_factor: f32,
    migration_factor: f32,
    step_size: f32,
    max_time: f32,

    last_step: Option<(f32, f32)>
}

impl LinearBirthModel {
    pub fn new(initial_population: f32, grow_factor: f32, migration_factor: f32, step_size: f32, max_time: f32) -> Self {
        Self {
            initial_population,
            grow_factor,
            migration_factor,
            step_size,
            max_time,
            last_step: None,
        }
    }
}

impl Iterator for LinearBirthModel {
    type Item = (f32, f32);
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.last_step {
            Some((time, population)) => {
                if time >= self.max_time {
                    None
                } else {
                    let next_time = time + self.step_size;
                    let next_population = population * self.grow_factor + self.migration_factor;
                    self.last_step = Some((next_time, next_population));
                    self.last_step.clone()
                }
            },
            None => {
                self.last_step = Some((0f32, self.initial_population));
                self.last_step.clone()
            },
        }
    }
}