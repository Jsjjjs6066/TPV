use std::cmp::min;
use crossterm::style::Color;
use figlet_rs::FIGfont;
use serde_json::Value;
use std::sync::LazyLock;

use crate::tprl::{content::{AdjustXAxisOptions, Content, Text}, element::Element};

pub static HEADING: LazyLock<Element> = LazyLock::new(||
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16)| {
            let font: FIGfont = FIGfont::standard().unwrap();
            
            Content::new(
                vec![
                    Text::new(
                        font.convert(
                        args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string(),
                        Color::Reset,
                        Color::Reset
                    )
                ],
                false,
                AdjustXAxisOptions::None,
                (font.convert(
                    args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string().lines().nth(0).unwrap_or("").chars().count() as u16,
                 font.convert(
                    args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string().lines().count() as u16)
            )
        },
    )
);