extern crate rand;

use rand::Rng;
use std::ops::IndexMut;

fn binary_tree_random_neighbour(eastern: Location, northern: Location) -> Location {
    let mut neighbours: Vec<Location> = vec![];
    neighbours.extend([eastern, northern]);

    let index = rand::thread_rng().gen_range(0..=1);
    let linked_neighbour = neighbours[index];
    linked_neighbour
}

fn binary_tree(mut grid: Grid) -> Grid {
    let mut links: Vec<Link> = vec![];

    for row in grid.cells.iter_mut() {
        for cell in row.iter_mut() {
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let is_north_eastern_cell = is_northmost_cell & is_eastmost_cell;

            // use cell.links()?
            // how to bi-directionally update?

            if is_north_eastern_cell {
                // println!("identified north eastern cell");
                break;
            } else if is_northmost_cell {
                // println!("identified northmost cell");
                let eastern_location = cell.east.unwrap();
                cell.links.push(eastern_location);
                links.push(Link {
                    source: cell.location,
                    target: eastern_location,
                });
            } else if is_eastmost_cell {
                // println!("identified eastmost cell");
                let northern_location = cell.north.unwrap();
                cell.links.push(northern_location);
                links.push(Link {
                    source: cell.location,
                    target: northern_location,
                });
            } else {
                // println!("identified non-eastmost and non-northmost cell");

                let linked_neighbour =
                    binary_tree_random_neighbour(cell.east.unwrap(), cell.north.unwrap());

                cell.links.push(linked_neighbour);
                links.push(Link {
                    source: cell.location,
                    target: linked_neighbour,
                });
            }
        }
    }
    // println!("Here's the updated links {:#?}", links);
    for link in links.iter() {
        let Link { source, target } = link;

        let target_cell = grid.cells.index_mut(target.row).index_mut(target.column);
        // println!("target cell before linking: {:#?}", target_cell);
        // println!("should next have source cell added to its links: {:#?}", source);
        target_cell.links.push(*source);
        // println!("target cell: {:#?}", target_cell);
    }
    grid
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Link {
    source: Location,
    target: Location,
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

    pub fn display_maze(self) {
        let start = String::from("+");
        let middle = String::from("---+".repeat(self.columns));
        let end = String::from("\n");
        let mut output = format!("{}{}{}", start, middle, end); // this is correct

        for row in self.cells.iter() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row.iter() {
                let body = "   ";
                let east_boundary = if Cell::is_linked(&cell, Direction::East) {
                    " "
                } else {
                    "|"
                };

                top.push_str((body.to_owned() + east_boundary).as_str());

                let south_boundary = if Cell::is_linked(&cell, Direction::South) {
                    "   "
                } else {
                    "---"
                };
                let corner = "+";
                bottom.push_str((south_boundary.to_owned() + corner).as_str());
            }
            output.push_str((top.to_owned() + "\n").as_str());
            output.push_str((bottom.to_owned() + "\n").as_str());
        }

        println!("{}", output);
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

    pub fn configure_cells(&mut self) {
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

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Cell {
    location: Location,
    north: Option<Location>,
    east: Option<Location>,
    south: Option<Location>,
    west: Option<Location>,
    links: Vec<Location>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Location {
    row: usize,
    column: usize,
}

impl Cell {
    pub fn link(&mut self, mut target: Cell) {
        println!("{:#?}", self);
        println!("{:#?}", target);
        self.links.push(target.location);
        target.links.push(self.location);
        println!("{:#?}", self);
        println!("{:#?}", target);
    }
    pub fn empty(row: usize, column: usize) -> Self {
        Cell {
            location: Location { row, column },
            ..Default::default()
        }
    }
    pub fn is_linked(&self, direction: Direction) -> bool {
        if self.links.is_empty() {
            return false;
        }
        match direction {
            Direction::North if self.north.is_some() => self.links.contains(&self.north.unwrap()),
            Direction::East if self.east.is_some() => self.links.contains(&self.east.unwrap()),
            Direction::South if self.south.is_some() => self.links.contains(&self.south.unwrap()),
            Direction::West if self.west.is_some() => self.links.contains(&self.west.unwrap()),
            _ => false,
        }
    }
}

fn main() {
    let mut grid = Grid {
        rows: 10,
        columns: 10,
        cells: Vec::new(),
    };

    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    // println!("{:#?}", grid);
    grid = binary_tree(grid);
    // println!("{:#?}", grid);
    grid.display_maze();
}
