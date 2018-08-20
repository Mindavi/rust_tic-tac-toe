#[derive(Clone, Copy)]
pub enum FieldState {
    Empty,
    Circle,
    X,
}

pub const FIELD_WIDTH: usize = 3;
pub const FIELD_HEIGHT: usize = 3;

pub struct Field {
    field: [[FieldState; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl Field {
    pub fn new() -> Field {
        Field {
            field: [[FieldState::Empty; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_state: FieldState) {
        if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
            return;
        }
        self.field[y][x] = new_state;
    }

    pub fn get(&self, x: usize, y: usize) -> Result<FieldState, &'static str> {
        if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
            return Err("Invalid index");
        }
        return Ok(self.field[y][x]);
    }

    pub fn print(&self) {
        for row in self.field.iter() {
            println!();
            for point in row.iter() {
                let mut result: char;
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