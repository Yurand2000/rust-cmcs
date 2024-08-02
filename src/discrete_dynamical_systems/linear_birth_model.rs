#[derive(Clone)]
pub struct LinearBirthModel {
    grow_factor: f32,
    migration_factor: f32,
    step_size: f32,
    max_time: f32,

    last_step: (f32, f32)
}

impl LinearBirthModel {
    pub fn new(initial_population: f32, grow_factor: f32, migration_factor: f32, step_size: f32, max_time: f32) -> Self {
        Self {
            grow_factor,
            migration_factor,
            step_size,
            max_time,
            last_step: (0f32, initial_population),
        }
    }
}

impl Iterator for LinearBirthModel {
    type Item = (f32, f32);
    
    fn next(&mut self) -> Option<Self::Item> {
        let (time, population) = self.last_step;

        if time >= self.max_time {
            None
        } else {
            let next_time = time + self.step_size;
            let next_population = population * self.grow_factor + self.migration_factor;
            self.last_step = (next_time, next_population);

            Some(self.last_step)
        }
    }
}