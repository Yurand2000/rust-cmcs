pub mod pages {
    mod traffic_jam;
    mod elementary_automaton;
    mod maze_solver;
    mod game_of_life;
    mod forest_fire;
}

pub mod prelude {
    pub use super::automaton_2d as automaton_2d;
    pub use super::elementary_automaton as elementary;
    pub use super::maze_solver as maze;
    pub use super::game_of_life as game_of_life;
    pub use super::forest_fire as forest_fire;
    pub use super::common::*;
}

pub mod elementary_automaton;
pub mod automaton_2d;
pub mod maze_solver;
pub mod game_of_life;
pub mod forest_fire;
pub mod common;