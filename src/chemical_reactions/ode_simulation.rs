use std::collections::{HashMap, HashSet};
use ode_solvers::*;
use crate::chemical_reactions::prelude::*;
use crate::continuous_dynamical_systems::ODESolver;

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

    pub fn num_species(&self) -> usize {
        self.species.len()
    }
}

impl ode_solvers::System<f32, ode_solvers::DVector<f32>> for ChemicalReactionODE {
    fn system(&self, _: f32, y: &ode_solvers::DVector<f32>, dy: &mut ode_solvers::DVector<f32>) {
        for reaction in self.reactions.iter() {
            reaction.apply_ode(&self.species, y, dy);
        }
    }
}

#[derive(Clone)]
pub struct ODESimulation {
    data: Vec<(f32, Vec<(Molecule, f32)>)>
}

impl ODESimulation {
    pub fn new(reactions: Vec<Reaction>, initial_state: HashMap<Molecule, u32>, solver: ODESolver, max_time: f32) -> Result<Self, String> {
        let step_size = 0.01f32;
        let ode = ChemicalReactionODE::new(reactions);
        let initial_state = Self::initial_state_to_vector(&ode, &initial_state);

        let data =
            match solver {
                ODESolver::DOP853 => {
                    let mut stepper = Dop853::new(ode.clone(), 0f32, max_time, step_size, initial_state, 1.0e-2, 1.0e-6);
                    stepper.integrate().map_err(|err| err.to_string())?;
                    stepper.results().to_owned()
                },
                ODESolver::DOPRI5 => {
                    let mut stepper = Dopri5::new(ode.clone(), 0f32, max_time, step_size, initial_state, 1.0e-2, 1.0e-6);
                    stepper.integrate().map_err(|err| err.to_string())?;
                    stepper.results().to_owned()
                },
                ODESolver::RK4 => {
                    let mut stepper = Rk4::new(ode.clone(), 0f32, initial_state, max_time, step_size);
                    stepper.integrate().map_err(|err| err.to_string())?;
                    stepper.results().to_owned()
                },
            };

        let (time, molecules) = data.get();
        let data = time.iter().cloned()
            .zip(
                molecules.iter().map(|molecules| molecules.iter().enumerate()
                    .map(|(id, quantity)| {
                        let molecule = ode.get_species_from_id(&id).cloned().unwrap();
                        (molecule, *quantity)
                    }).collect()
                )
            )
            .collect();

        Ok(Self { data })
    }

    fn initial_state_to_vector(ode: &ChemicalReactionODE, initial_state: &HashMap<Molecule, u32>) -> DVector<f32> {
        let mut state = DVector::from_element(ode.num_species(), 0f32);

        for (molecule, &quantity) in initial_state.iter() {
            state[ode.get_species_id(molecule).unwrap()] = quantity as f32;
        }

        state
    }
}

impl IntoIterator for ODESimulation {
    type Item = (f32, Vec<(Molecule, f32)>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}