use field;

pub fn is_winning_turn(x: usize, y: usize, field: &field::Field) -> bool {
    let turn_type = match field.get(x, y) {
        Ok(turn_type) => turn_type,
        Err(e) => {
            println!("{}", e);
            return false;
        },
    };
    return check_row(y, field, turn_type) || check_column(x, field, turn_type);
}

fn check_row(y: usize, field: &field::Field, turn_type: field::FieldState) -> bool {
    let mut result = true;
    for column_index in 0..field::FIELD_WIDTH {
        let field_type = field.get(column_index, y).expect("expected valid y for this function");
        if field_type != turn_type {
            result = false;
            break;
        }
    }
    return result;
}

fn check_column(x: usize, field: &field::Field, turn_type: field::FieldState) -> bool {
    let mut result = true;
    for row_index in 0..field::FIELD_HEIGHT {
        let field_type = field.get(x, row_index).expect("expected valid x for this function");
        if field_type != turn_type {
            result = false;
            break;
        }
    }
    return result;
}