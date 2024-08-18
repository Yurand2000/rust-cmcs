#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Cell {
    Wall,
    NotVisited,
    Visited{ len: u32 },
    End,
    Backtrace{ len: u32 }
}

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct Lattice {
    cells: Vec<Cell>,
    size: (u32, u32)
}

impl Lattice {
    pub fn from_string(maze_str: &str) -> Option<Self> {
        use Cell::*;

        let size_x = maze_str.lines().next().unwrap().chars().count() as u32;
        let size_y = maze_str.lines().count() as u32;

        if maze_str.lines().map(|line| line.chars().count()).any(|len| len != size_x as usize) {
            return None;
        }

        let mut maze = Self::empty(size_x, size_y);
        for (y, line) in maze_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let cell =
                    match ch {
                        '#' => Wall,
                        ' ' => NotVisited,
                        'S' => Visited { len: 0 },
                        'E' => End,
                        _ => Wall,
                    };

                maze.set(x as u32, y as u32, cell);
            }
        }

        Some(maze)
    }

    pub fn empty(size_x: u32, size_y: u32) -> Self {
        Self {
            cells: vec![Cell::Wall; (size_x * size_y) as usize],
            size: (size_x, size_y),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells.get(self.get_index(x, y))
    }

    pub fn set(&mut self, x: u32, y: u32, new_state: Cell) -> bool {
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
    
    fn get_unchecked(&self, x: u32, y: u32) -> Cell {
        let index = self.get_index(x, y);
        self.cells[index].clone()
    }

    fn get_neighbors(&self, x: u32, y: u32) -> Option<[Cell; 4]> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }

        let top = if y == 0 { Cell::Wall } else { self.get_unchecked(x, y - 1) };
        let bottom = if y + 1 == self.size.1 { Cell::Wall } else { self.get_unchecked(x, y + 1) };
        let left = if x == 0 { Cell::Wall } else { self.get_unchecked(x - 1, y) };
        let right = if x + 1 == self.size.0 { Cell::Wall } else { self.get_unchecked(x + 1, y) };

        Some([top, right, bottom, left])
    }
}

#[derive(Clone)]
pub struct MazeSolver {
    maze: Lattice,

    state: Option<Lattice>,
}

impl MazeSolver {
    pub fn new(maze: Lattice) -> Self {
        Self { maze, state: None }
    }

    fn step(state: Lattice) -> Lattice {
        use Cell::*;

        let mut next_state = Lattice::empty(state.size.0, state.size.1);

        for x in 0..next_state.size.0 {
            for y in 0..next_state.size.1 {
                let cell = state.get(x, y).unwrap();

                if *cell == Wall {
                    next_state.set(x, y, Wall);
                } else {
                    let neighbors = state.get_neighbors(x, y).unwrap().into_iter();

                    let next_cell = match cell {
                        NotVisited => {
                            let neighbor_visited_best =
                                neighbors.fold(None, |acc, cell| {
                                    if let Visited { len } = cell {
                                        Some(acc.map_or(len, |best_len| u32::min(best_len, len)))
                                    } else {
                                        acc
                                    }
                                });

                            if let Some(len) = neighbor_visited_best {
                                Visited { len: len + 1 }
                            } else {
                                NotVisited
                            }
                        },
                        Visited { len } => {
                            let neighbor_backtrace_best =
                                neighbors.fold(None, |acc, cell| {
                                    if let Backtrace { len } = cell {
                                        Some(acc.map_or(len, |best_len| u32::min(best_len, len)))
                                    } else {
                                        acc
                                    }
                                });
                            
                            if let Some(blen) = neighbor_backtrace_best {
                                if *len >= blen {
                                    Visited { len: *len }
                                } else {
                                    Backtrace { len: *len }
                                }
                            } else {
                                Visited { len: *len }
                            }
                        },
                        End => {
                            let neighbor_visited_best =
                                neighbors.fold(None, |acc, cell| {
                                    if let Visited { len } = cell {
                                        Some(acc.map_or(len, |best_len| u32::min(best_len, len)))
                                    } else {
                                        acc
                                    }
                                });

                            if let Some(len) = neighbor_visited_best {
                                Backtrace { len: len + 1 }
                            } else {
                                End
                            }
                        },
                        Backtrace { len } => Backtrace { len: *len },
                        Wall => panic!(),
                    };

                    next_state.set(x, y, next_cell);
                }
            }
        }

        next_state
    }
}

impl Iterator for MazeSolver {
    type Item = Lattice;

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::take(&mut self.state);
        match state {
            Some(state) => {
                self.state = Some(Self::step(state));
                self.state.clone()
            },
            None => {
                self.state = Some(self.maze.clone());
                self.state.clone()
            },
        }
    }
}

pub mod mazes;