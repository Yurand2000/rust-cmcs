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

    pub fn make_ode(initial_enzyme: f32, initial_reactant: f32, binding_rate: f32, unbinding_rate: f32, catalysis_rate: f32) -> EnzymaticActivityODE {
        let enzyme = Molecule::new("e");
        let reactant = Molecule::new("s");
        let reactions = Self::reactions(binding_rate, unbinding_rate, catalysis_rate);
        let ode = ChemicalReactionODE::new(reactions);

        let mut initial_state = ode_solvers::DVector::from_element(4, 0f32);
        initial_state[ode.get_species_id(&enzyme).unwrap()] = initial_enzyme;
        initial_state[ode.get_species_id(&reactant).unwrap()] = initial_reactant;

        EnzymaticActivityODE {
            ode,
            initial_state,
        }
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

pub struct EnzymaticActivityODE {
    ode: ChemicalReactionODE,
    initial_state: ode_solvers::DVector<f32>,
}   