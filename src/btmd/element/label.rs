use btmd_macro::unwrap_val;
use crossterm::style::Color;
use std::sync::LazyLock;
use std::{cell::RefCell, cmp::min};

use crate::{
    args_parser, btmd::{
        content::{Content, Text}, element::Element, values::{ConfigType, ValueTypes}
    }
};
use crate::config_preset;

pub static LABEL: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<ValueTypes>, parent_size: &(u16, u16), _, _| -> Content {
            let text = unwrap_val!(args.first().unwrap(), Text);
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
        |_| args_parser!(
            ValueTypes::Text(Default::default()),
            ValueTypes::Config(ConfigType(config_preset!(

            ), Default::default()))
        ),
    )
});
