use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

#[derive(Clone, Debug)]
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
            "{}{}{}{}{}",
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
            self.text,
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset),
        )
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {text: Default::default(), foreground_color: Color::Reset, background_color: Color::Reset}
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = char;
    type IntoIter = std::str::Chars<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.text.chars()
    }
}

impl<'a> std::iter::Sum<&'a Text> for Text {
    fn sum<I: Iterator<Item = &'a Text>>(iter: I) -> Self {
        let mut text = String::new();
        for t in iter {
            text.push_str(&t.text);
        }
        Text::new_default(text)
    }
}