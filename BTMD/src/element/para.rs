use std::cmp::min;
use serde_json::Value;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element};

use std::sync::LazyLock;

pub static PARA: LazyLock<Element> = LazyLock::new(|| 
    Element::new_default(
        |_, _, args: Vec<Value>, parent_size: &(u16, u16), _| {
            let text: String = args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string() + if args.len() > 0 {"\n"} else {""};
            Content::new(
                vec![Text::new_default(text.clone())],
                false,
                AdjustXAxisOptions::None,
                (min(text.chars().count() as u16 - 1, parent_size.0), text.lines().count() as u16)
            )
        }, "para".to_string()
    )
);