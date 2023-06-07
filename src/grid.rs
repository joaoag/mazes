use crate::cell::MazeCell;
use crate::direction::Direction;
use crate::location::Location;

#[derive(Debug)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<Vec<MazeCell>>,
}

pub trait GridTrait {
    fn prepare_grid(&mut self) -> Vec<Vec<MazeCell>>;
    fn get_neighbour( rows: &i32, columns: &i32, current_location: &Location, direction: Direction) -> Option<Location>;
    fn configure_cells(&mut self);
}

impl Grid {
    fn prepare_grid(&mut self) -> Vec<Vec<MazeCell>> {
        let mut cells: Vec<Vec<MazeCell>> = Vec::new();

        for r in 0..self.rows {
            let mut row: Vec<MazeCell> = Vec::new();

            for c in 0..self.columns {
                row.push(MazeCell::empty(r, c));
            }

            cells.push(row)
        }
        cells
    }

    fn get_neighbour(
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

    fn configure_cells(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                let rows = *&self.rows as i32;
                let columns = *&self.columns as i32;

                cell.north = Grid::get_neighbour(&rows, &columns, &cell.location, Direction::North);
                cell.east = Grid::get_neighbour(&rows, &columns, &cell.location, Direction::East);
                cell.south = Grid::get_neighbour(&rows, &columns, &cell.location, Direction::South);
                cell.west = Grid::get_neighbour(&rows, &columns, &cell.location, Direction::West);
            }
        }
    }
}
