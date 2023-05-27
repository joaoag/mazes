use std::collections::HashMap;

#[derive(Debug)]
pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: HashMap<Cell, bool>
}


// these probably need to be implemented as traits - might be useful for a blog post?

fn link(source: Cell, target: Cell){
    source.links.insert(
        target,
        true
    );
    target.links.insert(
        source,
        true
    )
    // not sure what's going to happen with ownership here
}

fn unlink(source: Cell, target: Cell){
    source.links.remove(
        target
    );
    target.links.remove(
        source
    );
    // same here - who owns what???
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
    println!("{:#?}", cell_two);
}


// next step, look into #[derive(Eq, Hash, PartialEq)]
// why do you need to derive it?
// why would it not be implemented by default?
