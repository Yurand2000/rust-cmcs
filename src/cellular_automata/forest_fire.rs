use rand::{Rng, SeedableRng};

use crate::cellular_automata::prelude::{*, automaton_2d::*};

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Cell {
    GreenTree,
    BurningTree,
    Empty
}

pub struct Boundary;

impl ToCell<Cell> for Boundary {
    fn to_cell() -> Cell {
        Cell::Empty
    }
}

pub type ForestLattice = Lattice<Cell>;

#[derive(Clone)]
pub struct GlobalState {
    rng: rand::rngs::SmallRng,
}

#[derive(Clone)]
pub struct ForestFireModel(Automaton2D<Cell, MooreNeighborhood, FixedBoundary<Cell, Boundary>, [Cell; 9], GlobalState>);

impl ForestFireModel
{
    pub fn new(size: u32, simulation_seed: u64, lightning_probability: f64, growth_probability: f64) -> Result<Self, String> {
        use Cell::*;

        let mut rng = rand::rngs::SmallRng::seed_from_u64(simulation_seed);
        let distribution = rand_distr::Bernoulli::new(0.5).unwrap();
        let forest = ForestLattice::from_fn(size, size, |_, _| {
            if rng.sample(distribution) {
                Ok(GreenTree)
            } else {
                Ok(Empty)
            }
        })?;

        let global_state = GlobalState {
            rng: rand::rngs::SmallRng::seed_from_u64(simulation_seed)
        };

        let automaton = Automaton2D::new(forest, global_state, move |neighborhood, global_state| {
            Self::automaton(neighborhood, lightning_probability, growth_probability, global_state)
        });

        Ok(Self(automaton))
    }

    fn automaton(neighborhood: &[Cell; 9], lightning_probability: f64, growth_probability: f64, global_state: &mut GlobalState) -> Cell {
        use Cell::*;

        let cell = &neighborhood[4];
        let rng = &mut global_state.rng;
        let mut neighbors = neighborhood[0..4].iter().chain(neighborhood[5..].iter());

        match cell {
            GreenTree => {
                if neighbors.any(|n| n == &BurningTree) {
                    BurningTree
                } else {
                    let distribution = rand_distr::Bernoulli::new(lightning_probability).unwrap();
                    if rng.sample(distribution) {
                        BurningTree
                    } else {
                        GreenTree
                    }
                }
            },
            BurningTree => Empty,
            Empty => {
                let distribution = rand_distr::Bernoulli::new(growth_probability).unwrap();
                if rng.sample(distribution) {
                    GreenTree
                } else {
                    Empty
                }
            },
        }
    }
}

impl Iterator for ForestFireModel
{
    type Item = Result<ForestLattice, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}