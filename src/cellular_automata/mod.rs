pub mod pages {
    mod traffic_jam;
    mod elementary_automaton;
}

pub mod prelude {
    pub use super::elementary_automaton::*;
    pub use super::common::*;
}

mod elementary_automaton;
mod common;