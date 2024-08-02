mod pages {
    mod linear_birth_model;
    mod linear_birth_death_model;
    mod linear_birth_model_with_migration;
    mod logistic_equation;
    mod male_female_fish_population;
}

mod prelude {
    pub use super::linear_birth_model::*;
    pub use super::logistic_equation::*;
    pub use super::male_female_fish_population::*;
}

mod linear_birth_model;
mod logistic_equation;
mod male_female_fish_population;