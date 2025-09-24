#[derive(Clone, Debug, Default)]
pub struct Cursor {
    pub position: (u16, u16),
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { position: (0, 0) }
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.position = (x, y);
    }

    pub fn get_position(&self) -> &(u16, u16) {
        &self.position
    }

    pub fn move_up(&mut self, steps: u16) {
        if self.position.1 < steps {
            self.position.1 = 0;
        } else {
            self.position.1 -= steps;
        }
    }
    pub fn move_down(&mut self, steps: u16) {
        if self.position.1 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).1 {
            self.position.1 = crossterm::terminal::size().unwrap_or((0, 0)).1 - 1;
        } else {
            self.position.1 += steps;
        }
    }

    pub fn move_left(&mut self, steps: u16) {
        if self.position.0 < steps {
            self.position.0 = 0;
        } else {
            self.position.0 -= steps;
        }
    }
    pub fn move_right(&mut self, steps: u16) {
        if self.position.0 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).0 {
            self.position.0 = crossterm::terminal::size().unwrap_or((0, 0)).0 - 1;
        } else {
            self.position.0 += steps;
        }
    }
}
