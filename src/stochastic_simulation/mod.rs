mod pages {
    mod enzymatic_activity;
    mod lotka_volterra;
    mod negative_feedback_loop;
}

pub mod prelude {
    pub use super::stochastic_simulation_algorithm::*;
    pub use super::enzymatic_activity::*;
    pub use super::lotka_volterra::*;
    pub use super::negative_feedback_loop::*;
}

mod stochastic_simulation_algorithm;
mod enzymatic_activity;
mod lotka_volterra;
mod negative_feedback_loop;