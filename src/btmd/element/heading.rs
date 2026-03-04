use std::
    cell::RefCell
;

use btmd_macro::unwrap_val;
use crossterm::style::Color;
use figlet_rs::FIGfont;
use {std::sync::LazyLock, usize};

use crate::{
    args_parser, btmd::{
        content::{Content, Text}, element::Element, values::{ConfigType, IntType, TextType, ValueTypes, int::Int}
    }, config_preset
};

pub static HEADING: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder: &mut Element, _, args: Vec<ValueTypes>, parent_size: &(u16, u16), timer: &u32, _| {
            let font: FIGfont = FIGfont::standard().unwrap();
            let text: TextType = unwrap_val!(args.first().unwrap(), Text);
            let config: ConfigType = unwrap_val!(args.get(1).unwrap(), Config);
            let heading: String = font.convert(&text.0.text).unwrap().to_string();
            let speed: u8 = 11
                - u8::from(unwrap_val!(config.1.get("speed").unwrap(), Int).int);

            let mut width: u16 = heading
                .lines()
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or("")
                .chars()
                .count() as u16;
            let height: u16 = heading.lines().count() as u16;

            let returned_heading: String = if width > parent_size.0 {
                let new_heading = heading;
                new_heading
                    .lines()
                    .map(|s| {
                        let mut temp: String = String::new();
                        for i in 0..parent_size.0 {
                            temp.push(
                                s.chars()
                                    .nth(
                                        (i as u32
                                            + (timer / speed as u32)
                                                % (width - parent_size.0 / 2) as u32)
                                            as usize,
                                    )
                                    .unwrap_or(' '),
                            );
                        }
                        temp
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
                    + "\n"
            } else {
                heading
            };
            width = returned_heading
                .lines()
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap_or("")
                .chars()
                .count() as u16;
            Content::new(
                vec![Text::new(returned_heading, Color::Reset, Color::Reset)],
                false,
                (width, height),
                RefCell::new(holder.to_owned()),
            )
        },
        "heading",
        |_| args_parser!(
            ValueTypes::Text(Default::default()),
            ValueTypes::Config(ConfigType(config_preset!(
                "speed" => ValueTypes::Int(IntType {
                    int: Int::Bit8U(7),
                    min: Int::Bit8U(1),
                    max: Int::Bit8U(10),
                })
            ), Default::default()))
        ),
    )
});
