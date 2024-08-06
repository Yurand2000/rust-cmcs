mod pages {
    mod enzimatic_activity;
}

pub mod prelude {
    pub use super::stochastic_simulation_algorithm::*;
    pub use super::enzimatic_activity::*;
}

mod stochastic_simulation_algorithm;
mod enzimatic_activity;