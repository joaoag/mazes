use crate::cell::MazeCell;
use crate::location::Location;
use crate::smart_grid::SmartGrid;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::IndexMut;

pub fn dijkstra_simplified_solver(mut grid: SmartGrid) -> SmartGrid {
    let start_location = Location { row: 0, column: 0 };
    let mut distance: usize = 0;
    let mut frontier: HashSet<_> = HashSet::from([start_location]);
    let mut visited: HashSet<Location> = HashSet::new();

    while !frontier.is_empty() {
        let mut next_frontier: HashSet<Location> = HashSet::new();

        for location in frontier.iter().copied() {
            let Location { row, column } = location;
            let mut current_cell = grid.cells.index_mut(row).index_mut(column).borrow_mut();

            current_cell.distance = distance;
            next_frontier.extend(Vec::from_iter(current_cell.links.iter()));
            visited.extend([location]);
        }
        frontier.clear(); // unsure if this is necessary
        let to_explore: HashSet<_> = next_frontier.difference(&visited).collect();
        frontier.extend(to_explore);
        distance += 1;
    }

    grid
}