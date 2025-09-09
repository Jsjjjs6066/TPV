use crossterm::style::ContentStyle;
use serde_json::Value;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element};

use std::sync::LazyLock;

pub static LINE: LazyLock<Element> = LazyLock::new(|| 
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16)| {
            let char: String = args.get(0).unwrap_or(&Value::String("─".to_string()))
            .as_str().unwrap_or("─").chars().last().unwrap_or('─')
            .to_string();

            if char == "\n" {
                return Content::new(vec![Text::new_default("\n".to_string())], false, AdjustXAxisOptions::None, (parent_size.0, 1));
            }
            if char == " " {
                return Content::new(vec![Text::new_default(String::new())], false, AdjustXAxisOptions::None, (parent_size.0, 1));
            }
            if char == "-" {
                return Content::new(vec![Text::new_default("─".to_string().repeat(parent_size.0 as usize))],
                 true, 
                 AdjustXAxisOptions::None, (parent_size.0, 1));
            }
            Content::new(vec![Text::new_default(char.repeat(parent_size.0 as usize))], true, AdjustXAxisOptions::None, (parent_size.0, 1))
        },
    )
);