use crate::cellular_automata::prelude::automaton_2d::*;

pub struct Boundary;

impl ToCell<bool> for Boundary {
    fn to_cell() -> bool {
        false
    }
}

pub type BoundaryFixed = FixedBoundary<bool, Boundary>;
pub type BoundaryPeriodic = PeriodicBoundary;

pub type State = Lattice<bool>;
pub struct GameOfLife<B>(Automaton2D<bool, MooreNeighborhood, B, [bool; 9], ()>);

impl<B> Clone for GameOfLife<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<B> GameOfLife<B> {
    pub fn from_string(state_str: &str) -> Option<Self>
        where B: 'static
    {
        let size_x = state_str.lines().next().unwrap().chars().count() as u32;
        let size_y = state_str.lines().count() as u32;

        if state_str.lines().map(|line| line.chars().count()).any(|len| len != size_x as usize) {
            return None;
        }

        let mut state = Lattice::fill(size_x, size_y, false);
        for (y, line) in state_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    state.set(x as u32, y as u32, true);
                }
            }
        }

        let automaton = Automaton2D::new(state, (), Self::automaton);

        Some(Self(automaton))
    }

    fn automaton(neighborhood: &[bool; 9], _: &mut ()) -> bool {
        let cell = neighborhood[4];

        let neighbors: u32 = neighborhood[..4].iter().chain(neighborhood[5..].iter())
            .map(|n| *n as u32).sum();

        if cell {
            if neighbors < 2 {
                false
            } else if neighbors > 3 {
                false
            } else {
                true
            }
        } else {
            if neighbors == 3 {
                true
            } else {
                false
            }
        }
    }
}

impl Iterator for GameOfLife<BoundaryFixed>
{
    type Item = Result<State, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for GameOfLife<BoundaryPeriodic>
{
    type Item = Result<State, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub mod states;