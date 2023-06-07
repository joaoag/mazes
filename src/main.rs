extern crate rand;

use crate::binary_tree::binary_tree;
use crate::display::display_maze;
use crate::grid::Grid;
use smart_grid::SmartGrid;

mod binary_tree;
mod cell;
mod direction;
mod display;
mod grid;
mod link;
mod location;
mod side_winder;
mod smart_grid;

fn main() {
    // below is working implementation for Grid + binary_tree
    // let mut grid = Grid {
    //     rows: 10,
    //     columns: 10,
    //     cells: Vec::new(),
    // };
    // let links_are_bidirectional = true;
    // grid.cells = grid.prepare_grid();
    // grid.configure_cells();
    // grid = binary_tree(grid, links_are_bidirectional);
    // display_maze(&grid);let links_are_bidirectional = true;

    let mut grid = SmartGrid {
            rows: 10,
            columns: 10,
            cells: Vec::new(),
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid = binary_tree(grid, links_are_bidirectional);

}
