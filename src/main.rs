use std::io;
// mod field;

#[derive(Clone, Copy)]
enum FieldState {
    Empty,
    Circle,
    X,
}

fn main() {
    let mut play_field: [[FieldState; 3]; 3] = [[FieldState::Empty; 3]; 3];

    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        stdin.read_line(& mut input);

        input = input.trim_right().to_string();
        let mut selected = 0;
        match input.as_str() {
            "1" => selected = 1,
            "5" => selected = 5,
            _ => println!("*"),
        }

        play_field[selected][0] = FieldState::Circle;

        for row in play_field.iter() {
            println!();
            for point in row.iter() {
                let mut result: char = '*';
                match point {
                    FieldState::Empty => result = '-',
                    FieldState::Circle => result = 'O',
                    FieldState::X => result = 'X',
                }
                print!("{} ", result); 
            }
            println!();
        }
    }
}
