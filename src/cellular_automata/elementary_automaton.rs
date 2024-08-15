use bitvec::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum BoundaryCondition {
    #[default]
    Fixed0,
    Fixed1,
    Periodic,
    Reflective
}

impl BoundaryCondition {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "fixed0" => Some(Self::Fixed0),
            "fixed1" => Some(Self::Fixed1),
            "periodic" => Some(Self::Periodic),
            "reflective" => Some(Self::Reflective),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Lattice {
    cells: BitVec
}

impl Lattice {
    pub fn empty(size: usize) -> Self {
        Self { cells: BitVec::repeat(false, size) }
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, idx: usize) -> Option<bool> {
        self.cells.get(idx).map(|bit| *bit)
    }

    pub fn set(&mut self, idx: usize, value: bool) -> bool {
        match self.cells.get_mut(idx) {
            Some(mut cell) => { *cell = value; true },
            None => false,
        }
    }
}

impl IntoIterator for Lattice {
    type Item = bool;

    type IntoIter = <BitBox<usize, Lsb0> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

#[derive(Clone)]
pub struct ElementaryAutomaton {
    initial_state: Lattice,
    boundary: BoundaryCondition,
    rule: u8,

    state: Option<(u32, Lattice)>,
}

impl ElementaryAutomaton {
    pub fn new(initial_state: Lattice, boundary: BoundaryCondition, rule: u8) -> Self {
        Self {
            initial_state,
            boundary,
            rule,
            state: None,
        }
    }

    fn step(state: Lattice, boundary: BoundaryCondition, rule: u8) -> Lattice {
        use BoundaryCondition::*;

        let size = state.size();
        let last = size - 1;
        let mut next_state = Lattice::empty(size);

        // compute boundary cells
        let (neighborhood_first, neighborhood_last) = 
            match boundary {
                Fixed0 => (
                    (((state.cells[0] as u8) << 1) + (state.cells[1] as u8)),
                    (((state.cells[last - 1] as u8) << 2) + ((state.cells[last] as u8) << 1))
                ),
                Fixed1 => (
                    (4 + ((state.cells[0] as u8) << 1) + (state.cells[1] as u8)),
                    (((state.cells[last - 1] as u8) << 2) + ((state.cells[last] as u8) << 1) + 1)
                ),
                Periodic => (
                    (((state.cells[last] as u8) << 2) + ((state.cells[0] as u8) << 1) + (state.cells[1] as u8)),
                    (((state.cells[last - 1] as u8) << 2) + ((state.cells[last] as u8) << 1) + (state.cells[0] as u8))
                ),
                Reflective => (
                    (((state.cells[1] as u8) << 2) + ((state.cells[0] as u8) << 1) + (state.cells[1] as u8)),
                    (((state.cells[last - 1] as u8) << 2) + ((state.cells[last] as u8) << 1) + (state.cells[last - 1] as u8))
                ),
            };

        next_state.cells
            .set(0, (rule & (1 << neighborhood_first)) > 0);
        next_state.cells
            .set(last, (rule & (1 << neighborhood_last)) > 0);

        // compute inner cells
        for idx in 1..last {
            let neighborhood = 
                ((state.cells[idx - 1] as u8) << 2) +
                ((state.cells[idx] as u8) << 1) +
                (state.cells[idx + 1] as u8);

            next_state.cells
                .set(idx, (rule & (1 << neighborhood)) > 0);
        }

        next_state
    }
}

impl Iterator for ElementaryAutomaton {
    type Item = (u32, Lattice);

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::take(&mut self.state);
        match state {
            Some((time, lattice)) => {
                let next_time = time + 1;
                let next_lattice = Self::step(lattice, self.boundary, self.rule);

                self.state = Some((next_time, next_lattice));
                self.state.clone()
            },
            None => {
                self.state = Some((0, self.initial_state.clone()));
                self.state.clone()
            },
        }
    }
}