use crate::continuous_dynamical_systems::ODESolver;
use crate::chemical_reactions::prelude::*;
use crate::stochastic_simulation::prelude::*;

pub struct LotkaVolterra;

impl LotkaVolterra {
    fn reactions(prey_birth_rate: f32, predator_death_rate: f32, hunter_meetings: f32, hunt_offsprings: u32) -> Vec<Reaction> {
        let (prey, predator) = LotkaVolterra::species();

        vec![
            Reaction::new(prey_birth_rate, [(1, &prey)], [(2, &prey)]),
            Reaction::new(predator_death_rate, [(1, &predator)], []),
            Reaction::new(hunter_meetings, [(1, &prey), (1, &predator)], [(hunt_offsprings + 1, &predator)]),
        ]
    }

    pub fn make_ode(initial_preys: u32, initial_predators: u32, prey_birth_rate: f32, predator_death_rate: f32, hunter_meetings: f32, hunt_offsprings: u32, solver: ODESolver, max_time: f32) -> ODESimulation {
        let (prey, predator) = LotkaVolterra::species();
        let reactions = Self::reactions(prey_birth_rate, predator_death_rate, hunter_meetings, hunt_offsprings);
        let initial_state = vec![
            (prey, initial_preys),
            (predator, initial_predators)
        ].into_iter().collect();

        ODESimulation::new(reactions, initial_state, solver, max_time).unwrap()
    }

    pub fn make_ssa(initial_preys: u32, initial_predators: u32, prey_birth_rate: f32, predator_death_rate: f32, hunter_meetings: f32, hunt_offsprings: u32, simulation_seed: u64) -> StochasticSimulation {
        let (prey, predator) = LotkaVolterra::species();
        let reactions = Self::reactions(prey_birth_rate, predator_death_rate, hunter_meetings, hunt_offsprings);
        let initial_state = vec![
            (prey, initial_preys),
            (predator, initial_predators)
        ].into_iter().collect();

        StochasticSimulation::new(reactions, initial_state, simulation_seed)
    }

    pub fn species() -> (Molecule, Molecule) {
        let prey = Molecule::new("v");
        let predator = Molecule::new("p");

        (prey, predator)
    }
}