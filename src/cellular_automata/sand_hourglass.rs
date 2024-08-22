use std::{f32::consts::E, fmt::format};

use rand::{Rng, SeedableRng};

use crate::cellular_automata::prelude::{*, block_automaton::*};

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Cell {
    Empty,
    Sand,
    Wall,
}

pub struct Boundary;

impl ToCell<Cell> for Boundary {
    fn to_cell() -> Cell {
        Cell::Empty
    }
}

pub type SandLattice = Lattice<Cell>;

#[derive(Clone)]
pub struct GlobalState {
    rng: rand::rngs::SmallRng,
}

#[derive(Clone)]
pub struct SandHourglassModel(BlockAutomaton<Cell, FixedBoundary<Cell, Boundary>, [Cell; 4], GlobalState>);

impl SandHourglassModel
{
    pub fn from_str(map_str: &str, simulation_seed: u64, friction_probability: f64) -> Result<Self, String> {
        use Cell::*;

        let size_x = map_str.lines().next().unwrap().chars().count() as u32;
        let size_y = map_str.lines().count() as u32;

        if map_str.lines().map(|line| line.chars().count()).any(|len| len != size_x as usize) {
            return Err(format!("Error in parsing the map string"));
        }

        let mut hourglass = Lattice::fill(size_x, size_y, Cell::Wall);
        for (y, line) in map_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let cell =
                    match ch {
                        '#' => Wall,
                        ' ' => Empty,
                        '*' => Sand,
                        _ => Empty,
                    };

                hourglass.set(x as u32, y as u32, cell);
            }
        }

        let global_state = GlobalState {
            rng: rand::rngs::SmallRng::seed_from_u64(simulation_seed)
        };

        let automaton = BlockAutomaton::new(hourglass, global_state, move |neighbors, state| {
            Self::automaton(neighbors, state, friction_probability)
        })?;

        Ok(Self(automaton))
    }

    fn automaton(neighborhood: &[Cell; 4], global_state: &mut GlobalState, friction_probability: f64) -> [Cell; 4] {
        use Cell::*;

        let (tl, tr, bl, br) = (&neighborhood[0], &neighborhood[1], &neighborhood[2], &neighborhood[3]);

        match (tl, tr, bl, br) {
            (Empty, Sand, Empty, Empty) => [Empty, Empty, Empty, Sand],
            (Empty, Sand, Empty, Sand) => [Empty, Empty, Sand, Sand],
            (Empty, Sand, Empty, Wall) => [Empty, Empty, Sand, Wall],
            (Empty, Sand, Sand, Empty) => [Empty, Empty, Sand, Sand],
            (Empty, Sand, Wall, Empty) => [Empty, Empty, Wall, Sand],
            (Sand, Empty, Empty, Empty) => [Empty, Empty, Sand, Empty],
            (Sand, Empty, Empty, Sand) => [Empty, Empty, Sand, Sand],
            (Sand, Empty, Empty, Wall) => [Empty, Empty, Sand, Wall],
            (Sand, Empty, Sand, Empty) => [Empty, Empty, Sand, Sand],
            (Sand, Empty, Wall, Empty) => [Empty, Empty, Wall, Sand],
            (Sand, Sand, Empty, Empty) => {
                let distribution = rand_distr::Bernoulli::new(friction_probability).unwrap();
                if global_state.rng.sample(distribution) {
                    [Sand, Sand, Empty, Empty]
                } else {
                    [Empty, Empty, Sand, Sand]
                }
            },
            (Sand, Sand, Empty, Sand) => [Empty, Sand, Sand, Sand],
            (Sand, Sand, Empty, Wall) => [Empty, Sand, Sand, Wall],
            (Sand, Sand, Sand, Empty) => [Sand, Empty, Sand, Sand],
            (Sand, Sand, Wall, Empty) => [Sand, Empty, Wall, Sand],
            (Sand, Wall, Empty, Empty) => [Empty, Wall, Sand, Empty],
            (Sand, Wall, Empty, Sand) => [Empty, Wall, Sand, Sand],
            (Sand, Wall, Empty, Wall) => [Empty, Wall, Sand, Wall],
            (Sand, Wall, Sand, Empty) => [Sand, Wall, Sand, Empty], //identity
            (Sand, Wall, Wall, Empty) => [Sand, Wall, Wall, Empty], //identity
            (Wall, Sand, Empty, Empty) => [Wall, Empty, Empty, Sand],
            (Wall, Sand, Empty, Sand) => [Wall, Sand, Empty, Sand], //identity
            (Wall, Sand, Empty, Wall) => [Wall, Sand, Empty, Wall], //identity
            (Wall, Sand, Sand, Empty) => [Wall, Empty, Sand, Sand],
            (Wall, Sand, Wall, Empty) => [Wall, Empty, Wall, Sand],
            _ => [tl.clone(), tr.clone(), bl.clone(), br.clone()]
        }
    }
}

impl Iterator for SandHourglassModel
{
    type Item = Result<SandLattice, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub mod maps;