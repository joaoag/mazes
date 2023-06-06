use crate::location::Location;
use crate::direction::Direction;

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Cell {
    pub location: Location,
    pub north: Option<Location>,
    pub east: Option<Location>,
    pub south: Option<Location>,
    pub west: Option<Location>,
    pub links: Vec<Location>,
}


impl Cell {
    // pub fn link(&mut self, target_cell: Location) {
    //     self.links.push(target_cell);
    // }
    pub fn empty(row: usize, column: usize) -> Self {
        Cell {
            location: Location { row, column },
            ..Default::default()
        }
    }
    pub fn is_linked(&self, direction: Direction) -> bool {
        if self.links.is_empty() {
            return false;
        }
        match direction {
            Direction::North if self.north.is_some() => self.links.contains(&self.north.unwrap()),
            Direction::East if self.east.is_some() => self.links.contains(&self.east.unwrap()),
            Direction::South if self.south.is_some() => self.links.contains(&self.south.unwrap()),
            Direction::West if self.west.is_some() => self.links.contains(&self.west.unwrap()),
            _ => false,
        }
    }
}