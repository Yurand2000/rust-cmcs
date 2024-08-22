use crate::cellular_automata::prelude::{*, automaton_2d::*};

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Cell {
    Wall,
    NotVisited,
    Visited{ len: u32 },
    End,
    Backtrace{ len: u32 }
}

pub struct Boundary;

impl ToCell<Cell> for Boundary {
    fn to_cell() -> Cell {
        Cell::Wall
    }
}

pub type Maze = Lattice<Cell>;
pub struct MazeSolver(Automaton2D<Cell, VonNeumannNeighborhood, FixedBoundary<Cell, Boundary>, [Cell; 5], ()>);

impl Clone for MazeSolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl MazeSolver
{
    pub fn from_string(maze_str: &str) -> Option<Self> {
        use Cell::*;

        let size_x = maze_str.lines().next().unwrap().chars().count() as u32;
        let size_y = maze_str.lines().count() as u32;

        if maze_str.lines().map(|line| line.chars().count()).any(|len| len != size_x as usize) {
            return None;
        }

        let mut maze = Lattice::fill(size_x, size_y, Cell::Wall);
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

        let automaton = Automaton2D::new(maze, (), Self::automaton);

        Some(Self(automaton))
    }

    fn automaton(neighborhood: &[Cell; 5], _: &mut ()) -> Cell {
        use Cell::*;

        let cell = &neighborhood[2];
        let neighbors = neighborhood[0..2].iter().chain(neighborhood[3..].iter());

        match cell {
            NotVisited => {
                let neighbor_visited_best =
                    neighbors.fold(None, |acc, cell| {
                        if let Visited { len } = cell {
                            Some(acc.map_or(*len, |best_len| u32::min(best_len, *len)))
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
                            Some(acc.map_or(*len, |best_len| u32::min(best_len, *len)))
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
                            Some(acc.map_or(*len, |best_len| u32::min(best_len, *len)))
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
            Wall => Wall,
        }
    }
}

impl Iterator for MazeSolver
{
    type Item = Result<Maze, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub mod mazes;