use std::io::{stdin, stdout, Write};
mod field;
mod player;
mod winchecker;
mod game;

fn number_to_coordinate(number: usize, num_rows: usize) -> Result<(usize, usize), &'static str> {
    if number <= 0 || number > field::FIELD_HEIGHT * field::FIELD_WIDTH {
        return Err("Invalid number given");
    }
    let index = number - 1;
    let y = index / num_rows;
    let x = index - (y * num_rows);
    return Ok((x, y));
}

fn ask_player_names() -> (String, String) {
    let stdin = stdin();

    let mut player1_name = String::new();
    print!("Input a name for player 1: ");
    stdout().flush().unwrap();
    stdin.read_line(& mut player1_name).unwrap();
    player1_name = player1_name.trim_right().to_string();

    let mut player2_name = String::new();
    print!("Input a name for player 2: ");
    stdout().flush().unwrap();
    stdin.read_line(& mut player2_name).unwrap();
    player2_name = player2_name.trim_right().to_string();

    if player1_name == "" {
        player1_name = "1".to_string();
    }
    if player2_name == "" {
        player2_name = "2".to_string();
    }

    return (player1_name, player2_name);
}

fn main() {
    let stdin = stdin();
    let num_rows = field::FIELD_WIDTH;

    let (player1_name, player2_name) = ask_player_names();

    loop {
        println!();
        println!();
        let mut game = game::Game::new(&player1_name, &player2_name);
        loop {
            game.print();
            println!();
            {
                let player = game.get_current_player();
                print!("{0}: input number between 1-9 for {1}: ", player.name, player.field_type);
                stdout().flush().unwrap();
            }
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

            match game.do_move(x, y) {
                Ok(game::GameState::Running) => {},
                Ok(game::GameState::Winner(winner)) => {
                    game.print();
                    println!();
                    println!("{} is the winner!", winner);
                    break;
                }
                Ok(game::GameState::Tie) => {
                    println!("It's a tie.");
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
