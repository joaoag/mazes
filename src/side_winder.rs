use crate::grid::Grid;
use crate::link::Link;
use crate::location::Location;
use rand::seq::SliceRandom;
use crate::rand::Rng;

pub fn side_winder(mut grid: Grid) -> Grid {
    for row in grid.cells.iter_mut() {
        let mut run: Vec<Location> = vec![];

        for cell in row.iter_mut() {
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let zero_or_one = rand::thread_rng().gen_range(0..=1);

            let should_close_run = is_eastmost_cell || (!is_northmost_cell & (zero_or_one == 0));
            if should_close_run {
                let member = run.choose(&mut rand::thread_rng());
                // this algorithm is throwing up a problem for the approach I used in binary tree
                // In binary tree I was able to update the source cell's links at element access time
                // Then I would add link to a vec, which would be iterated over later to ensure
                // The target cell's links field was updated with the source cell's location
                // If I were to keep with that approach here I would be creating a links vec
                // Iterating over cells, adding ALL links to that links vec
                // Iterating over source and giving them their targets *then*
                // Iterating over targets and giving them their source
                // So that's three loops instead of one in the Ruby implementation
                // in the Ruby implementation you have access to all cells in the run collection

                // maybe I can try a grid with an Rc<RefCell> implementation
                // Read this https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
            }
        }
    }
    grid
}
