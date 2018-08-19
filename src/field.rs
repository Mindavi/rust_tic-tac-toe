#[derive(Clone, Copy)]
enum FieldState {
    Empty,
    Circle,
    X,
}

pub struct Field {
    field: [[FieldState; 3]; 3] = [[FieldState::Empty; 3]; 3];
}

impl Field {
    pub fn set(&mut self, x: u8, y: u8) {
        if 
    }
}