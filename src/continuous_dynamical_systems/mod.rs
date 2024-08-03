mod pages {
    mod linear_birth_model;
    mod radioactive_decay;
}

mod prelude {
    pub use super::linear_birth_model::*;
}

mod linear_birth_model;