#[path = "../symbol/mod.rs"]
mod symbol;

use std::str::FromStr;

use symbol::Symbol;

#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<Symbol>>
}
impl Maze {
    pub fn new(maze_str: &Vec<&str>) -> Self {
        Maze {
            maze: maze_str.iter()
            .map(|line| {
                line.chars()
                .map(|c| Symbol::from_str(c.to_string().as_str()).unwrap())
                .collect::<Vec<Symbol>>()
            }).collect::<Vec<Vec<Symbol>>>()
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Symbol {
        *self.maze.get(x).unwrap().get(y).unwrap()
    }
}