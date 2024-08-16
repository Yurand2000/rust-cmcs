pub mod pages {
    mod traffic_jam;
    mod elementary_automaton;
    mod maze_solver;
}

pub mod prelude {
    pub use super::elementary_automaton as elementary;
    pub use super::maze_solver as maze;
    pub use super::common::*;
}

pub mod elementary_automaton;
pub mod maze_solver;
pub mod common;