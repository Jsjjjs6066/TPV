pub mod element;
pub mod page;
pub mod cursor;

use crate::tprl::element::registry;
use crate::tprl::element::Element;
use serde_json::Value;

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
            args.get(0).unwrap_or(&Value::String("-".to_string()))
            .as_str().unwrap_or("-").chars().last().unwrap_or('-')
            .to_string().repeat(parent_size.0 as usize)
        },
        vec![],
    ));

    registry::register_element("border".to_string(), Element::new(
        |_, _, parent_size: &(u16, u16)| {
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
            for _ in 0..height - 3 {
                border.push(vertical_char);
                border.push_str(&" ".repeat(width - 2));
                border.push(vertical_char);
            }
            border.push(bottom_left);
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);
            border
        },
        vec![],
    ));
    registry::add_alias("b".to_string(), "border");
}