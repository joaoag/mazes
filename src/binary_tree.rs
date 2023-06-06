use rand::Rng;
use std::ops::IndexMut;

use crate::location::Location;
use crate::Grid;

#[derive(Debug)]
struct Link {
    source: Location,
    target: Location,
}

fn binary_tree_random_neighbour(eastern: Location, northern: Location) -> Location {
    let mut neighbours: Vec<Location> = vec![];
    neighbours.extend([eastern, northern]);

    let index = rand::thread_rng().gen_range(0..=1);
    let linked_neighbour = neighbours[index];
    linked_neighbour
}

pub fn binary_tree(mut grid: Grid, bidirectional_link: bool) -> Grid {
    let mut links: Vec<Link> = vec![];

    for row in grid.cells.iter_mut() {
        for cell in row.iter_mut() {
            let is_northmost_cell = cell.north.is_none();
            let is_eastmost_cell = cell.east.is_none();
            let is_north_eastern_cell = is_northmost_cell & is_eastmost_cell;

            // TODO investigate smart pointers for bi-directional updates

            if is_north_eastern_cell {
                break;
            } else if is_northmost_cell {
                let eastern_location = cell.east.unwrap();
                cell.links.push(eastern_location);
                links.push(Link {
                    source: cell.location,
                    target: eastern_location,
                });
            } else if is_eastmost_cell {
                let northern_location = cell.north.unwrap();
                cell.links.push(northern_location);
                links.push(Link {
                    source: cell.location,
                    target: northern_location,
                });
            } else {
                let linked_neighbour =
                    binary_tree_random_neighbour(cell.east.unwrap(), cell.north.unwrap());

                cell.links.push(linked_neighbour);
                links.push(Link {
                    source: cell.location,
                    target: linked_neighbour,
                });
            }
        }
    }

    if bidirectional_link {
        for link in links.iter() {
            let Link { source, target } = link;

            let target_cell = grid.cells.index_mut(target.row).index_mut(target.column);
            target_cell.links.push(*source);
        }
    }

    grid
}
