extern crate rand;

use crate::binary_tree::binary_tree;
use crate::display::display_maze;
use crate::side_winder::side_winder;
use smart_grid::SmartGrid;

mod binary_tree;
mod cell;
mod direction;
mod display;
mod link;
mod location;
mod side_winder;
mod smart_grid;

fn main() {
    let mut grid = SmartGrid {
        rows: 10,
        columns: 10,
        cells: Vec::new(),
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    // grid = binary_tree(grid);
    grid = side_winder(grid);
    display_maze(&grid);
}
