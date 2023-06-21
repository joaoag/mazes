extern crate rand;

use crate::maze::{display_maze, SmartGrid};
use crate::maze_makers::{binary_tree, side_winder};
use crate::maze_solvers::dijkstra_simplified_solver as solver;

mod maze;
mod maze_makers;
mod maze_solvers;

fn main() {
    let mut grid = SmartGrid {
        rows: 4,
        columns: 4,
        cells: Vec::new(),
    };
    grid.cells = grid.prepare_grid();
    grid.configure_cells();
    // grid = binary_tree(grid);
    grid = side_winder(grid);

    grid = solver(grid);
    display_maze(&grid);
}

//TODO - how to reorganise overly small files into sensible modules?
// chuck everything maze structure + display related in a maze.rs file?
// maze making algos in maze_making_algorithms.rs?
// maze solving algos in maze_solving_algorithms.rs?
// Yeah, that seems sensible
