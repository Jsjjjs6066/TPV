use crossterm::style::Color;
use figlet_rs::FIGfont;
use serde_json::Value;
use std::{cmp::min, sync::LazyLock};

use crate::{content::{Content, Text}, element::Element};

pub static HEADING: LazyLock<Element> = LazyLock::new(||
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32| {
            let font: FIGfont = FIGfont::standard().unwrap();
            let heading: String = font.convert(
                        args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string();
            let returned_heading: String = heading.lines().map(|line| {
                line[(timer / 5 % (parent_size.0 as u32)) as usize..(min(line.len(), (timer / 5 % (parent_size.0 as u32) + (parent_size.0 as u32)) as usize))].to_string()
            }).collect::<Vec<String>>().join("\n") + "\n";
            
            Content::new(
                vec![
                    Text::new(
                        returned_heading,
                        Color::Reset,
                        Color::Reset
                    )
                ],
                false,
                (font.convert(
                    args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string().lines().nth(0).unwrap_or("").chars().count() as u16,
                 font.convert(
                    args.get(0)
                        .unwrap_or(&Value::String("".to_string())).
                        as_str().unwrap()).unwrap().to_string().lines().count() as u16)
            )
        }, "heading".to_string()
    )
);