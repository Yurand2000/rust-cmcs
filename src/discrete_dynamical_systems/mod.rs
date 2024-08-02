mod pages {
    mod linear_birth_model;
    mod linear_birth_death_model;
    mod linear_birth_model_with_migration;
    mod logistic_equation;
}

mod prelude {
    pub use super::linear_birth_model::*;
    pub use super::logistic_equation::*;
}

mod linear_birth_model;
mod logistic_equation;