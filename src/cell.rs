use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Grid {
    rows: u8,
    columns: u8,
    grid: Vec<Vec<Cell>>
}

impl Grid {

    pub fn prepare_grid(&mut self) -> Vec<Vec<Cell>> {
        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for r in 1..=self.rows {
            let mut row: Vec<Cell> = Vec::new();

            for c in 1..=self.columns {
                row.push(Cell::empty(r,c));
            }

            grid.push(row)
        }
        grid
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: Box<HashMap<Box<Cell>, bool>>
}


impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}


impl Cell {
    pub fn link(&mut self, target: Box<Cell>){
        self.links.insert(
            target,
            true
        );
        // target.links.insert(
        //     self,
        //     true
        // );
        // println!("{:#?}", target);
        // not sure what's going to happen with ownership here

    }
}

impl Cell {
    pub fn empty(row: u8, column: u8) -> Self {
        Cell {
            row,
            column,
            ..Default::default()
        }
    }

}

fn main(){

    let mut grid = Grid {
        rows: 3,
        columns: 3,
        grid: Vec::new()
    };

    grid.grid = grid.prepare_grid();
    println!("{:#?}", grid);
}

// Things I am yet to understand
// 1. why does links field need to be in a box, why can it not just be HashMap<Box<Cell>, bool>>?
// (This is probably the same reason as NESW needs to be in Box, i.e. the compiler has no way of knowing the size in advance, so it cannot compile)
// (Could there be a way to communicate this to the compiler, e.g. declaring a links type?)
// 2. why was it necessary to manually implement hash for Cell?
// 3. why is hash() only called on row and column and not the other fields?
// 4. what is `state` in reference to the hash() function?


// Grid
// should own cells