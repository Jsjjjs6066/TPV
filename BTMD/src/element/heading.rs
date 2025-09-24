use crossterm::style::Color;
use figlet_rs::FIGfont;
use serde_json::Value;
use {std::sync::LazyLock, usize};

use crate::{
    content::{Content, Text},
    element::Element,
};

pub static HEADING: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32| {
            let font: FIGfont = FIGfont::standard().unwrap();
            let heading: String = font
                .convert(
                    args.first()
                        .unwrap_or(&Value::String("".to_string()))
                        .as_str()
                        .unwrap(),
                )
                .unwrap()
                .to_string();

            let mut width: u16 = heading.lines().max_by(|a, b| a.len().cmp(&b.len())).unwrap_or("").chars().count() as u16;
            let height: u16 = heading.lines().count() as u16;
            
            let returned_heading: String = if width > parent_size.0 {
                let new_heading = heading;
                new_heading.lines().map(|s| {
                    let mut temp: String = String::new();
                        for i in 0..parent_size.0 {
                            temp.push(s.chars().nth((i as u32 + (timer / 4) % (width - parent_size.0 / 2) as u32) as usize).unwrap_or(' '));
                        }
                    temp
                }).collect::<Vec<String>>().join("\n") + "\n"
                    // .lines()
                    // .map(|line| {
                    //     line[(timer / 5 % (line.len() as u32)) as usize
                    //         ..(min(
                    //             line.len(),
                    //             (timer / 5 % (parent_size.0 as u32) + (parent_size.0 as u32)) as usize,
                    //         ))]
                    //         .to_string()
                    // })
                    // .collect::<Vec<String>>()
                    // .join("\n")
                    // + "\n"
                
            }
            else {
                heading
            };
            width = returned_heading.lines().max_by(|a, b| a.len().cmp(&b.len())).unwrap_or("").chars().count() as u16;
            Content::new(
                vec![Text::new(returned_heading, Color::Reset, Color::Reset)],
                false,
                (
                    width,
                    height,
                ),
            )
        },
        "heading".to_string(),
    )
});
