use std::collections::{HashMap, HashSet};
use rand::{Rng, SeedableRng};
use crate::chemical_reactions::prelude::*;

#[derive(Clone)]
pub struct StochasticSimulation {
    reactions: Vec<Reaction>,
    initial_state: HashMap<Molecule, u32>,
    rng: rand::rngs::SmallRng,

    state: Option<(f32, HashMap<Molecule, u32>)>,
}

impl StochasticSimulation {
    pub fn new(reactions: Vec<Reaction>, mut initial_state: HashMap<Molecule, u32>, seed: u64) -> Self {
        let molecular_species: HashSet<&Molecule> = reactions.iter()
            .flat_map(|reaction| reaction.get_species())
            .collect();

        for species in molecular_species.into_iter() {
            if !initial_state.contains_key(species) {
                initial_state.insert(species.clone(), 0);
            }
        }

        Self {
            reactions,
            initial_state,
            rng: rand::rngs::SmallRng::seed_from_u64(seed),
            state: None,
        }
    }
}

impl Iterator for StochasticSimulation {
    type Item = (f32, Vec<(Molecule, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::replace(&mut self.state, None);
        match state {
            Some((time, state)) => {
                // compute propensities
                let propensities: Vec<f32> = self.reactions.iter()
                    .map(|reaction| reaction.get_propensity(&state)).collect();

                let propensities_sum: f32 = propensities.iter().sum();

                // compute delta time
                let distribution = rand_distr::Exp::new(propensities_sum).unwrap();
                let delta_time = self.rng.sample(distribution);

                // choose reaction to apply
                let distibution = rand::distributions::Uniform::new(0f32, propensities_sum);
                let chosen_reaction_value = self.rng.sample(distibution);

                let reaction_probabilities =
                    propensities.into_iter()
                    .scan(0f32, |partial_sum, probability| {
                        *partial_sum = *partial_sum + probability;
                        Some(*partial_sum)
                    });

                let chosen_reaction = reaction_probabilities
                    .zip(self.reactions.iter())
                    .find_map(|(value, reaction)| if value > chosen_reaction_value { Some(reaction) } else { None })
                    .unwrap();
                
                // apply reaction
                let mut new_state = state;

                chosen_reaction.apply_ssa(&mut new_state);
                self.state = Some((time + delta_time, new_state.clone()));
                Some((time + delta_time, new_state.into_iter().collect()))
            },
            None => {
                self.state = Some((0f32, self.initial_state.clone()));
                Some((0f32, self.initial_state.iter().map(|(molecule, amount)| (molecule.clone(), *amount)).collect()))
            },
        }
    }
}