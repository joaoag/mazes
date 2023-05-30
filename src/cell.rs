use std::collections::HashMap;

#[derive(Debug)]
pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: HashMap<Box<Cell>, bool>
}


fn main(){
    let cell_one: Cell = Cell {
        row: 1,
        column: 1,
        north:  None::<Box<Cell>>,
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: HashMap::new()
    };
    let cell_two: Cell = Cell{
        row: 0,
        column: 0,
        north: Some(Box::new(cell_one)),
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: HashMap::new()
    };
    let cell_three: Cell = Cell{
        row: 0,
        column: 1,
        north: None::<Box<Cell>>,
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>,
        links: HashMap::new()
    };

    cell_two.links.insert(Box::new(cell_three), true);
    println!("{:#?}", cell_two);
}
