use crate::direction::Direction;
use crate::location::Location;

#[derive(Eq, PartialEq, Debug, Default)]
pub struct MazeCell {
    pub location: Location,
    pub north: Option<Location>,
    pub east: Option<Location>,
    pub south: Option<Location>,
    pub west: Option<Location>,
    pub links: Vec<Location>,
}

impl MazeCell {
    pub fn link(&mut self, linked_to: Location) {
        // MVP, update a cell with a target
        // v 1.0, also update target cell
        self.links.push(linked_to);
    }

    pub fn empty(row: usize, column: usize) -> Self {
        MazeCell {
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
