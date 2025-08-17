pub mod element;
pub mod page;
pub mod cursor;

use std::fs::{File, OpenOptions};
use std::io::Write;
use figlet_rs::{FIGfont, FIGure};
use crate::tprl::element::registry;
use crate::tprl::element::Element;
use serde_json::{json, Map, Value};
use crate::parse::parse_vec_to_vec;
use crate::tprl::page::Page;

pub fn import_default_elements() {
    registry::register_element("none".to_string(), Element::new(|_, _, _| String::new(), vec![]));
    
    registry::register_element("para".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            args.get(0).unwrap_or(&Value::String("".to_string())).as_str().
            unwrap_or("").to_string() + if args.len() > 0 {"\n"} else {""}
        },
        vec![],
    ));
    registry::add_alias("p".to_string(), "para");
    registry::add_alias("paragraph".to_string(), "para");

    registry::register_element("label".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string()
        },
        vec![Value::String("text".to_string())],
    ));
    registry::add_alias("l".to_string(), "label");

    registry::register_element("line".to_string(), Element::new(
        |_, args: Vec<Value>, parent_size: &(u16, u16)| {
            args.get(0).unwrap_or(&Value::String("─".to_string()))
            .as_str().unwrap_or("─").chars().last().unwrap_or('─')
            .to_string().repeat(parent_size.0 as usize)
        },
        vec![],
    ));

    registry::register_element("border".to_string(), Element::new(
        |page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)| {
            let mut default_config: Map<String, Value> = Map::new();
            default_config.insert("min-height".to_string(), Value::Number(0.into()));
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

            let width: usize = parent_size.0 as usize;
            let height: usize = parent_size.1 as usize;
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

            let mut rendered_content = String::new();
            for element in body {
                rendered_content.push_str(&element.render(page, &(parent_size.0 - 2, parent_size.1 - 2)));
            }

            let mut lines: u16 = 1;

            for char in rendered_content.chars() {
                if (border.chars().count() + 1) % width == 0 {
                    border.push(vertical_char);
                    border.push(vertical_char);
                    lines += 1;
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

            if std::env::args().any(|arg| arg == "--log") {
                let mut file = OpenOptions::new().append(true).open("log.txt").unwrap();
                file.write(border.chars().count().to_string().as_ref()).expect("TODO: panic message");
                file.write(b"\n").expect("TODO: panic message");
                file.write(width.to_string().as_ref()).expect("TODO: panic message");
                file.write(b"\n").expect("TODO: panic message");
            }

            if (border.chars().count() - 1) % width == 0 {
                border.pop();
                lines -= 1;
            }

            if std::env::args().any(|arg| arg == "--log") {
                let mut file = OpenOptions::new().append(true).open("log.txt").unwrap();
                file.write(border.as_ref()).expect("TODO: panic message");
                file.write(b"\n").expect("TODO: panic message");
                file.write(width.to_string().as_ref()).expect("TODO: panic message");
                file.write(b"\n").expect("TODO: panic message");
            }

            if !(border.chars().count() % width == 0) {
                border.push_str(&*" ".repeat(width - 1 - border.chars().count() % width));
                border.push(vertical_char);
            }

            if lines < default_config.get("min-height").unwrap().as_u64().unwrap_or(0) as u16 {
                let additional_lines: u16 = default_config.get("min-height").unwrap().as_u64().unwrap_or(0) as u16 - lines;
                for _ in 0..additional_lines {
                    border.push(vertical_char);
                    border.push_str(&*" ".repeat(width - 2));
                    border.push(vertical_char);
                }
            }

            if std::env::args().any(|arg| arg == "--log") {
                let mut file = OpenOptions::new().append(true).open("log.txt").unwrap();
                file.write(border.as_ref()).expect("TODO: panic message");
                file.write(b"\n").expect("TODO: panic message");
            }

            border.push(bottom_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);

            /* border.push(top_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(top_right);
            for _ in 0..height - 3 {
                border.push(vertical_char);
                border.push_str(&" ".repeat(width - 2));
                border.push(vertical_char);
            }
            border.push(bottom_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);
            */
            border
        },
        vec![],
    ));
    registry::add_alias("b".to_string(), "border");

    registry::register_element("heading".to_string(), Element::new(
        |_, args: Vec<Value>, _| {
            let font: FIGfont = FIGfont::standard().unwrap();
            font.convert("Ž").unwrap().to_string()
        },
        vec![Value::String("text".to_string()), Value::Number(1.into())]
    ));
    registry::add_alias("h".to_string(), "heading");
}