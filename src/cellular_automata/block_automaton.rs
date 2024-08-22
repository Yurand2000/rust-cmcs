use std::{marker::PhantomData, rc::Rc};
use crate::cellular_automata::prelude::*;

#[derive(Clone)]
struct BlockAutomatonState {
    step: u8
}

struct BlockAutomatonMachine<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    automaton: Rc<dyn Fn(&NN, &mut S) -> [C; 4]>,
    _phantom: PhantomData<(C, B, S)>,
}

impl<C, B, NN, S> Clone for BlockAutomatonMachine<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn clone(&self) -> Self {
        Self { automaton: self.automaton.clone(), _phantom: self._phantom }
    }
}

impl<C, S, B> BlockAutomatonMachine<C, FixedBoundary<C, B>, [C; 4], S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32, step: u8) -> Result<[C; 4], String> {
        let (size_x, size_y) = (lattice.size.0, lattice.size.1);

        match step {
            0 => {  // top left
                if x == size_x || y == size_y {
                    Ok([B::to_cell(), B::to_cell(), B::to_cell(), B::to_cell()])
                } else {
                    let tl = lattice.get_result(x, y)?;
                    let tr = lattice.get_result(x + 1, y)?;
                    let bl = lattice.get_result(x, y + 1)?;
                    let br = lattice.get_result(x + 1, y + 1)?;
                    Ok([tl, tr, bl, br])
                }
            },
            1 => {  // bottom right
                let tl = if x == 0 || y == 0 { B::to_cell() } else { lattice.get_result(x - 1, y - 1)? };
                let tr = if x == size_x || y == 0 { B::to_cell() } else { lattice.get_result(x, y - 1)? };
                let bl = if x == 0 || y == size_y { B::to_cell() } else { lattice.get_result(x - 1, y)? };
                let br = if x == size_x || y == size_y { B::to_cell() } else { lattice.get_result(x, y)? };
                Ok([tl, tr, bl, br])
            },
            2 => {  // bottom left
                let tl = if x == size_x || y == 0 { B::to_cell() } else { lattice.get_result(x, y - 1)? };
                let tr = if x == size_x || y == 0 { B::to_cell() } else { lattice.get_result(x + 1, y - 1)? };
                let bl = if x == size_x || y == size_y { B::to_cell() } else { lattice.get_result(x, y)? };
                let br = if x == size_x || y == size_y { B::to_cell() } else { lattice.get_result(x + 1, y)? };
                Ok([tl, tr, bl, br])
            },
            3 => {  // top right
                let tl = if x == 0 || y == size_y { B::to_cell() } else { lattice.get_result(x - 1, y)? };
                let tr = if x == size_x || y == size_y { B::to_cell() } else { lattice.get_result(x, y)? };
                let bl = if x == 0 || y == size_y { B::to_cell() } else { lattice.get_result(x - 1, y + 1)? };
                let br = if x == size_x || y == size_y { B::to_cell() } else { lattice.get_result(x, y + 1)? };
                Ok([tl, tr, bl, br])
            },
            _ => panic!()
        }
    }
}

impl<C, S> BlockAutomatonMachine<C, PeriodicBoundary, [C; 4], S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn get_neighbors(lattice: &Lattice<C>, x: u32, y: u32, step: u8) -> Result<[C; 4], String> {
        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        match step {
            0 => {  // top left
                let tl = lattice.get_result(x, y)?;
                let tr = if x == max_x { lattice.get_result(0, y)? } else { lattice.get_result(x + 1, y)? };
                let bl = if y == max_y { lattice.get_result(x, 0)? } else { lattice.get_result(x, y + 1)? };
                let br = if x == max_x && y == max_y { lattice.get_result(0, 0)? } else { lattice.get_result(x + 1, y + 1)? };
                Ok([tl, tr, bl, br])
            },
            1 => {  // bottom right
                let tl = if x == 0 && y == 0 { lattice.get_result(max_x, max_y)? } else { lattice.get_result(x - 1, y - 1)? };
                let tr = if y == 0 { lattice.get_result(x, max_y)? } else { lattice.get_result(x, y - 1)? };
                let bl = if x == 0 { lattice.get_result(max_x, y)? } else { lattice.get_result(x - 1, y)? };
                let br = lattice.get_result(x, y)?;
                Ok([tl, tr, bl, br])
            },
            2 => {  // bottom left
                let tl = if y == 0 { lattice.get_result(x, max_y)? } else { lattice.get_result(x, y - 1)? };
                let tr = if y == 0 { lattice.get_result(x + 1, max_y)? } else { lattice.get_result(x + 1, y - 1)? };
                let bl = lattice.get_result(x, y)?;
                let br = lattice.get_result(x + 1, y)?;
                Ok([tl, tr, bl, br])
            },
            3 => {  // top right
                let tl = if y == 0 { lattice.get_result(max_x, y)? } else { lattice.get_result(x - 1, y)? };
                let tr = lattice.get_result(x, y)?;
                let bl = if y == 0 { lattice.get_result(max_x, y + 1)? } else { lattice.get_result(x - 1, y + 1)? };
                let br = lattice.get_result(x, y + 1)?;
                Ok([tl, tr, bl, br])
            },
            _ => panic!()
        }
    }
}

impl<C, S, B> AutomatonMachine<C, (BlockAutomatonState, S)> for BlockAutomatonMachine<C, FixedBoundary<C, B>, [C; 4], S>
    where C: Clone + PartialEq + Eq, S: Clone, B: ToCell<C>
{
    fn step(&self, lattice: Lattice<C>, state: &mut (BlockAutomatonState, S)) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        let step = state.0.step;
        state.0.step = (state.0.step + 1) % 4;

        let mut new_lattice = lattice.clone();
        let (size_x, size_y) = (lattice.size.0, lattice.size.1);
        
        for x in (0..=lattice.size.0).step_by(2) {
            for y in (0..=lattice.size.1).step_by(2) {
                let neighborhood = Self::get_neighbors(&lattice, x, y, step)?;
                let (tl, tr, bl, br) = automaton(&neighborhood, &mut state.1).into();

                match step {
                    0 => {  // top left
                        if x != size_x && y != size_y { new_lattice.set(x, y, tl); };
                        if x != size_x && y != size_y { new_lattice.set(x + 1, y, tr); };
                        if x != size_x && y != size_y { new_lattice.set(x, y + 1, bl); };
                        if x != size_x && y != size_y { new_lattice.set(x + 1, y + 1, br); };
                    },
                    1 => {  // bottom right
                        if x != 0 && y != 0 { new_lattice.set(x - 1, y - 1, tl); };
                        if x != size_x && y != 0 { new_lattice.set(x, y - 1, tr); };
                        if x != 0 && y != size_y { new_lattice.set(x - 1, y, bl); };
                        if x != size_x && y != size_y { new_lattice.set(x, y, br); };
                    },
                    2 => {  // bottom left
                        if x != size_x && y != 0 { new_lattice.set(x, y - 1, tl); };
                        if x != size_x && y != 0 { new_lattice.set(x + 1, y - 1, tr); };
                        if x != size_x && y != size_y { new_lattice.set(x, y, bl); };
                        if x != size_x && y != size_y { new_lattice.set(x + 1, y, br); };
                    },
                    3 => {  // top right
                        if x != 0 && y != size_y { new_lattice.set(x - 1, y, tl); };
                        if x != size_x && y != size_y { new_lattice.set(x, y, tr); };
                        if x != 0 && y != size_y { new_lattice.set(x - 1, y + 1, bl); };
                        if x != size_x && y != size_y { new_lattice.set(x, y + 1, br); };
                    },
                    _ => panic!(),
                };
            }
        }

        Ok(new_lattice)
    }
}

impl<C, S> AutomatonMachine<C, (BlockAutomatonState, S)> for BlockAutomatonMachine<C, PeriodicBoundary, [C; 4], S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn step(&self, lattice: Lattice<C>, state: &mut (BlockAutomatonState, S)) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        let step = state.0.step;
        state.0.step = (state.0.step + 1) % 4;

        let mut new_lattice = lattice.clone();
        let (max_x, max_y) = (lattice.size.0 - 1, lattice.size.1 - 1);

        for x in (0..lattice.size.0).step_by(2) {
            for y in (0..lattice.size.1).step_by(2) {
                let neighborhood = Self::get_neighbors(&lattice, x, y, step)?;
                let (tl, tr, bl, br) = automaton(&neighborhood, &mut state.1).into();

                match step {
                    0 => {  // top left
                        new_lattice.set(x, y, tl);
                        new_lattice.set(x + 1, y, tr);
                        new_lattice.set(x, y + 1, bl);
                        new_lattice.set(x + 1, y + 1, br);
                    },
                    1 => {  // bottom right
                        if x != 0 && y != 0 { new_lattice.set(x - 1, y - 1, tl); } else { new_lattice.set(max_x, max_y, tl); };
                        if y != 0 { new_lattice.set(x, y - 1, tr); } else { new_lattice.set(x, max_y, tr); };
                        if x != 0 { new_lattice.set(x - 1, y, bl); } else { new_lattice.set(max_x, y, bl); };
                        new_lattice.set(x, y, br);
                    },
                    2 => {  // bottom left
                        if y != 0 { new_lattice.set(x, y - 1, tl); } else { new_lattice.set(x, max_y, tl); };
                        if y != 0 { new_lattice.set(x + 1, y - 1, tr); } else { new_lattice.set(x + 1, max_y, tr); };
                        new_lattice.set(x, y, bl);
                        new_lattice.set(x + 1, y, br);
                    },
                    3 => {  // top right
                        if x != 0 { new_lattice.set(x - 1, y, tl); } else { new_lattice.set(max_x, y, tl); };
                        new_lattice.set(x, y, tr);
                        if x != 0 { new_lattice.set(x - 1, y + 1, bl); } else { new_lattice.set(max_x, y + 1, bl); };
                        new_lattice.set(x, y + 1, br);
                    },
                    _ => panic!(),
                };
            }
        }

        Ok(new_lattice)
    }
}

pub struct BlockAutomaton<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    lattice: Lattice<C>,
    automaton: BlockAutomatonMachine<C, B, NN, S>,
    global_state: (BlockAutomatonState, S),

    state: Option<Lattice<C>>,
    error: bool,
}

impl<C, B, NN, S> Clone for  BlockAutomaton<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn clone(&self) -> Self {
        Self { lattice: self.lattice.clone(), automaton: self.automaton.clone(), global_state: self.global_state.clone(), state: self.state.clone(), error: self.error }
    }
}

impl<C, B, NN, S> BlockAutomaton<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    pub fn new(lattice: Lattice<C>, global_state: S, automaton: impl for<'a, 'b> Fn(&'a NN, &'b mut S) -> [C; 4] + 'static) -> Result<Self, String> {
        if lattice.size.0 % 2 == 1 || lattice.size.1 % 2 == 1 {
            return Err(format!("Lattice size for block automata must be even"));
        }

        let automaton = BlockAutomatonMachine {
            automaton: Rc::new(automaton),
            _phantom: PhantomData,
        };

        Ok(Self { lattice, automaton, global_state: (BlockAutomatonState { step: 0 }, global_state), state: None, error: false })
    }
}

impl<C, B, NN, S> Iterator for BlockAutomaton<C, B, NN, S>
    where C: Clone + PartialEq + Eq, S: Clone, BlockAutomatonMachine<C, B, NN, S>: AutomatonMachine<C, (BlockAutomatonState, S)>
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