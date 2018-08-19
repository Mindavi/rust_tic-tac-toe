#[derive(Clone, Copy)]
pub enum FieldState {
    Empty,
    Circle,
    X,
}

pub struct Field {
    field: [[FieldState; 3]; 3],
}

impl Field {
    pub fn new() -> Field {
        Field {
            field: [[FieldState::Empty; 3]; 3],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_state: FieldState) {
        if x >= 3 || y >= 3 {
            return;
        }
        self.field[y][x] = new_state;
    }

    pub fn get(&self, x: usize, y: usize) -> Result<FieldState, &'static str> {
        if x >= 3 || y >= 3 {
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