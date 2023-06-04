extern crate rand;

use rand::{thread_rng, Rng};

fn binary_tree(mut grid: Grid) {
    for row in grid.cells.iter_mut() {
        for cell in row.iter_mut() {
            if cell.north.is_none() & cell.east.is_none() {
                println!("we're in the top right corner!");
                break;
            }
            else if cell.north.is_none() {
                // at this point we're referring to the first row
                // for binary tree, the eastern cells are always linked
                // apart from the case handled above
                // so we want, in this case, to add the eastern location to
                // current cells 'links' vector
                // let's try that
                let eastern_location = cell.east.unwrap();
                cell.links.push(Location {
                    row: eastern_location.row,
                    column: eastern_location.column,
                });
                println!("cell links updated to {:#?}", cell.links);
            }

            let mut neighbours: Vec<Location> = vec![];
            if cell.north.is_some() {
                println!("yep, found a cell to the north");
                let northern_location = cell.north.unwrap();
                neighbours.push(Location {
                    row: northern_location.row,
                    column: northern_location.column,
                })
            }
            if cell.east.is_some() {
                println!("yep, found a cell to the east");
                let eastern_location = cell.east.unwrap();
                neighbours.push(Location {
                    row: eastern_location.row,
                    column: eastern_location.column,
                })
            }
            // what do we want to do for the top row?
            // and the eastern row?
            // always link, I suppose
            let index = rand::thread_rng().gen_range(0..=1);
            let linked_neighbour = neighbours[index];



        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn prepare_grid(&mut self) -> Vec<Vec<Cell>> {
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for r in 0..self.rows {
            let mut row: Vec<Cell> = Vec::new();

            for c in 0..self.columns {
                row.push(Cell::empty(r, c));
            }

            cells.push(row)
        }
        cells
    }

    pub fn get_neighbour(
        rows: &i32,
        columns: &i32,
        current_location: &Location,
        direction: &str,
    ) -> Option<Location> {
        let row_range = 0..*rows;
        let col_range = 0..*columns;
        let current_row = current_location.row as i32;
        let current_column = current_location.column as i32;

        match direction {
            "north" => {
                if row_range.contains(&(current_row - 1)) {
                    Some(Location {
                        row: current_location.row - 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            "east" => {
                if col_range.contains(&(current_column + 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column + 1,
                    })
                } else {
                    None
                }
            }
            "south" => {
                if row_range.contains(&(current_row + 1)) {
                    Some(Location {
                        row: current_location.row + 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            "west" => {
                if row_range.contains(&(current_column - 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column - 1,
                    })
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub fn configure_cells(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                let location = Location {
                    row: cell.row,
                    column: cell.column,
                };
                let rows = *&self.rows as i32;
                let columns = *&self.columns as i32;

                cell.north = Grid::get_neighbour(&rows, &columns, &location, "north");
                cell.east = Grid::get_neighbour(&rows, &columns, &location, "east");
                cell.south = Grid::get_neighbour(&rows, &columns, &location, "south");
                cell.west = Grid::get_neighbour(&rows, &columns, &location, "west");
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Cell {
    row: usize,
    column: usize,
    north: Option<Location>,
    east: Option<Location>,
    south: Option<Location>,
    west: Option<Location>,
    links: Vec<Location>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Location {
    row: usize,
    column: usize,
}

impl Cell {
    pub fn link(&mut self, mut target: Cell) {
        self.links.push(Location {
            row: target.row,
            column: target.column,
        });
        target.links.push(Location {
            row: self.row,
            column: self.column,
        });
    }
}

impl Cell {
    pub fn empty(row: usize, column: usize) -> Self {
        Cell {
            row,
            column,
            ..Default::default()
        }
    }
}

fn main() {
    let mut grid = Grid {
        rows: 3,
        columns: 3,
        cells: Vec::new(),
    };

    grid.cells = grid.prepare_grid();
    // println!("{:#?}", grid);
    grid.configure_cells();
    println!("{:#?}", grid);

    binary_tree(grid);
}
