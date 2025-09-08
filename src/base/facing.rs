#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn facing(&self) -> Facing {
        self.clone()
    }
}