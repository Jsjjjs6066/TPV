use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

pub struct Text {
    pub text: String,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Text {
    pub fn new(text: String, foreground_color: Color, background_color: Color) -> Self {
        Text {
            text,
            foreground_color,
            background_color,
        }
    }
    pub fn new_default(text: String) -> Self {
        Text {
            text,
            foreground_color: Color::Reset,
            background_color: Color::Reset,
        }
    }

    pub fn render(&self) -> String {
        format!(
            "{}{}{}",
            SetForegroundColor(self.foreground_color),
            self.text,
            SetBackgroundColor(self.background_color)
        )
    }
}