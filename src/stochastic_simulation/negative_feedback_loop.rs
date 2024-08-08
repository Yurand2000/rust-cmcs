use std::collections::HashMap;

use crate::continuous_dynamical_systems::ODESolver;
use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

pub struct NegativeFeedbackLoop;

impl NegativeFeedbackLoop {
    fn reactions(production_rates: (f32, f32, f32), binding_rates: (f32, f32, f32), unbinding_rates: (f32, f32, f32), decay_rates: (f32, f32, f32)) -> Vec<Reaction> {
        let ((g1, g2, g3), (p1, p2, p3), (p1g2, p2g3, p3g1)) = Self::species();

        vec![
            Reaction::new(production_rates.0, [(1, &g1)], [(1, &g1), (1, &p1)]),
            Reaction::new(production_rates.1, [(1, &g2)], [(1, &g2), (1, &p2)]),
            Reaction::new(production_rates.2, [(1, &g3)], [(1, &g3), (1, &p3)]),

            Reaction::new(binding_rates.0, [(1, &p1), (1, &g2)], [(1, &p1g2)]),
            Reaction::new(binding_rates.1, [(1, &p2), (1, &g3)], [(1, &p2g3)]),
            Reaction::new(binding_rates.2, [(1, &p3), (1, &g1)], [(1, &p3g1)]),
            
            Reaction::new(unbinding_rates.0, [(1, &p1g2)], [(1, &p1), (1, &g2)]),
            Reaction::new(unbinding_rates.1, [(1, &p2g3)], [(1, &p2), (1, &g3)]),
            Reaction::new(unbinding_rates.2, [(1, &p3g1)], [(1, &p3), (1, &g1)]),

            Reaction::new(decay_rates.0, [(1, &p1)], []),
            Reaction::new(decay_rates.1, [(1, &p2)], []),
            Reaction::new(decay_rates.2, [(1, &p3)], []),
        ]
    }

    pub fn make_ode(initial_state: (u32, u32, u32), production_rates: (f32, f32, f32), binding_rates: (f32, f32, f32), unbinding_rates: (f32, f32, f32), decay_rates: (f32, f32, f32), solver: ODESolver, max_time: f32) -> ODESimulation {
        let reactions = Self::reactions(production_rates, binding_rates, unbinding_rates, decay_rates);
        let initial_state = Self::initial_state(initial_state);

        ODESimulation::new(reactions, initial_state, solver, max_time).unwrap()
    }

    pub fn make_ssa(initial_state: (u32, u32, u32), production_rates: (f32, f32, f32), binding_rates: (f32, f32, f32), unbinding_rates: (f32, f32, f32), decay_rates: (f32, f32, f32), simulation_seed: u64) -> StochasticSimulation {
        let reactions = Self::reactions(production_rates, binding_rates, unbinding_rates, decay_rates);
        let initial_state = Self::initial_state(initial_state);

        StochasticSimulation::new(reactions, initial_state, simulation_seed)
    }

    pub fn species() -> ((Molecule, Molecule, Molecule), (Molecule, Molecule, Molecule), (Molecule, Molecule, Molecule)) {
        let g = (Molecule::new("g1"), Molecule::new("g2"), Molecule::new("g3"));
        let p = (Molecule::new("p1"), Molecule::new("p2"), Molecule::new("p3"));
        let gp = (Molecule::new("p1g2"), Molecule::new("p2g3"), Molecule::new("p3g1"));

        (g, p, gp)
    }

    fn initial_state(initial_state: (u32, u32, u32)) -> HashMap<Molecule, u32> {
        let ((g1, g2, g3), _, _) = Self::species();
        vec![
            (g1, initial_state.0),
            (g2, initial_state.1),
            (g3, initial_state.2)
        ].into_iter().collect()
    }
}