// use std::collections::VecDeque;

// use crate::page::Page;

#[derive(Clone, Debug, Default)]
pub struct Cursor {
    pub position: (u16, u16),
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { position: (0, 0) }
    }

    pub fn get_position(&self) -> &(u16, u16) {
        &self.position
    }
}
