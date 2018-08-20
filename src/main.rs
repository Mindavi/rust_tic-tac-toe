use std::io;
mod field;

fn number_to_coordinate(number: usize, num_rows: usize) -> Result<(usize, usize), &'static str> {
    if number <= 0 || number > field::FIELD_HEIGHT * field::FIELD_WIDTH {
        return Err("Invalid number given");
    }
    let index = number - 1;
    let y = index / num_rows;
    let x = index - (y * num_rows);
    return Ok((x, y));
}

fn main() {
    let mut field = field::Field::new();
    let stdin = io::stdin();
    let num_rows = field::FIELD_WIDTH;

    loop {
        println!("Player 1: where to put an X?");
        let mut input = String::new();
        stdin.read_line(& mut input).unwrap();

        let selected = match input.trim_right().parse::<usize>() {
            Ok(selected) => selected,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
        };
        let (x,y) = match number_to_coordinate(selected, num_rows) {
            Ok(result) => (result),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
        };
        field.set(x, y, field::FieldState::Circle);
        field.print();
    }
}
