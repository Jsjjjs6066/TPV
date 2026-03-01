use btmd_macro::unwrap_val;
use crossterm::style::Color;
use serde_jsonc::Value;
use std::sync::LazyLock;
use std::{cell::RefCell, cmp::min};

use crate::{
    args_parser, btmd::{
        content::{Content, Text}, element::Element, values::{TextType, ValueTypes}
    }
};

pub static LABEL: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<Value>, parent_size: &(u16, u16), _, _| -> Content {
            let arg_parser = args_parser!(ValueTypes::Text(Default::default()));
            let args_parsed = arg_parser.parse(args);
            let text: TextType = unwrap_val!(args_parsed.first().unwrap(), Text);
            Content::new(
                vec![Text::new(text.0.text.clone(), Color::Reset, Color::Reset)],
                false,
                (
                    min(text.0.text.chars().count() as u16, parent_size.0),
                    text.0.text.lines().count() as u16,
                ),
                RefCell::new(holder.to_owned()),
            )
        },
        "label",
    )
});
