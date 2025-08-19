pub mod element;
pub mod page;
pub mod cursor;
pub mod content;

use std::fs::OpenOptions;
use std::io::Write;
use figlet_rs::FIGfont;
use crate::tprl::content::AdjustXAxisOptions;
use crate::tprl::content::Content;
use crate::tprl::element::registry;
use crate::tprl::element::Element;
use serde_json::{Map, Value};
use crate::parse::parse_vec_to_vec;
use crate::tprl::page::Page;

pub fn import_default_elements() {
    registry::register_element("none".to_string(), Element::new(|_, _, _| Content::new(String::new(), false, AdjustXAxisOptions::None), vec![]));
    
    registry::register_element("para".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            Content::new(
                args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string() + if args.len() > 0 {"\n"} else {""},
                false,
                AdjustXAxisOptions::None
            )
        },
        vec![],
    ));
    registry::add_alias("p".to_string(), "para");
    registry::add_alias("paragraph".to_string(), "para");

    registry::register_element("label".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            Content::new(
                args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string(),
                false,
                AdjustXAxisOptions::None
            )
        },
        vec![Value::String("text".to_string())],
    ));
    registry::add_alias("l".to_string(), "label");

    registry::register_element("line".to_string(), Element::new(
        |_, args: Vec<Value>, parent_size: &(u16, u16)| {
            let char: String = args.get(0).unwrap_or(&Value::String("─".to_string()))
            .as_str().unwrap_or("─").chars().last().unwrap_or('─')
            .to_string();

            if char == "\n" {
                return Content::new("\n".to_string(), false, AdjustXAxisOptions::None);
            }
            if char == " " {
                return Content::new(String::new(), false, AdjustXAxisOptions::None);
            }
            if char == "-" {
                return Content::new("─".to_string().repeat(parent_size.0 as usize), true, AdjustXAxisOptions::None);
            }
            Content::new(char.repeat(parent_size.0 as usize), true, AdjustXAxisOptions::None)
        },
        vec![],
    ));

    registry::register_element("border".to_string(), Element::new(
        |page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)| {
            let mut default_config: Map<String, Value> = Map::new();
            default_config.insert("min-height".to_string(), Value::Number(0.into()));
            default_config.insert("connect-to-horizontal-chars".to_string(), Value::Bool(true));
            let config: Map<String, Value> = args.get(1).unwrap_or(&Value::Object(Map::new())).as_object().unwrap_or(&default_config).iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            for (k, v) in config.iter() {
                default_config.insert(k.clone(), v.clone());
            }

            if default_config.get("min-height").is_none() {
                default_config.insert("min-height".to_string(), Value::Number(0.into()));
            }
            else if default_config.get("min-height").unwrap().is_string() {
                if default_config.get("min-height").unwrap().as_str().unwrap() == "auto" {
                    default_config.insert("min-height".to_string(), Value::Number(0.into()));
                }
                else if default_config.get("min-height").unwrap().as_str().unwrap() == "max" {
                    default_config.insert("min-height".to_string(), Value::Number((parent_size.1 - 2).into()));
                }
            }

            let should_connect_to_horizontal_chars: bool = default_config.get("connect-to-horizontal-chars").unwrap_or(&Value::Bool(true)).as_bool().unwrap_or(true);

            let width: usize = parent_size.0 as usize;
            let horizontal_char: char = '─';
            let vertical_char: char = '│';
            let top_left: char = '┌';
            let top_right: char = '┐';
            let bottom_left: char = '└';
            let bottom_right: char = '┘';
            let mut border: String = String::new();
            border.push(top_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(top_right);
            border.push(vertical_char);

            let body_raw: Vec<Value> = args.get(0).unwrap_or(&Value::Array(vec![])).as_array().unwrap_or(&Vec::new()).to_vec();
            let mut body: Vec<Element> = parse_vec_to_vec(body_raw);

            let mut rendered_content: Vec<Content> = Vec::new();
            for element in body {
                rendered_content.push(element.render(page, &(parent_size.0 - 2, parent_size.1 - 2)));
            }

            let mut lines: u16 = 1;

            for c in rendered_content {
                for char in c.text.chars() {
                    if (border.chars().count() + 1) % width == 0 {
                        if std::env::args().any(|arg| arg == "--log") {
                            let mut file = OpenOptions::new().append(true).open("log.txt").unwrap();
                            file.write(border.chars().last().unwrap_or(' ').to_string().as_ref()).expect("TODO: panic message");
                            file.write(b"\n").expect("TODO: panic message");
                        }
                        if border.chars().last().unwrap_or(' ') == horizontal_char && should_connect_to_horizontal_chars {
                            border.push('┤');
                        }
                        else {
                            border.push(vertical_char);
                        }
                        border.push(vertical_char);
                        lines += 1;
                    }
                    if (border.chars().count() - 1) % width == 0 && char == horizontal_char && should_connect_to_horizontal_chars {
                        border.pop();
                        border.push('├');
                    }
                    if char == '\n' {
                        border.push_str(&*" ".repeat(width - 2 - (border.chars().count() - 1) % width));
                        border.push(vertical_char);
                        border.push(vertical_char);
                        lines += 1;
                    }
                    else {
                        border.push(char);
                    }
                }
            }

            if !(border.chars().last().unwrap() != '|') {
                for char in border.chars().rev().collect::<Vec<char>>() {
                    if char == ' ' {
                        border.pop();
                    } else {
                        break;
                    }
                }
                lines -= 1;
            }

            if (border.chars().count() - 1) % width == 0 {
                border.pop();
                lines -= 1;
            }

            if !(border.chars().count() % width == 0) {
                border.push_str(&*" ".repeat(width - 1 - border.chars().count() % width));
                if border.chars().last().unwrap_or(' ') == horizontal_char && should_connect_to_horizontal_chars {
                    border.push('┤');
                }
                else {
                    border.push(vertical_char);
                }
            }

            if lines < default_config.get("min-height").unwrap().as_u64().unwrap_or(0) as u16 {
                let additional_lines: u16 = default_config.get("min-height").unwrap().as_u64().unwrap_or(0) as u16 - lines;
                for _ in 0..additional_lines {
                    border.push(vertical_char);
                    border.push_str(&*" ".repeat(width - 2));
                    border.push(vertical_char);
                }
            }

            border.push(bottom_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);
            Content::new(border, false, AdjustXAxisOptions::None)
        },
        vec![],
    ));
    registry::add_alias("b".to_string(), "border");

    registry::register_element("heading".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            let font: FIGfont = FIGfont::standard().unwrap();
            
            Content::new(
                font.convert(args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap()).unwrap().to_string(),
                false,
                AdjustXAxisOptions::None
            )
        },
        vec![Value::String("text".to_string()), Value::Number(1.into())]
    ));
    registry::add_alias("h".to_string(), "heading");

    registry::register_element("new line".to_string(), Element::new(
        |_, _, _| Content::new("\n".to_string(), false, AdjustXAxisOptions::None),
        vec![],
    ));
    registry::add_alias("nl".to_string(), "new line");
    registry::add_alias("newline".to_string(), "new line");
    registry::add_alias("new_line".to_string(), "new line");
}