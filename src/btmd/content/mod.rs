pub mod content_builder;
pub mod text;

use std::cell::RefCell;

pub use content_builder::ContentBuilder;
pub use text::Text;

use crate::btmd;

use btmd::element::Element;

#[derive(Clone)]
pub struct Content {
    pub text: Vec<Text>,
    pub rerender_needed: bool,
    pub size: (u16, u16),
    current_text_index: usize,
    current_char_index: usize,
    pub position: Option<(u16, u16)>,
    pub holder: RefCell<Element>,
}

impl Content {
    pub fn new(text: Vec<Text>, rerender_needed: bool, size: (u16, u16), holder: RefCell<Element>) -> Content {
        Content {
            text,
            rerender_needed,
            size,
            current_text_index: 0,
            current_char_index: 0,
            position: None,
            holder,
        }
    }

    pub fn render(&self) -> String {
        self.text
            .iter()
            .map(|content_type: &Text| content_type.render())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Iterator for Content {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check if current_text_index is out of bounds
            if self.current_text_index >= self.text.len() {
                return None;
            }

            // Get the current text element
            let current_text = &self.text[self.current_text_index];
            let chars: Vec<char> = current_text.text.chars().collect();

            // Check if current_char_index is within the current text's characters
            if self.current_char_index < chars.len() {
                let ch = chars[self.current_char_index];
                self.current_char_index += 1;
                return Some(ch);
            } else {
                // Move to the next text element
                self.current_text_index += 1;
                self.current_char_index = 0;
            }
        }
    }
}
