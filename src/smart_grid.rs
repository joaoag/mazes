use crate::cell::MazeCell;
use crate::direction::Direction;
use crate::location::Location;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct SmartGrid {
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<Rc<RefCell<MazeCell>>>>
}

impl SmartGrid {
    pub fn prepare_grid(&mut self) -> Vec<Vec<Rc<RefCell<MazeCell>>>> {
        // TODO use new consistently for initialising empty Vec
        let mut cells  = Vec::new();
        // can I wrap the Rc and RefCell in a type, say SmartCell
        // no leave this out, it's idiomatic and expressive to Rust people, wrapping in a type might obscure this
        // TODO look into Arc<Mutex>

        for r in 0..self.rows {
            let mut row: Vec<Rc<RefCell<MazeCell>>> = Vec::new();

            for c in 0..self.columns {
                row.push(Rc::new (RefCell::new (MazeCell::empty(r, c))));
            }

            cells.push(row)
        }
        cells
    }

    pub fn get_neighbour(
        rows: &i32,
        columns: &i32,
        current_location: &Location,
        direction: Direction,
    ) -> Option<Location> {
        let row_range = 0..*rows;
        let col_range = 0..*columns;
        let current_row = current_location.row as i32;
        let current_column = current_location.column as i32;

        match direction {
            Direction::North => {
                if row_range.contains(&(current_row - 1)) {
                    Some(Location {
                        row: current_location.row - 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            Direction::East => {
                if col_range.contains(&(current_column + 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column + 1,
                    })
                } else {
                    None
                }
            }
            Direction::South => {
                if row_range.contains(&(current_row + 1)) {
                    Some(Location {
                        row: current_location.row + 1,
                        column: current_location.column,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if row_range.contains(&(current_column - 1)) {
                    Some(Location {
                        row: current_location.row,
                        column: current_location.column - 1,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn configure_cells(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let rows = *&self.rows as i32;
                let columns = *&self.columns as i32;
                let mut cell = cell.borrow_mut();
                // unknown why below does not work
                // let mut cell = *cell.borrow_mut();
                cell.north = SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::North);
                cell.east = SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::East);
                cell.south = SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::South);
                cell.west = SmartGrid::get_neighbour(&rows, &columns, &cell.location, Direction::West);
            }
        }
    }

}
