use crossterm::style::Color;

use crate::tprl::content::{AdjustXAxisOptions, Content, Text};

pub struct ContentBuilder {
    pub content: Vec<Text>
}

impl ContentBuilder {
    pub fn new() -> Self {
        ContentBuilder { content: Vec::new() }
    }

    pub fn append_text(&mut self, text: String, foreground_color: Color, background_color: Color) {
        self.content.push(Text::new(text, foreground_color, background_color))
    }
    pub fn append_text_default(&mut self, text: String) {
        self.content.push(Text::new_default(text))
    }

    pub fn build(self, rerender_needed: bool, adjust_x_axis_options: AdjustXAxisOptions, size: (u16, u16)) -> Content {
        Content {
            text: self.content,
            rerender_needed: rerender_needed,
            adjust_x_axis: adjust_x_axis_options,
            size,
        }
    }
}