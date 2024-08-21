use std::{marker::PhantomData, rc::Rc};

pub trait ToCell<C> {
    fn to_cell() -> C;
}

pub struct FixedBoundary<C, B: ToCell<C>>(PhantomData<(C, B)>);
pub struct PeriodicBoundary;
pub struct VonNeumannNeighborhood;
pub struct MooreNeighborhood;

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct Lattice<C>
    where C: Clone + PartialEq + Eq
{
    cells: Vec<C>,
    size: (u32, u32),
}

impl<C> Lattice<C>
    where C: Clone + PartialEq + Eq
{
    pub fn from_fn(size_x: u32, size_y: u32, fun: impl Fn(u32, u32) -> Result<C, String>) -> Result<Self, String> {
        let mut cells = Vec::with_capacity((size_x * size_y) as usize);

        for y in 0..size_y {
            for x in 0..size_x {
                cells.push(fun(x, y)?);
            }
        }

        Ok(Self {
            cells,
            size: (size_x, size_y),
        })
    }

    pub fn fill(size_x: u32, size_y: u32, cell: C) -> Self {
        Self {
            cells: vec![cell; (size_x * size_y) as usize],
            size: (size_x, size_y),
        }   
    }

    pub fn empty(size_x: u32, size_y: u32) -> Self
        where C: Default
    {
        Self::fill(size_x, size_y, Default::default())
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&C> {
        self.cells.get(self.get_index(x, y))
    }

    pub fn set(&mut self, x: u32, y: u32, new_state: C) -> bool {
        let index = self.get_index(x, y);
        let cell = self.cells.get_mut(index);
        match cell {
            Some(cell) => { *cell = new_state; true },
            None => false,
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.size.0) as usize
    }

    fn get_result(&self, x: u32, y: u32) -> Result<C, String> {
        self.get(x, y) .cloned()
            .ok_or_else(|| format!("Lattice get error: {}/{}, {}/{}", x, self.size.0, y, self.size.1))
    }
}



impl<C, B, NN> AutomatonMachine2D<C, VonNeumannNeighborhood, FixedBoundary<C, B>, NN>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
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

impl<C, NN> AutomatonMachine2D<C, VonNeumannNeighborhood, PeriodicBoundary, NN>
    where C: Clone + PartialEq + Eq
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

impl<C, B, NN> AutomatonMachine2D<C, MooreNeighborhood, FixedBoundary<C, B>, NN>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
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

impl<C, NN> AutomatonMachine2D<C, MooreNeighborhood, PeriodicBoundary, NN>
    where C: Clone + PartialEq + Eq
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

trait AutomatonMachine<C>
    where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C>) -> Result<Lattice<C>, String>;
}

struct AutomatonMachine2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    automaton: Rc<dyn Fn(&NN) -> C>,
    _phantom: PhantomData<(C, N, B)>,
}

impl<C, N, B, F> Clone for AutomatonMachine2D<C, N, B, F>
    where C: Clone + PartialEq + Eq
{
    fn clone(&self) -> Self {
        Self { automaton: self.automaton.clone(), _phantom: self._phantom }
    }
}

impl<C, B> AutomatonMachine<C> for AutomatonMachine2D<C, VonNeumannNeighborhood, FixedBoundary<C, B>, [C; 5]>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn step(&self, state: Lattice<C>) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&state, x, y)?;
            Ok(automaton(&neighborhood))
        })
    }
}

impl<C> AutomatonMachine<C> for AutomatonMachine2D<C, VonNeumannNeighborhood, PeriodicBoundary, [C; 5]>
    where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C>) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&state, x, y)?;
            Ok(automaton(&neighborhood))
        })
    }
}

impl<C, B> AutomatonMachine<C> for AutomatonMachine2D<C, MooreNeighborhood, FixedBoundary<C, B>, [C; 9]>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn step(&self, state: Lattice<C>) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&state, x, y)?;
            Ok(automaton(&neighborhood))
        })
    }
}

impl<C> AutomatonMachine<C> for AutomatonMachine2D<C, MooreNeighborhood, PeriodicBoundary, [C; 9]>
    where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C>) -> Result<Lattice<C>, String> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = Self::get_neighbors(&state, x, y)?;
            Ok(automaton(&neighborhood))
        })
    }
}

pub struct Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    lattice: Lattice<C>,
    automaton: AutomatonMachine2D<C, N, B, NN>,

    state: Option<Lattice<C>>,
    error: bool,
}

impl<C, N, B, NN> Clone for  Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    fn clone(&self) -> Self {
        Self { lattice: self.lattice.clone(), automaton: self.automaton.clone(), state: self.state.clone(), error: self.error }
    }
}

impl<C, N, B, NN> Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    pub fn new(lattice: Lattice<C>, automaton: impl Fn(&NN) -> C + 'static) -> Self {
        let automaton = AutomatonMachine2D {
            automaton: Rc::new(automaton),
            _phantom: PhantomData,
        };

        Self { lattice, automaton, state: None, error: false }
    }
}

impl<C, N, B, F> Iterator for Automaton2D<C, N, B, F>
    where C: Clone + PartialEq + Eq, AutomatonMachine2D<C, N, B, F>: AutomatonMachine<C>
{
    type Item = Result<Lattice<C>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        let state = std::mem::take(&mut self.state);
        match state {
            Some(state) => {
                match self.automaton.step(state) {
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