use std::io;
mod field;

fn number_to_coordinate(number: usize) -> (usize, usize) {
    match number {
        1 => (0,0),
        2 => (1,0),
        3 => (2,0),
        4 => (0,1),
        5 => (1,1),
        6 => (2,1),
        7 => (0,2),
        8 => (1,2),
        9 => (2,2),
        _ => (10,10),
    }
}

fn main() {
    let mut field = field::Field::new();
    let stdin = io::stdin();

    loop {
        println!("Player 1: where to put an X?");
        let mut input = String::new();
        stdin.read_line(& mut input).unwrap();

        let selected = input.trim_right().parse::<usize>().unwrap();
        let (x,y) = number_to_coordinate(selected);
        field.set(x, y, field::FieldState::Circle);
        field.print();
    }
}
