use std::marker::PhantomData;

#[derive(Clone, Copy)]
#[derive(Default)]
pub enum StartingState {
    #[default]
    SingleCell,
    Random,
    Full,
    Empty,
}

impl StartingState {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "single" => Some(Self::SingleCell),
            "random" => Some(Self::Random),
            "full" => Some(Self::Full),
            "empty" => Some(Self::Empty),
            _ => None,
        }
    }
}

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
    pub cells: Vec<C>,
    pub size: (u32, u32),
}

impl<C> Lattice<C>
    where C: Clone + PartialEq + Eq
{
    pub fn from_fn(size_x: u32, size_y: u32, mut fun: impl FnMut(u32, u32) -> Result<C, String>) -> Result<Self, String> {
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

    pub fn get_result(&self, x: u32, y: u32) -> Result<C, String> {
        self.get(x, y) .cloned()
            .ok_or_else(|| format!("Lattice get error: {}/{}, {}/{}", x, self.size.0, y, self.size.1))
    }
}

pub trait AutomatonMachine<C, S>
    where C: Clone + PartialEq + Eq, S: Clone
{
    fn step(&self, lattice: Lattice<C>, state: &mut S) -> Result<Lattice<C>, String>;
}