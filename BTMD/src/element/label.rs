use std::cmp::min;
use crossterm::style::Color;
use serde_json::Value;
use std::sync::LazyLock;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element};

pub static LABEL: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16)| -> Content {
            let text: String = args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string();
            Content::new(
                vec![Text::new(
                    text.clone(),
                    Color::Reset,
                    Color::Reset,
                )],
                false,
                AdjustXAxisOptions::None,
                (min(text.chars().count() as u16, parent_size.0), text.lines().count() as u16),
            )
        }, "label".to_string()
    )
});