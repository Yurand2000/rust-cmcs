use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

pub struct EnzymaticActivity;

impl EnzymaticActivity {
    fn reactions(binding_rate: f32, unbinding_rate: f32, catalysis_rate: f32) -> Vec<Reaction> {
        let enzyme = Molecule::new("e");
        let reactant = Molecule::new("s");
        let bound = Molecule::new("es");
        let product = Molecule::new("p");

        vec![
            Reaction::new(binding_rate, [(1, &enzyme), (1, &reactant)], [(1, &bound)]),
            Reaction::new(unbinding_rate, [(1, &bound)], [(1, &enzyme), (1, &reactant)]),
            Reaction::new(catalysis_rate, [(1, &bound)], [(1, &enzyme), (1, &product)]),
        ]
    }

    pub fn make_ode(initial_enzyme: u32, initial_reactant: u32, binding_rate: f32, unbinding_rate: f32, catalysis_rate: f32, solver: ODESolver, max_time: f32) -> ODESimulation {
        let reactions = Self::reactions(binding_rate, unbinding_rate, catalysis_rate);
        let initial_state = vec![
            (Molecule::new("e"), initial_enzyme),
            (Molecule::new("s"), initial_reactant)
        ].into_iter().collect();

        ODESimulation::new(reactions, initial_state, solver, max_time).unwrap()
    }

    pub fn make_ssa(initial_enzyme: u32, initial_reactant: u32, binding_rate: f32, unbinding_rate: f32, catalysis_rate: f32, simulation_seed: u64) -> StochasticSimulation {
        let reactions = Self::reactions(binding_rate, unbinding_rate, catalysis_rate);
        let initial_state = vec![
            (Molecule::new("e"), initial_enzyme),
            (Molecule::new("s"), initial_reactant)
        ].into_iter().collect();

        StochasticSimulation::new(reactions, initial_state, simulation_seed)
    }
}