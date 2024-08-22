use std::{marker::PhantomData, rc::Rc};
use crate::cellular_automata::prelude::*;

struct AutomatonMachine2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    automaton: Rc<dyn Fn(&NN, &mut S) -> C>,
    _phantom: PhantomData<(C, N, B, S)>,
}

impl<C, B, NN, S> AutomatonMachine2D<C, VonNeumannNeighborhood, FixedBoundary<C, B>, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32) -> Result<[C; 5], String> {
        if x >= lattice.size.0 || y >= lattice.size.1 {
            return Err(format!("Get neighbors error: {}/{}, {}/{}", x, lattice.size.0, y, lattice.size.1));
        }

        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        let top = if y == 0 { B::to_cell() } else { lattice.get_result(x, y - 1)? };
        let bottom = if y == max_y { B::to_cell() } else { lattice.get_result(x, y + 1)? };
        let center = lattice.get_result(x, y)?;
        let left = if x == 0 { B::to_cell() } else { lattice.get_result(x - 1, y)? };
        let right = if x == max_x { B::to_cell() } else { lattice.get_result(x + 1, y)? };

        Ok([top, right, center, bottom, left])
    }
}

impl<C, NN, S> AutomatonMachine2D<C, VonNeumannNeighborhood, PeriodicBoundary, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32) -> Result<[C; 5], String> {
        if x >= lattice.size.0 || y >= lattice.size.1 {
            return Err(format!("Get neighbors error: {}/{}, {}/{}", x, lattice.size.0, y, lattice.size.1));
        }

        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        let top = if y == 0 { lattice.get_result(x, max_y)? } else { lattice.get_result(x, y - 1)? };
        let bottom = if y == max_y { lattice.get_result(x, 0)? } else { lattice.get_result(x, y + 1)? };
        let center = lattice.get_result(x, y)?;
        let left = if x == 0 { lattice.get_result(max_x, y)? } else { lattice.get_result(x - 1, y)? };
        let right = if x == max_x { lattice.get_result(0, y)? } else { lattice.get_result(x + 1, y)? };

        Ok([top, right, center, bottom, left])
    }
}

impl<C, B, NN, S> AutomatonMachine2D<C, MooreNeighborhood, FixedBoundary<C, B>, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32) -> Result<[C; 9], String> {
        if x >= lattice.size.0 || y >= lattice.size.1 {
            return Err(format!("Get neighbors error: {}/{}, {}/{}", x, lattice.size.0, y, lattice.size.1))
        }

        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        let top_left = match (x, y) {
            (0, 0) => B::to_cell(),
            (0, _) => B::to_cell(),
            (_, 0) => B::to_cell(),
            (_, _) => lattice.get_result(x - 1, y - 1)?
        };
        let top = if y == 0 { B::to_cell() } else { lattice.get_result(x, y - 1)? };
        let top_right = match (x, y) {
            (x, 0) if x == max_x => B::to_cell(),
            (x, _) if x == max_x => B::to_cell(),
            (_, 0) => B::to_cell(),
            (_, _) => lattice.get_result(x + 1, y - 1)?
        };
        
        let left = if x == 0 { B::to_cell() } else { lattice.get_result(x - 1, y)? };
        let center = lattice.get_result(x, y)?;
        let right = if x == max_x { B::to_cell() } else { lattice.get_result(x + 1, y)? };

        let bottom_left = match (x, y) {
            (0, y) if y == max_y => B::to_cell(),
            (0, _) => B::to_cell(),
            (_, y) if y == max_y => B::to_cell(),
            (_, _) => lattice.get_result(x - 1, y + 1)?
        };
        let bottom = if y == max_y { B::to_cell() } else { lattice.get_result(x, y + 1)? };
        let bottom_right = match (x, y) {
            (x, y) if x == max_x && y == max_y => B::to_cell(),
            (x, _) if x == max_x => B::to_cell(),
            (_, y) if y == max_y => B::to_cell(),
            (_, _) => lattice.get_result(x + 1, y + 1)?
        };

        Ok([top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right])
    }
}

impl<C, NN, S> AutomatonMachine2D<C, MooreNeighborhood, PeriodicBoundary, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32) -> Result<[C; 9], String> {
        if x >= lattice.size.0 || y >= lattice.size.1 {
            return Err(format!("Get neighbors error: {}/{}, {}/{}", x, lattice.size.0, y, lattice.size.1));
        }

        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        let top_left = match (x, y) {
            (0, 0) => lattice.get_result(max_x, max_y)?,
            (0, _) => lattice.get_result(max_x, y - 1)?,
            (_, 0) => lattice.get_result(x - 1, max_y)?,
            (_, _) => lattice.get_result(x - 1, y - 1)?
        };
        let top = if y == 0 { lattice.get_result(x, max_y)? } else { lattice.get_result(x, y - 1)? };
        let top_right = match (x, y) {
            (x, 0) if x == max_x => lattice.get_result(0, max_y)?,
            (x, _) if x == max_x => lattice.get_result(0, y - 1)?,
            (_, 0) => lattice.get_result(x + 1, max_y)?,
            (_, _) => lattice.get_result(x + 1, y - 1)?
        };
        
        let left = if x == 0 { lattice.get_result(max_x, y)? } else { lattice.get_result(x - 1, y)? };
        let center = lattice.get_result(x, y)?;
        let right = if x == max_x { lattice.get_result(0, y)? } else { lattice.get_result(x + 1, y)? };

        let bottom_left = match (x, y) {
            (0, y) if y == max_y => lattice.get_result(max_x, 0)?,
            (0, _) => lattice.get_result(max_x, y + 1)?,
            (_, y) if y == max_y => lattice.get_result(x - 1, 0)?,
            (_, _) => lattice.get_result(x - 1, y + 1)?
        };
        let bottom = if y == max_y { lattice.get_result(x, 0)? } else { lattice.get_result(x, y + 1)? };
        let bottom_right = match (x, y) {
            (x, y) if x == max_x && y == max_y => lattice.get_result(0, 0)?,
            (x, _) if x == max_x => lattice.get_result(0, y + 1)?,
            (_, y) if y == max_y => lattice.get_result(x + 1, 0)?,
            (_, _) => lattice.get_result(x + 1, y + 1)?
        };
        
        Ok([top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right])
    }
}

impl<C, N, B, NN, S> Clone for AutomatonMachine2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn clone(&self) -> Self {
        Self { automaton: self.automaton.clone(), _phantom: self._phantom }
    }
}

impl<C, S, B> AutomatonMachine<C, S> for AutomatonMachine2D<C, VonNeumannNeighborhood, FixedBoundary<C, B>, [C; 5], S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn step(&self, lattice: Lattice<C>, state: &mut S) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(lattice.size.0, lattice.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&lattice, x, y)?;
            Ok(automaton(&neighborhood, state))
        })
    }
}

impl<C, S> AutomatonMachine<C, S> for AutomatonMachine2D<C, VonNeumannNeighborhood, PeriodicBoundary, [C; 5], S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn step(&self, lattice: Lattice<C>, state: &mut S) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(lattice.size.0, lattice.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&lattice, x, y)?;
            Ok(automaton(&neighborhood, state))
        })
    }
}

impl<C, S, B> AutomatonMachine<C, S> for AutomatonMachine2D<C, MooreNeighborhood, FixedBoundary<C, B>, [C; 9], S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn step(&self, lattice: Lattice<C>, state: &mut S) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(lattice.size.0, lattice.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&lattice, x, y)?;
            Ok(automaton(&neighborhood, state))
        })
    }
}

impl<C, S> AutomatonMachine<C, S> for AutomatonMachine2D<C, MooreNeighborhood, PeriodicBoundary, [C; 9], S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn step(&self, lattice: Lattice<C>, state: &mut S) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(lattice.size.0, lattice.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&lattice, x, y)?;
            Ok(automaton(&neighborhood, state))
        })
    }
}

pub struct Automaton2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    lattice: Lattice<C>,
    automaton: AutomatonMachine2D<C, N, B, NN, S>,
    global_state: S,

    state: Option<Lattice<C>>,
    error: bool,
}

impl<C, N, B, NN, S> Clone for  Automaton2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn clone(&self) -> Self {
        Self { lattice: self.lattice.clone(), automaton: self.automaton.clone(), global_state: self.global_state.clone(), state: self.state.clone(), error: self.error }
    }
}

impl<C, N, B, NN, S> Automaton2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    pub fn new(lattice: Lattice<C>, global_state: S, automaton: impl for<'a, 'b> Fn(&'a NN, &'b mut S) -> C + 'static) -> Self {
        let automaton = AutomatonMachine2D {
            automaton: Rc::new(automaton),
            _phantom: PhantomData,
        };

        Self { lattice, automaton, global_state, state: None, error: false }
    }
}

impl<C, N, B, NN, S> Iterator for Automaton2D<C, N, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone, AutomatonMachine2D<C, N, B, NN, S>: AutomatonMachine<C, S>
{
    type Item = Result<Lattice<C>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        let state = std::mem::take(&mut self.state);
        match state {
            Some(state) => {
                match self.automaton.step(state, &mut self.global_state) {
                    Ok(new_state) => {
                        self.state = Some(new_state);
                        self.state.clone().map(|state| Ok(state))
                    },
                    Err(err) => {
                        self.error = true;
                        Some(Err(err))
                    },
                }
            },
            None => {
                self.state = Some(self.lattice.clone());
                self.state.clone().map(|state| Ok(state))
            },
        }
    }
}