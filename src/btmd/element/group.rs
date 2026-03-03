use btmd_macro::unwrap_val;
use crossterm::style::Color;
use serde_jsonc::Value;
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, RwLock};

use crate::btmd::content::ContentBuilder;
use crate::btmd::values::ValueTypes::{Array, Config};
use crate::btmd::values::{ArrayType, ColorType, ConfigType, OnHoverType, ValueTypes};
use crate::{args_parser, config_preset, element_array};
use crate::btmd::{content::Content, element::Element, page::Page, parse::parse_vec_to_vec};

pub static GROUP: LazyLock<Element> = LazyLock::new(|| {
    let mut group = Element::new(
        |holder: &mut Element,
         page: &mut Page,
         args: Vec<Value>,
         parent_size: &(u16, u16),
         timer: &u32,
         pos: (u32, u32)| {
            let config_preset = config_preset!(
                "background-color" => ValueTypes::Color(ColorType { value: Color::Reset })
            );
            let arg_parser = args_parser!(Array(ArrayType {
                array: vec![],
                vec_type: Box::new(ValueTypes::Element(Default::default())),
            }), Config(ConfigType(config_preset, Default::default())));
            let args_parsed = arg_parser.parse(args);
            let config: ConfigType = unwrap_val!(args_parsed.get(1).unwrap(), Config);
            let background_color = unwrap_val!(config.1.get("background-color").unwrap(), Color).value;

            let width: i32 = parent_size.0 as i32;

            let mut border_builder: ContentBuilder = ContentBuilder::new();

            let mut i: u32 = 0;
            let mut lines: u16 = 0;

            // let body_raw: Vec<Value> = args
            //     .get(0)
            //     .unwrap_or(&Value::Array(vec![]))
            //     .as_array()
            //     .unwrap()
            //     .to_vec();

            // let body: Vec<Element> = parse_vec_to_vec(body_raw, &page.registry);

            let mut rendered_content: Vec<Content> = Vec::new();

            for element_rc in holder.children.iter() {
                let mut element = element_rc.write().unwrap();
                rendered_content.push(element.render(
                    page,
                    &(parent_size),
                    timer,
                    (
                        i as u32 % parent_size.0 as u32 + pos.0,
                        lines as u32 + pos.1,
                    ),
                ));
                for t in &rendered_content.last().unwrap().text {
                    let mut temp: String = String::new();
                    for char in t.text.chars() {
                        if char == '\n' {
                            if i % parent_size.0 as u32 != 0 {
                                temp.push_str(&*" ".repeat(
                                    (width as u32 - (i) % width as u32).try_into().unwrap(),
                                ));
                                i += width as u32 - (i - 1) % width as u32;
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    if t.background_color == Color::Reset { background_color } else { t.background_color },
                                );
                                temp = String::new();
                                lines += 1;
                            }
                        } else if i % parent_size.0 as u32 == 0 {
                            lines += 1;
                            i += 1;
                            temp.push(char);
                        } else if char == '\t' {
                            let spaces: usize = 4 - (i as usize - 1) % 4;
                            temp.push_str(&*" ".repeat(spaces));
                            i += spaces as u32;
                        } else {
                            temp.push(char);
                            i += 1;
                        }
                    }
                    border_builder.append_text(temp, t.foreground_color, if t.background_color == Color::Reset { background_color } else { t.background_color });
                }
            }

            if !(i % width as u32 == 0) {
                border_builder.append_text(
                    (&*" ".repeat((width as u32 - i % width as u32) as usize)).to_string(),
                    Color::Reset,
                    background_color,
                );
            }

            border_builder.build(
                true,
                (parent_size.0, lines),
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
        "group",
        (0, 0),
    );

    group.set_on_hover_func(|holder: &mut Element, _| {
        let config_preset = config_preset!(
            "background-color" => ValueTypes::Color(ColorType { value: Color::Reset }),
            "onhover" => ValueTypes::OnHover(OnHoverType { map: Default::default() })
        );
        let arg_parser = args_parser!(element_array!(), Config(ConfigType(config_preset, Default::default())));
        let args_parsed = arg_parser.parse(holder.raw_args.to_owned());
        let config: ConfigType = unwrap_val!(args_parsed.get(1).unwrap(), Config);
        let background_color = unwrap_val!(config.1.get("background-color").unwrap(), Color).value;
        let onhover_config = unwrap_val!(config.1.get("onhover").unwrap(), OnHover);
        let onhover_config = onhover_config.parse_inner(config_preset!(
            "background-color" => ValueTypes::Color(ColorType { value: background_color })
        ));
        let onhover_background_color: Value = unwrap_val!(onhover_config.get("background-color").unwrap(), Color).into();
        holder.args[1]
            .as_object_mut()
            .unwrap()
            .insert("background-color".to_string(), onhover_background_color);
    });

    group
});
