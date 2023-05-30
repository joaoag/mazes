use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug, Default)]
// need to understand why Eq and PartialEq were required
pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: Box<HashMap<Box<Cell>, bool>>
    //     ^ why does the whole thing need to be in a Box?
    //     is it due to the recursive data type?
}


impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}
//

fn main(){
    let cell_one: Cell = Cell {
        row: 1,
        column: 1,
        north:  None::<Box<Cell>>,
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: Box::new(HashMap::new())
    };
    let mut cell_two: Cell = Cell{
        row: 0,
        column: 0,
        north: Some(Box::new(cell_one)),
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: Box::new(HashMap::new())
    };
    let cell_three: Cell = Cell{
        row: 0,
        column: 1,
        north: None::<Box<Cell>>,
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: Box::new(HashMap::new())
    };

    cell_two.links.insert(Box::new(cell_three), true);
    println!("{:#?}", cell_two);
}
