extern crate rand;

use crate::binary_tree::binary_tree;
use crate::dijkstra_simplified_solver::dijkstra_simplified_solver as solver;
use crate::display::display_maze;
use crate::side_winder::side_winder;
use smart_grid::SmartGrid;

mod binary_tree;
mod cell;
mod dijkstra_simplified_solver;
mod direction;
mod display;
mod link;
mod location;
mod side_winder;
mod smart_grid;

fn main() {
    let mut grid = SmartGrid {
        rows: 4,
        columns: 4,
        cells: Vec::new(),
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid = binary_tree(grid);
    // grid = side_winder(grid);

    grid = solver(grid);
    display_maze(&grid);
}
