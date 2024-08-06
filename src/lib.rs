pub mod prelude {
    pub use super::utils::prelude::*;
}

pub mod discrete_dynamical_systems;
pub mod continuous_dynamical_systems;
pub mod chemical_reactions;
pub mod stochastic_simulation;
pub mod utils;

// WASM specific allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;