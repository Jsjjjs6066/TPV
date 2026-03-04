use btmd_macro::unwrap_val;
use serde_jsonc::Value;

use crate::{
    args_parser,
    btmd::{
        content::{Content, ContentBuilder},
        element::Element,
        page::Page,
        parse::parse_vec_to_vec,
        values::{ArrayType, BoolType, ColorType, ConfigType, OnHoverType, SizeType, ValueTypes, int::Int},
    },
    config_preset,
};

use crossterm::style::Color;
use std::{
    cell::RefCell,
    sync::{Arc, LazyLock, RwLock},
};

pub static BORDER: LazyLock<Element> = LazyLock::new(|| {
    let mut e = Element::new(
        |holder: &mut Element,
         page: &mut Page,
         args: Vec<ValueTypes>,
         parent_size: &(u16, u16),
         timer: &u32,
         pos: (u32, u32)| {
            let config: ConfigType = unwrap_val!(args.get(1).unwrap(), Config);
            let min_height: u16 = unwrap_val!(config.1.get("min-height").unwrap(), Size)
                .size
                .into();
            let connect_to_horizontal_chars: bool =
                unwrap_val!(config.1.get("connect-to-horizontal-chars").unwrap(), Bool).value;
            let color: Color = unwrap_val!(config.1.get("color").unwrap(), Color).value;
            let background_color: Color = unwrap_val!(config.1.get("background-color").unwrap(), Color).value;
            // let mut default_config: Map<String, Value> = Map::new();
            // default_config.insert("min-height".to_string(), Value::Number(0.into()));
            // default_config.insert("connect-to-horizontal-chars".to_string(), Value::Bool(true));
            // default_config.insert("color".to_string(), Value::String("default".to_string()));
            // let config: Map<String, Value> = args
            //     .get(1)
            //     .unwrap_or(&Value::Object(Map::new()))
            //     .as_object()
            //     .unwrap_or(&default_config)
            //     .iter()
            //     .map(|(k, v)| (k.clone(), v.clone()))
            //     .collect();
            // for (k, v) in config.iter() {
            //     default_config.insert(k.clone(), v.clone());
            // }

            // if default_config.get("min-height").is_none() {
            //     default_config.insert("min-height".to_string(), Value::Number(0.into()));
            // } else if default_config.get("min-height").unwrap().is_string() {
            //     if default_config.get("min-height").unwrap().as_str().unwrap() == "auto" {
            //         default_config.insert("min-height".to_string(), Value::Number(0.into()));
            //     } else if default_config.get("min-height").unwrap().as_str().unwrap() == "max" {
            //         default_config.insert(
            //             "min-height".to_string(),
            //             Value::Number((parent_size.1 - 2).into()),
            //         );
            //     }
            // }

            // let should_connect_to_horizontal_chars: bool = default_config
            //     .get("connect-to-horizontal-chars")
            //     .unwrap_or(&Value::Bool(true))
            //     .as_bool()
            //     .unwrap_or(true);

            let width: usize = parent_size.0 as usize;
            let horizontal_char: char = '─';
            let vertical_char: char = '│';
            let top_left: char = '┌';
            let top_right: char = '┐';
            let bottom_left: char = '└';
            let bottom_right: char = '┘';
            let mut border: String = String::new();

            // let color: &str = default_config
            //     .get("color")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or("default");
            // let color: Color = Color::try_from(color).unwrap_or(Color::Reset);

            let mut border_builder: ContentBuilder = ContentBuilder::new();
            border_builder.append_text(
                top_left.to_string()
                    + &horizontal_char.to_string().repeat(width - 2)
                    + top_right.to_string().as_str()
                    + vertical_char.to_string().as_str(),
                color,
                background_color,
            );

            let mut i = 1;
            let mut lines: u16 = 1;

            let mut rendered_content: Vec<Content> = Vec::new();
            for element_rc in holder.children.iter() {
                let mut element = element_rc.write().unwrap();
                if (i + 1) % width as u32 == 0 {
                    rendered_content.push(element.render(
                        page,
                        &(parent_size.0 - 2, parent_size.1 - 2),
                        timer,
                        (
                            (i + 2) % parent_size.0 as u32 + pos.0 as u32,
                            lines as u32 + 1 + pos.1,
                        ),
                    ));
                } else {
                    rendered_content.push(element.render(
                        page,
                        &(parent_size.0 - 2, parent_size.1 - 2),
                        timer,
                        (
                            i % parent_size.0 as u32 + pos.0 as u32,
                            lines as u32 + pos.1,
                        ),
                    ));
                }
                for t in &rendered_content.last().unwrap().text {
                    let mut temp: String = String::new();
                    for char in t.text.chars() {
                        if (i + 1) % width as u32 == 0 {
                            let last_char = temp.chars().last().unwrap_or_else(|| {
                                border_builder
                                    .content
                                    .last()
                                    .and_then(|t| t.text.chars().last())
                                    .unwrap_or(' ')
                            });
                            if last_char == horizontal_char && connect_to_horizontal_chars {
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    if t.background_color == Color::Reset { background_color } else { t.background_color },
                                );
                                border_builder.append_text(
                                    '┤'.to_string() + vertical_char.to_string().as_str(),
                                    color,
                                    background_color,
                                );
                                temp = String::new();
                            } else {
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    if t.background_color == Color::Reset { background_color } else { t.background_color },
                                );
                                border_builder.append_text(
                                    vertical_char.to_string() + vertical_char.to_string().as_str(),
                                    color,
                                    background_color,
                                );
                                temp = String::new();
                            }
                            i += 2;
                            lines += 1;
                        }
                        if (i - 1) % width as u32 == 0
                            && char == horizontal_char
                            && connect_to_horizontal_chars
                        {
                            border_builder.content.last_mut().unwrap().text.pop();
                            border_builder.content.last_mut().unwrap().text.push('├');
                        }
                        if char == '\n' {
                            if i % width as u32 != 1 {
                                temp.push_str(&*" ".repeat(width - 2 - (i as usize - 1) % width));
                                i += width as u32 - 2 - (i - 1) % width as u32;
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    if t.background_color == Color::Reset { background_color } else { t.background_color },
                                );
                                border_builder.append_text(
                                    vertical_char.to_string() + vertical_char.to_string().as_str(),
                                    color,
                                    background_color,
                                );
                                temp = String::new();
                                i += 2;
                                lines += 1;
                            }
                        } else if char == '\t' {
                            let spaces: usize = 4 - (i as usize - 1) % 4;
                            temp.push_str(&*" ".repeat(spaces));
                            i += spaces as u32;
                        } else {
                            temp.push(char);
                            i += 1;
                        }
                    }
                    if temp != "" {
                        border_builder.append_text(temp, t.foreground_color, if t.background_color == Color::Reset { background_color } else { t.background_color });
                    }
                }
            }

            if (i - 1) % width as u32 == 0 {
                while let Some(last) = border_builder
                    .content
                    .last_mut()
                    .and_then(|c| c.text.chars().last())
                {
                    if last == ' ' {
                        border_builder.content.last_mut().unwrap().text.pop();
                        i -= 1;
                    } else {
                        break;
                    }
                }
                lines -= 1;
            }

            if (i - 1) % width as u32 == 0 {
                if border_builder.content.len() > 1 {
                    let ind: usize = border_builder.content.len() - 1;
                    border_builder.content[ind].text.pop();
                } else {
                    border_builder.content.last_mut().unwrap().text.pop();
                }
                i -= 1;
            }

            if !(i % width as u32 == 0) {
                let padding_needed = width - 1 - i as usize % width;
                border_builder.append_text((&*" ".repeat(padding_needed)).to_string(), color, background_color);
                if padding_needed == 0 {
                    let last_char = border_builder
                        .content
                        .get(border_builder.content.len().saturating_sub(2))
                        .and_then(|t| t.text.chars().last())
                        .unwrap_or(' ');
                    if last_char == horizontal_char && connect_to_horizontal_chars {
                        border_builder.append_text('┤'.to_string(), color, background_color);
                    } else {
                        border_builder.append_text(vertical_char.to_string(), color, background_color);
                    }
                } else {
                    border_builder.append_text(vertical_char.to_string(), color, background_color);
                }
            }

            if lines < min_height {
                let additional_lines: u16 = min_height.saturating_sub(lines);
                for _ in 0..additional_lines {
                    border_builder.append_text(
                        vertical_char.to_string()
                            + &*" ".repeat(width - 2)
                            + vertical_char.to_string().as_str(),
                        color,
                        background_color,
                    );
                    lines += 1;
                }
            }

            border_builder.append_text(
                bottom_left.to_string()
                    + &horizontal_char.to_string().repeat(width - 2)
                    + bottom_right.to_string().as_str(),
                color,
                background_color,
            );
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);
            border_builder.build(
                true,
                (parent_size.0, lines + 2),
                RefCell::new(holder.to_owned()),
            )
        },
        vec![],
        |args: &Vec<Value>, page: &Page| -> Vec<Arc<RwLock<Element>>> {
            let res = parse_vec_to_vec(
                (*args
                    .get(0)
                    .unwrap_or(&Value::Array(vec![]))
                    .as_array()
                    .unwrap_or(&vec![]))
                .clone(),
                &page.registry,
            );
            res
        },
        "border",
        (0, 0),
        |parent_size: &(u16, u16)| args_parser!(
            ValueTypes::Array(ArrayType {
                array: Default::default(),
                vec_type: Box::new(ValueTypes::Element(Default::default()))
            }),
            ValueTypes::Config(ConfigType(config_preset!(
                "min-height" => ValueTypes::Size(SizeType {
                    size: Int::Bit16U(0),
                    min: Int::Bit16U(0),
                    max: Int::Bit16U(parent_size.1 - 2),
                    auto: Int::Bit16U(0),
                }),
                "connect-to-horizontal-chars" => ValueTypes::Bool(BoolType { value: true }),
                "color" => ValueTypes::Color(Default::default()),
                "background-color" => ValueTypes::Color(Default::default())
            ), Default::default()))
        ),
    );
    e.set_on_hover_func(|holder: &mut Element, _| {
        let config_preset = config_preset!(
            "color" => ValueTypes::Color(Default::default()),
            "background_color" => ValueTypes::Color(Default::default()),
            "onhover" => ValueTypes::OnHover(OnHoverType {
                map: Default::default()
            })
        );
        let arg_parser = args_parser!(
            ValueTypes::Array(ArrayType {
                array: Default::default(),
                vec_type: Box::new(ValueTypes::Element(Default::default()))
            }),
            ValueTypes::Config(ConfigType(config_preset, Default::default()))
        );
        let args_parsed = arg_parser.parse(&holder.raw_args);
        let config: ConfigType = unwrap_val!(args_parsed.get(1).unwrap(), Config);
        let color: ColorType = unwrap_val!(config.1.get("color").unwrap(), Color);
        let background_color: ColorType = unwrap_val!(config.1.get("background_color").unwrap(), Color);
        let onhover_config: OnHoverType = unwrap_val!(config.1.get("onhover").unwrap(), OnHover);
        let onhover_config = onhover_config.parse_inner(config_preset!(
            "color" => ValueTypes::Color(color),
            "background_color" => ValueTypes::Color(background_color)
        ));
        let color: Value = unwrap_val!(onhover_config.get("color").unwrap(), Color).into();
        let background_color: Value = unwrap_val!(onhover_config.get("background_color").unwrap(), Color).into();
        if holder.args.len() <= 1 {
            holder.args.resize(2, Value::Object(Default::default()));
        }
        holder.args[1]
            .as_object_mut()
            .unwrap()
            .insert("color".to_string(), color);
        holder.args[1]
            .as_object_mut()
            .unwrap()
            .insert("background_color".to_string(), background_color);
    });
    e
});
