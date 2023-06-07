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

            }
        }
    }
    grid
}
