#[path = "../symbol/mod.rs"]
mod symbol;

use std::str::FromStr;

use symbol::Symbol;

#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<Symbol>>,
    start_index: (usize, usize),
}
impl Maze {
    pub fn new(maze_str: &Vec<&str>) -> Self {
        let mut start_index = (0, 0);
        let maze = maze_str.iter()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
            .enumerate()
            .map(|(y, c)| {
                let sym = Symbol::from_str(c.to_string().as_str()).unwrap();
                match sym {
                    Symbol::Start => {
                        start_index = (x, y);
                        return sym
                    },
                    _ => return sym,
                }
            })
            .collect::<Vec<Symbol>>()
        }).collect::<Vec<Vec<Symbol>>>();
        Maze {maze, start_index}
    }

    pub fn get(&self, x: usize, y: usize) -> Symbol {
        *self.maze.get(x).unwrap().get(y).unwrap()
    }
}