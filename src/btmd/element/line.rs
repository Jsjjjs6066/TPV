use btmd_macro::unwrap_val;

use crate::{
    args_parser, btmd::{
        content::{Content, Text}, element::Element, values::{CharType, ConfigType, ValueTypes}
    }, config_preset
};

use std::sync::LazyLock;

pub static LINE: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |_, _, args: Vec<ValueTypes>, parent_size: &(u16, u16), _, _| {
            let char: char = unwrap_val!(args.first().unwrap(), Char).0;
            if char == '\n' {
                return Content::new(
                    vec![Text::new_default("\n".to_string())],
                    false,
                    (parent_size.0, 1),
                );
            }
            if char == ' ' {
                return Content::new(
                    vec![Text::new_default(String::new())],
                    false,
                    (parent_size.0, 1),
                );
            }
            if char == '-' {
                return Content::new(
                    vec![Text::new_default(
                        "─".to_string().repeat(parent_size.0 as usize),
                    )],
                    true,
                    (parent_size.0, 1),
                );
            }
            Content::new(
                vec![Text::new_default(char.to_string().repeat(parent_size.0 as usize))],
                true,
                (parent_size.0, 1),
            )
        },
        "line",
        |_| args_parser!(
            ValueTypes::Char(CharType('─')),
            ValueTypes::Config(ConfigType(config_preset!(

            ), Default::default()))
        ),
    )
});
