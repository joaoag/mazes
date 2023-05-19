pub struct Cell {
    row: u8,
    column: u8,
    north: Option<Cell>,
    east: Option<Cell>,
    south: Option<Cell>,
    west: Option<Cell>,
}

