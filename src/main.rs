use crate::grid::Grid;
use crate::display::display_maze;
use crate::binary_tree::binary_tree;

mod grid;
mod cell;
mod display;
mod binary_tree;
mod location;
mod direction;

fn main() {
    let mut grid = Grid {
        rows: 10,
        columns: 10,
        cells: Vec::new(),
    };
    let links_are_bidirectional = true;
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    grid = binary_tree(grid, links_are_bidirectional);
    display_maze(&grid);
}
