pub mod pages {
    mod customer_queue;
}

pub mod prelude {
    pub use super::discete_event_simulation::*;
    pub use super::customer_queue::*;
}

mod discete_event_simulation;
mod customer_queue;