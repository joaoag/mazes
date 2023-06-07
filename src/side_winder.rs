use crate::link::Link;
use crate::location::Location;
use crate::rand::Rng;
use crate::smart_grid::SmartGrid;
use rand::seq::SliceRandom;

pub fn side_winder(grid: SmartGrid) -> SmartGrid {
    for row in &grid.cells {
        let mut run: Vec<Location> = Vec::new();

        for cell in row {
            let cell = cell.borrow_mut();
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let zero_or_one = rand::thread_rng().gen_range(0..=1);
            let should_close_run = is_eastmost_cell || (!is_northmost_cell & (zero_or_one == 0));

            run.push(cell.location.clone());

            if should_close_run {
                let member_location = run.choose(&mut rand::thread_rng()).unwrap();
                // if member_location is current cell
                // leave cell as is
                // else
                // re-assign cell to member location (hopefully freeing up original borrow?)
                let mut member_cell;
                if member_location == &cell.location {
                    member_cell = cell;
                } else {
                    member_cell =
                        grid.cells[member_location.row][member_location.column].borrow_mut();
                }

                if !is_northmost_cell {
                    let northern_location = member_cell.north.unwrap();
                    member_cell.links.push(northern_location);

                    // assuming bidirectional links
                    let mut target_cell =
                        grid.cells[northern_location.row][northern_location.column].borrow_mut();
                    target_cell.links.push(member_cell.location);

                    run.clear();
                }
            }
        }
    }
    grid
}
