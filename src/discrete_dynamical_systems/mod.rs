mod pages {
    mod linear_birth_model;
    mod linear_birth_death_model;
    mod linear_birth_model_with_migration;
}

mod prelude {
    pub use super::linear_birth_model::*;
}

mod linear_birth_model;