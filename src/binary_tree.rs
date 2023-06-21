use rand::Rng;

use crate::location::Location;
use crate::smart_grid::SmartGrid;

const BIDI: bool = true;

fn binary_tree_random_neighbour(eastern: Location, northern: Location) -> Location {
    let mut neighbours: Vec<Location> = vec![];
    neighbours.extend([eastern, northern]);

    let linked_location = rand::thread_rng().gen_range(0..=1);

    neighbours[linked_location]
}

pub fn binary_tree(grid: SmartGrid) -> SmartGrid {
    for row in &grid.cells {
        for cell in row {
            let cell = cell.borrow_mut();
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let is_north_eastern_cell = is_northmost_cell & is_eastmost_cell;

            if is_north_eastern_cell {
                break;
            } else if is_northmost_cell {
                let eastern_location = cell.east.unwrap();
                SmartGrid::link_cells(&grid, cell, eastern_location, BIDI);
            } else if is_eastmost_cell {
                let northern_location = cell.north.unwrap();
                SmartGrid::link_cells(&grid, cell, northern_location, BIDI);
            } else {
                let linked_neighbour =
                    binary_tree_random_neighbour(cell.east.unwrap(), cell.north.unwrap());
                SmartGrid::link_cells(&grid, cell, linked_neighbour, BIDI);
            }
        }
    }
    grid
}
