#[derive(Clone)]
pub struct MaleFemaleFishPopulation {
    initial_population: (f32, f32),
    birth_rate: f32,
    male_death_rate: f32,
    carrying_capacity: f32,

    last_state: Option<(f32, (f32, f32))>
}

impl MaleFemaleFishPopulation {
    pub fn new(initial_population: (f32, f32), birth_rate: f32, male_fights: f32, carrying_capacity: f32) -> Self {
        Self {
            initial_population,
            birth_rate,
            male_death_rate: male_fights,
            carrying_capacity,
            last_state: None,
        }
    }
}

impl Iterator for MaleFemaleFishPopulation {
    type Item = (f32, (f32, f32));

    fn next(&mut self) -> Option<Self::Item> {
        match self.last_state {
            Some((time, (female_pop, male_pop))) => {
                let total_pop = female_pop + male_pop;

                let next_time = time + 1f32;
                let next_female_pop = f32::max(0f32, self.birth_rate * female_pop * (1f32 - total_pop / self.carrying_capacity));
                let next_male_pop = f32::max(0f32, self.birth_rate * female_pop * (1f32 - total_pop / self.carrying_capacity) - self.male_death_rate * male_pop);

                self.last_state = Some((next_time, (next_female_pop, next_male_pop)));
                self.last_state.clone()
            },
            None => {
                self.last_state = Some((0f32, self.initial_population));
                self.last_state.clone()
            },
        }
    }
}