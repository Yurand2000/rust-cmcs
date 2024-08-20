use std::{marker::PhantomData, rc::Rc};

pub trait ToCell<C> {
    fn to_cell() -> C;
}

pub struct FixedBoundary<C, B: ToCell<C>>(PhantomData<(C, B)>);
pub struct PeriodicBoundary;
pub struct VonNeumannNeighborhood;
pub struct MooreNeighborhood;

pub struct Lattice<C, N, B>
    where C: Clone + PartialEq + Eq
{
    cells: Vec<C>,
    size: (u32, u32),
    _phantom: PhantomData<(N, B)>,
}

impl<C, N, B> Clone for Lattice<C, N, B>
    where C: Clone + PartialEq + Eq
{
    fn clone(&self) -> Self {
        Self { cells: self.cells.clone(), size: self.size.clone(), _phantom: self._phantom }
    }
}


impl<C, N, B> PartialEq for Lattice<C, N, B>
    where C: Clone + PartialEq + Eq
{
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells && self.size == other.size
    }
}

impl<C, N, B> Eq for Lattice<C, N, B>
    where C: Clone + PartialEq + Eq { }

impl<C, N, B> Lattice<C, N, B>
    where C: Clone + PartialEq + Eq
{
    pub fn from_fn(size_x: u32, size_y: u32, fun: impl Fn(u32, u32) -> C) -> Self {
        let mut cells = Vec::with_capacity((size_x * size_y) as usize);

        for y in 0..size_y {
            for x in 0..size_x {
                cells.push(fun(x, y));
            }
        }

        Self {
            cells,
            size: (size_x, size_y),
            _phantom: PhantomData,
        }
    }

    pub fn fill(size_x: u32, size_y: u32, cell: C) -> Self {
        Self {
            cells: vec![cell; (size_x * size_y) as usize],
            size: (size_x, size_y),
            _phantom: PhantomData,
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
    
    fn get_unchecked(&self, x: u32, y: u32) -> C {
        let index = self.get_index(x, y);
        self.cells[index].clone()
    }
}

impl<C, B> Lattice<C, VonNeumannNeighborhood, FixedBoundary<C, B>>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn get_neighbors(&self, x: u32, y: u32) -> Option<[C; 5]> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }

        let (max_x, max_y) = (self.size.0 - 1, self.size.1 - 1);

        let top = if y == 0 { B::to_cell() } else { self.get_unchecked(x, y - 1) };
        let bottom = if y == max_y { B::to_cell() } else { self.get_unchecked(x, y + 1) };
        let center = self.get_unchecked(x, y);
        let left = if x == 0 { B::to_cell() } else { self.get_unchecked(x - 1, y) };
        let right = if x == max_x { B::to_cell() } else { self.get_unchecked(x + 1, y) };

        Some([top, right, center, bottom, left])
    }
}

impl<C> Lattice<C, VonNeumannNeighborhood, PeriodicBoundary>
    where C: Clone + PartialEq + Eq
{
    fn get_neighbors(&self, x: u32, y: u32) -> Option<[C; 5]> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }

        let (max_x, max_y) = (self.size.0 - 1, self.size.1 - 1);

        let top = if y == 0 { self.get_unchecked(x, max_y) } else { self.get_unchecked(x, y - 1) };
        let bottom = if y == max_y { self.get_unchecked(x, 0) } else { self.get_unchecked(x, y + 1) };
        let center = self.get_unchecked(x, y);
        let left = if x == 0 { self.get_unchecked(max_x, y) } else { self.get_unchecked(x - 1, y) };
        let right = if x == max_x { self.get_unchecked(0, y) } else { self.get_unchecked(x + 1, y) };

        Some([top, right, center, bottom, left])
    }
}

impl<C, B> Lattice<C, MooreNeighborhood, FixedBoundary<C, B>>
    where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn get_neighbors(&self, x: u32, y: u32) -> Option<[C; 9]> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }

        let (max_x, max_y) = (self.size.0 - 1, self.size.1 - 1);

        let top_left = if y == 0 && x == 0 { B::to_cell() } else { self.get_unchecked(x - 1, y - 1) };
        let top = if y == 0 { B::to_cell() } else { self.get_unchecked(x, y - 1) };
        let top_right = if y == 0 && x == max_x { B::to_cell() } else { self.get_unchecked(x + 1, y - 1) };
        
        let left = if x == 0 { B::to_cell() } else { self.get_unchecked(x - 1, y) };
        let center = self.get_unchecked(x, y);
        let right = if x == max_x { B::to_cell() } else { self.get_unchecked(x + 1, y) };

        let bottom_left = if y == max_y && x == 0 { B::to_cell() } else { self.get_unchecked(x - 1, y + 1) };
        let bottom = if y == max_y { B::to_cell() } else { self.get_unchecked(x, y + 1) };
        let bottom_right = if y == max_y && x == max_x { B::to_cell() } else { self.get_unchecked(x + 1, y + 1) };

        Some([top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right])
    }
}

impl<C> Lattice<C, MooreNeighborhood, PeriodicBoundary>
    where C: Clone + PartialEq + Eq
{
    fn get_neighbors(&self, x: u32, y: u32) -> Option<[C; 9]> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }

        let (max_x, max_y) = (self.size.0 - 1, self.size.1 - 1);

        let top_left = if y == 0 && x == 0 { self.get_unchecked(max_x, max_y) } else { self.get_unchecked(x - 1, y - 1) };
        let top = if y == 0 { self.get_unchecked(x, max_y) } else { self.get_unchecked(x, y - 1) };
        let top_right = if y == 0 && x == max_x { self.get_unchecked(0, max_y) } else { self.get_unchecked(x + 1, y - 1) };
        
        let left = if x == 0 { self.get_unchecked(max_x, y) } else { self.get_unchecked(x - 1, y) };
        let center = self.get_unchecked(x, y);
        let right = if x == max_x { self.get_unchecked(0, y) } else { self.get_unchecked(x + 1, y) };

        let bottom_left = if y == max_y && x == 0 { self.get_unchecked(max_x, 0) } else { self.get_unchecked(x - 1, y + 1) };
        let bottom = if y == max_y { self.get_unchecked(x, 0) } else { self.get_unchecked(x, y + 1) };
        let bottom_right = if y == max_y && x == max_x { self.get_unchecked(0, 0) } else { self.get_unchecked(x + 1, y + 1) };

        Some([top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right])
    }
}

trait AutomatonMachine<C, N, B>
    where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C, N, B>) -> Lattice<C, N, B>;
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

impl<C, B> AutomatonMachine<C, VonNeumannNeighborhood, FixedBoundary<C, B>> for
    AutomatonMachine2D<C, VonNeumannNeighborhood, FixedBoundary<C, B>, [C; 5]>
        where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn step(&self, state: Lattice<C, VonNeumannNeighborhood, FixedBoundary<C, B>>) -> Lattice<C, VonNeumannNeighborhood, FixedBoundary<C, B>> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = state.get_neighbors(x, y).unwrap();
            automaton(&neighborhood)
        })
    }
}

impl<C> AutomatonMachine<C, VonNeumannNeighborhood, PeriodicBoundary> for
    AutomatonMachine2D<C, VonNeumannNeighborhood, PeriodicBoundary, [C; 5]>
        where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C, VonNeumannNeighborhood, PeriodicBoundary>) -> Lattice<C, VonNeumannNeighborhood, PeriodicBoundary> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = state.get_neighbors(x, y).unwrap();
            automaton(&neighborhood)
        })
    }
}

impl<C, B> AutomatonMachine<C, MooreNeighborhood, FixedBoundary<C, B>> for
    AutomatonMachine2D<C, MooreNeighborhood, FixedBoundary<C, B>, [C; 9]>
        where C: Clone + PartialEq + Eq, B: ToCell<C>
{
    fn step(&self, state: Lattice<C, MooreNeighborhood, FixedBoundary<C, B>>) -> Lattice<C, MooreNeighborhood, FixedBoundary<C, B>> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = state.get_neighbors(x, y).unwrap();
            automaton(&neighborhood)
        })
    }
}

impl<C> AutomatonMachine<C,  MooreNeighborhood, PeriodicBoundary> for
    AutomatonMachine2D<C, MooreNeighborhood, PeriodicBoundary, [C; 9]>
        where C: Clone + PartialEq + Eq
{
    fn step(&self, state: Lattice<C, MooreNeighborhood, PeriodicBoundary>) -> Lattice<C, MooreNeighborhood, PeriodicBoundary> {
        let automaton = self.automaton.clone();
        Lattice::from_fn(state.size.0, state.size.1, move |x, y| {
            let neighborhood = state.get_neighbors(x, y).unwrap();
            automaton(&neighborhood)
        })
    }
}

pub struct Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    lattice: Lattice<C, N, B>,
    automaton: AutomatonMachine2D<C, N, B, NN>,

    state: Option<Lattice<C, N, B>>,
}

impl<C, N, B, NN> Clone for  Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    fn clone(&self) -> Self {
        Self { lattice: self.lattice.clone(), automaton: self.automaton.clone(), state: self.state.clone() }
    }
}

impl<C, N, B, NN> Automaton2D<C, N, B, NN>
    where C: Clone + PartialEq + Eq
{
    pub fn new(lattice: Lattice<C, N, B>, automaton: impl Fn(&NN) -> C + 'static) -> Self {
        let automaton = AutomatonMachine2D {
            automaton: Rc::new(automaton),
            _phantom: PhantomData,
        };

        Self { lattice, automaton, state: None }
    }
}

impl<C, N, B, F> Iterator for Automaton2D<C, N, B, F>
    where C: Clone + PartialEq + Eq, AutomatonMachine2D<C, N, B, F>: AutomatonMachine<C, N, B>
{
    type Item = Lattice<C, N, B>;

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::take(&mut self.state);
        match state {
            Some(state) => {
                self.state = Some(self.automaton.step(state));
                self.state.clone()
            },
            None => {
                self.state = Some(self.lattice.clone());
                self.state.clone()
            },
        }
    }
}