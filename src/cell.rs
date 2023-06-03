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

#[derive(Debug, Eq, PartialEq)]
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

        // not sure what's going to happen with ownership here
        // if using cell instances is too tricky, could just use locations with xy axes instead
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
}

// Things I am yet to understand
// 1. why does links field need to be in a box, why can it not just be HashMap<Box<Cell>, bool>>?
// (This is probably the same reason as NESW needs to be in Box, i.e. the compiler has no way of knowing the size in advance, so it cannot compile)
// (Could there be a way to communicate this to the compiler, e.g. declaring a links type?)
// 2. why was it necessary to manually implement hash for Cell?
// 3. why is hash() only called on row and column and not the other fields?
// 4. what is `state` in reference to the hash() function?
