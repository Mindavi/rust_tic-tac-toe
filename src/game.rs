use player;
use field;
use winchecker;

pub struct Game {
    player1: player::Player,
    player2: player::Player,
    field: field::Field,
    current_player: u8,
}

pub enum GameState {
    Running,
    Winner(String),
    Tie,
}

impl Game {
    pub fn new(player1_name: &str, player2_name: &str) -> Game {
        Game {
            player1: player::Player {
                name: player1_name.to_string(),
                field_type: field::FieldState::Circle,
            },
            player2: player::Player {
                name: player2_name.to_string(),
                field_type: field::FieldState::X,
            },
            field: field::Field::new(),
            current_player: 1,
        }
    }

    pub fn get_current_player(&self) -> &player::Player {
        match self.current_player {
            1 => return &self.player1,
            2 => return &self.player2,
            _ => panic!("only 2 players"),
        };
    }

    pub fn do_move(&mut self, x: usize, y: usize) -> Result<GameState, &'static str> {
        // FIXME: same as get_current_player, but putting that here gives issues with the borrow checker
        let player = match self.current_player {
            1 => &self.player1,
            2 => &self.player2,
            _ => panic!("only 2 players"),
        };

        match self.field.get(x, y).unwrap() {
            field::FieldState::Empty => {
                self.field.set(x, y, player.field_type);
            },
            _ => {
                return Err("This field is already filled");
            },
        }

        match self.current_player {
            1 => self.current_player = 2,
            2 => self.current_player = 1,
            _ => panic!("invalid player selected by game"),
        }

        if winchecker::is_winning_turn(x, y, &self.field) {
            return Ok(GameState::Winner(player.name.to_string()));
        }
        if winchecker::is_game_done(&self.field) {
            return Ok(GameState::Tie);
        }

        return Ok(GameState::Running);
    }

    pub fn print(&self) {
        self.field.print();
    }
}
