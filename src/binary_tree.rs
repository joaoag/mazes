use rand::Rng;
use std::ops::IndexMut;

use crate::link::Link;
use crate::location::Location;
use crate::smart_grid::SmartGrid;

fn binary_tree_random_neighbour(eastern: Location, northern: Location) -> Location {
    let mut neighbours: Vec<Location> = vec![];
    neighbours.extend([eastern, northern]);

    let index = rand::thread_rng().gen_range(0..=1);
    let linked_neighbour = neighbours[index];
    linked_neighbour
}

pub fn binary_tree(grid: SmartGrid, bidirectional_link: bool) -> SmartGrid {
    for row in &grid.cells {
        for cell in row {
            let mut cell = cell.borrow_mut();
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let is_north_eastern_cell = is_northmost_cell & is_eastmost_cell;

            // TODO investigate smart pointers for bi-directional updates

            if is_north_eastern_cell {
                break;
            } else if is_northmost_cell {
                let eastern_location = cell.east.unwrap();
                cell.links.push(eastern_location);
                let mut target_cell =
                    grid.cells[eastern_location.row][eastern_location.column].borrow_mut();
                target_cell.links.push(cell.location);
            } else if is_eastmost_cell {
                let northern_location = cell.north.unwrap();
                cell.links.push(northern_location);
                let mut target_cell =
                    grid.cells[northern_location.row][northern_location.column].borrow_mut();
                target_cell.links.push(cell.location);
            } else {
                let linked_neighbour =
                    binary_tree_random_neighbour(cell.east.unwrap(), cell.north.unwrap());
                cell.links.push(linked_neighbour);
                let mut target_cell =
                    grid.cells[linked_neighbour.row][linked_neighbour.column].borrow_mut();
                target_cell.links.push(cell.location);
            }
        }
    }
    grid
}
