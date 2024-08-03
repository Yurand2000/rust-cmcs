mod pages {
    mod linear_birth_model;
    mod radioactive_decay;
    mod logistic_equation;
}

mod prelude {
    pub use super::linear_birth_model::*;
    pub use super::logistic_equation::*;
}

mod linear_birth_model;
mod logistic_equation;