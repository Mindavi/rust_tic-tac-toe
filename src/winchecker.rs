use field;

pub fn is_winning_turn(x: usize, y: usize, field: &field::Field) -> bool {
    let turn_type = match field.get(x, y) {
        Ok(turn_type) => turn_type,
        Err(e) => {
            println!("{}", e);
            return false;
        },
    };
    return check_row(y, field, turn_type) || check_column(x, field, turn_type) || check_diagonal(x, y, field, turn_type);
}

fn check_row(y: usize, field: &field::Field, turn_type: field::FieldState) -> bool {
    for column_index in 0..field::FIELD_WIDTH {
        let field_type = field.get(column_index, y).expect("expected valid y for this function");
        if field_type != turn_type {
            return false;
        }
    }
    return true;
}

fn check_column(x: usize, field: &field::Field, turn_type: field::FieldState) -> bool {
    for row_index in 0..field::FIELD_HEIGHT {
        let field_type = field.get(x, row_index).expect("expected valid x for this function");
        if field_type != turn_type {
            return false;
        }
    }
    return true;
}

fn is_corner_or_middle(x: usize, y: usize) -> bool {
    return (x % 2 == 0 && y % 2 == 0) || (x == 1 && y == 1);
}

fn check_diagonal_left(field: &field::Field, turn_type: field::FieldState) -> bool {
    let mut column_index = 0;
    let mut row_index = 0;
    while column_index < field::FIELD_WIDTH || row_index < field::FIELD_HEIGHT {
        if field.get(column_index, row_index).unwrap() != turn_type {
            return false;
        }
        column_index += 1;
        row_index += 1;
    }
    return true;
}

fn check_diagonal_right(field: &field::Field, turn_type: field::FieldState) -> bool {
    let mut column_index = field::FIELD_WIDTH - 1;
    let mut row_index = 0;
    loop {
        if field.get(column_index, row_index as usize).unwrap() != turn_type {
            return false;
        }
        if row_index >= field::FIELD_HEIGHT - 1 || column_index == 0 {
            break;
        }
        column_index -= 1;
        row_index += 1;
    }
    return true;
}

fn check_diagonal(x: usize, y: usize, field: &field::Field, turn_type: field::FieldState) -> bool {
    if !is_corner_or_middle(x, y) {
        return false;
    }
    return check_diagonal_left(field, turn_type) || check_diagonal_right(field, turn_type);
}

pub fn is_game_done(field: &field::Field) -> bool {
    for row_index in 0..field::FIELD_HEIGHT {
        for column_index in 0..field::FIELD_WIDTH {
            let field_type = field.get(column_index, row_index).unwrap();
            if field_type == field::FieldState::Empty {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_or_middle() {
        let corners: [(usize, usize); 4] = [(0,0), (0,2), (2,0), (2,2)];
        for corner in corners.iter() {
            let (x, y) = corner;
            assert!(is_corner_or_middle(*x, *y) == true);
        }
        let middle_x = 1;
        let middle_y = 1;
        assert!(is_corner_or_middle(middle_x, middle_y) == true);
    }
}