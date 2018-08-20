use std::io;
mod field;
mod player;
mod winchecker;

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
    let stdin = io::stdin();
    let num_rows = field::FIELD_WIDTH;
    let player1 = player::Player {
        player_name: "1".to_string(),
        field_type: field::FieldState::Circle,
    };

    let player2 = player::Player {
        player_name: "2".to_string(),
        field_type: field::FieldState::X,
    };

    loop {
        let mut current_player = &player1;
        let mut field = field::Field::new();
        println!("New game started with {} and {}", player1.player_name, player2.player_name);

        loop {
            field.print();
            println!();
            println!("{0}: where to put an {1}?", current_player.player_name, current_player.field_type);
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

            match field.get(x, y).unwrap() {
                field::FieldState::Empty => {
                    field.set(x, y, current_player.field_type);
                },
                _ => {
                    println!("This field is already filled");
                    continue;
                },
            }

            if winchecker::is_winning_turn(x, y, &field) {
                field.print();
                println!();
                println!("{} has won!", current_player.player_name);
                break;
            }
            if winchecker::is_game_done(&field) {
                field.print();
                println!();
                println!("It's a tie.");
                break;
            }

            if current_player as *const _ == &player1 {
                current_player = &player2;
            } else {
                current_player = &player1;
            }
        }
    }
}
