use cell::Cell;

pub fn display_maze(grid: &Grid) {
    let start = String::from("+");
    let middle = String::from("---+".repeat(grid.columns));
    let end = String::from("\n");
    let mut output = format!("{}{}{}", start, middle, end);

    // TODO work out if there's a cleaner and/or faster way to handle the string concatenation
    for row in grid.cells.iter() {
        let mut top = String::from("|");
        let mut bottom = String::from("+");

        for cell in row.iter() {
            let body = "   ";
            let east_boundary = if Cell::is_linked(&cell, Direction::East) {
                " "
            } else {
                "|"
            };

            top.push_str((body.to_owned() + east_boundary).as_str());

            let south_boundary = if Cell::is_linked(&cell, Direction::South) {
                "   "
            } else {
                "---"
            };
            let corner = "+";
            bottom.push_str((south_boundary.to_owned() + corner).as_str());
        }
        output.push_str((top.to_owned() + "\n").as_str());
        output.push_str((bottom.to_owned() + "\n").as_str());
    }

    println!("{}", output);
}