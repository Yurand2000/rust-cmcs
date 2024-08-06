use std::collections::{HashMap, HashSet};

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

    pub fn get_propensity(&self, molecules: &HashMap<Molecule, u32>) -> f32 {
        let distinct_reactant_combinations: u32 =
            self.reactants.iter()
            .map(|(stochiometric_coeff, molecule)| {
                let molecule_amount = molecules.get(molecule).cloned().unwrap_or(0u32);
                num::integer::binomial(molecule_amount, *stochiometric_coeff)
            })
            .product();

        self.kinetic_constant * (distinct_reactant_combinations as f32)
    }

    pub fn apply_ode(&self, species: &HashMap<Molecule, usize>, y: &ode_solvers::DVector<f32>, dy: &mut ode_solvers::DVector<f32>) {
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

    pub fn apply_ssa(&self, molecules: &mut HashMap<Molecule, u32>) {
        for (stochiometric_coeff, molecule) in self.reactants.iter() {
            let molecule_quantity = molecules.get_mut(molecule).unwrap();
            *molecule_quantity = *molecule_quantity - stochiometric_coeff;
        }

        for (stochiometric_coeff, molecule) in self.products.iter() {
            let molecule_quantity = molecules.get_mut(molecule).unwrap();
            *molecule_quantity = *molecule_quantity + stochiometric_coeff;
        }
    }

    pub fn get_species(&self) -> HashSet<&Molecule> {
        let reactants = self.reactants.iter().map(|(_, molecule)| molecule);
        let products = self.products.iter().map(|(_, molecule)| molecule);
        reactants.chain(products).collect()
    }
}
