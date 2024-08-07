mod pages {
    mod enzymatic_activity;
}

pub mod prelude {
    pub use super::stochastic_simulation_algorithm::*;
    pub use super::enzymatic_activity::*;
}

mod stochastic_simulation_algorithm;
mod enzymatic_activity;