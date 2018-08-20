use field;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub field_type: field::FieldState,
}
