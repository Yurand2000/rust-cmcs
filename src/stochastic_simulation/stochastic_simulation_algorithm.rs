use std::collections::{HashMap, HashSet};
use ode_solvers::*;
use rand::{Rng, SeedableRng};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Molecule(String);

impl Molecule {
    pub fn new(str: &'static str) -> Self {
        Self(str.to_owned())
    }
}

#[derive(Clone)]
pub struct Reaction {
    reactants: HashSet<(u32, Molecule)>,
    products: HashSet<(u32, Molecule)>,
    kinetic_constant: f32
}

impl Reaction {
    pub fn new<const C1: usize, const C2: usize>(kinetic_constant: f32, reactants: [(u32, &Molecule); C1], products: [(u32, &Molecule); C2]) -> Self {
        Self {
            reactants: reactants.iter().map(|(k, mol)| (*k, (*mol).clone())).collect(),
            products: products.iter().map(|(k, mol)| (*k, (*mol).clone())).collect(),
            kinetic_constant,
        }
    }

    fn get_propensity(&self, molecules: &HashMap<Molecule, u32>) -> f32 {
        let distinct_reactant_combinations: u32 =
            self.reactants.iter()
            .map(|(stochiometric_coeff, molecule)| {
                let molecule_amount = molecules.get(molecule).cloned().unwrap_or(0u32);
                num::integer::binomial(molecule_amount, *stochiometric_coeff)
            })
            .product();

        self.kinetic_constant * (distinct_reactant_combinations as f32)
    }

    fn apply_ode(&self, species: &HashMap<Molecule, usize>, y: &ode_solvers::DVector<f32>, dy: &mut ode_solvers::DVector<f32>) {
        let reactants: f32 = self.reactants.iter()
            .map(|(stochiometric_coeff, reactant)| f32::powi(y[species[reactant]], *stochiometric_coeff as i32))
            .product();

        for (stochiometric_coeff, reactant) in self.reactants.iter() {
            dy[species[reactant]] -= (*stochiometric_coeff as f32) * self.kinetic_constant * reactants;
        }
        
        for (stochiometric_coeff, product) in self.products.iter() {
            dy[species[product]] += (*stochiometric_coeff as f32) * self.kinetic_constant * reactants;
        }
    }

    fn apply_ssa(&self, molecules: &mut HashMap<Molecule, u32>) {
        for (stochiometric_coeff, molecule) in self.reactants.iter() {
            let molecule_quantity = molecules.get_mut(molecule).unwrap();
            *molecule_quantity = *molecule_quantity - stochiometric_coeff;
        }

        for (stochiometric_coeff, molecule) in self.products.iter() {
            let molecule_quantity = molecules.get_mut(molecule).unwrap();
            *molecule_quantity = *molecule_quantity + stochiometric_coeff;
        }
    }

    fn get_species(&self) -> HashSet<&Molecule> {
        let reactants = self.reactants.iter().map(|(_, molecule)| molecule);
        let products = self.products.iter().map(|(_, molecule)| molecule);
        reactants.chain(products).collect()
    }
}

#[derive(Clone)]
pub struct ChemicalReactionODE {
    reactions: Vec<Reaction>,
    species: HashMap<Molecule, usize>,
    inv_species: HashMap<usize, Molecule>,
}

impl ChemicalReactionODE {
    pub fn new(reactions: Vec<Reaction>) -> Self {
        let molecular_species: HashSet<&Molecule> =
            reactions.iter()
            .flat_map(|reaction| reaction.get_species())
            .collect();

        let species_iter: Vec<_> = molecular_species.into_iter()
            .enumerate()
            .map(|(id, mol)| (mol.clone(), id))
            .collect();

        let inv_species = species_iter.iter().cloned()
            .map(|(a, b)| (b, a)).collect();
        let species = species_iter.into_iter().collect();

        Self { reactions, species, inv_species }
    }

    pub fn get_species_id(&self, molecule: &Molecule) -> Option<usize> {
        self.species.get(molecule).cloned()
    }

    pub fn get_species_from_id(&self, molecule_id: &usize) -> Option<&Molecule> {
        self.inv_species.get(molecule_id)
    }
}

impl ode_solvers::System<f32, ode_solvers::DVector<f32>> for ChemicalReactionODE {
    fn system(&self, _: f32, y: &ode_solvers::DVector<f32>, dy: &mut ode_solvers::DVector<f32>) {
        for reaction in self.reactions.iter() {
            reaction.apply_ode(&self.species, y, dy);
        }
    }
}

use nalgebra::*;

#[derive(Clone)]
pub struct ODESimulation {
    data: Vec<(f32, Vec<(Molecule, f32)>)>
}

impl ODESimulation {
    pub fn new(reactions: Vec<Reaction>, mut initial_state: HashMap<Molecule, u32>, max_time: f32) -> Result<Self, String> {
        let step_size = 0.01f32;
        let model = ChemicalReactionODE::new(reactions);
        let mut stepper = Dop853::new(model.clone(), 0f32, max_time, step_size, DVector::from_element(5, 0f32), 1.0e-2, 1.0e-6);
        stepper.integrate().map_err(|err| err.to_string())?;

        let data = stepper.results().to_owned();
        let (time, molecules) = data.get();
        let data = time.iter().cloned()
            .zip(
                molecules.iter().map(|molecules| molecules.iter().enumerate()
                    .map(|(id, quantity)| {
                        let molecule = model.get_species_from_id(&id).cloned().unwrap();
                        (molecule, *quantity)
                    }).collect()
                )
            )
            .collect();

        Ok(Self { data })
    }
}

impl Iterator for ODESimulation {
    type Item = (f32, Vec<(Molecule, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

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