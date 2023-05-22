#[derive(Debug)]
pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
}

fn main(){
    let cell_one: Cell = Cell {
        row: 1,
        column: 1,
        north:  None::<Box<Cell>>,
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>
    };
    let cell_two: Cell = Cell{
        row: 0,
        column: 0,
        north: Some(Box::new(cell_one)),
        east: None::<Box<Cell>>,
        south: None::<Box<Cell>>,
        west: None::<Box<Cell>>
    };
    println!("{:#?}", cell_two);
}