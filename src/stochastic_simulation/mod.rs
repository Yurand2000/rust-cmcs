mod pages {
    mod enzymatic_activity;
    mod lotka_volterra;
}

pub mod prelude {
    pub use super::stochastic_simulation_algorithm::*;
    pub use super::enzymatic_activity::*;
    pub use super::lotka_volterra::*;
}

mod stochastic_simulation_algorithm;
mod enzymatic_activity;
mod lotka_volterra;