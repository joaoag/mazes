// we need north east south west so a cell knows of its neighbours
// however, we cannot declare the type as Cell
// this is because not all Cells will have Cells as neighbours
// for example, a Cell at the edge of a maze will only have three Cell neighbours
//

pub struct Cell {
    row: u8,
    column: u8,
    north: Cell,
    east: Cell,
    south: Cell,
    west: Cell
}
