use std::sync::LazyLock;
use serde_json::{Map, Value};

use crate::{parse::parse_vec_to_vec, content::Content, element::Element, page::Page};
use crate::content::ContentBuilder;

pub static GROUP: LazyLock<Element> = LazyLock::new(|| {
    Element::new(
        |holder: &mut Element, page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32| {
            let mut default_config: Map<String, Value> = Map::new();
            let config: Map<String, Value> = args.get(1).unwrap_or(&Value::Object(Map::new())).as_object().unwrap_or(&default_config).iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            for (k, v) in config.iter() {
                default_config.insert(k.clone(), v.clone());
            }

            let width: i32 = parent_size.0 as i32;

            let mut border_builder: ContentBuilder = ContentBuilder::new();

            let mut i: i32 = 0;

            let body_raw: Vec<Value> = args.get(0).unwrap_or(&Value::Array(vec![])).as_array().unwrap().to_vec();

            let mut body: Vec<Element> = parse_vec_to_vec(body_raw);

            let mut rendered_content: Vec<Content> = Vec::new();

            for mut element in &holder.children {
                rendered_content.push(element.to_owned().render(page, &(parent_size), timer));
            }

            let mut lines: u16 = 1;

            for c in rendered_content.iter() {
                for t in &c.text {
                    let mut temp: String = String::new();
                    for char in t.text.chars() {
                        if char == '\n' {
                            if i % parent_size.0 as i32 != 0 {
                                temp.push_str(&*" ".repeat((width - (i) % width) as usize));
                                i += width - (i - 1) % width;
                                border_builder.append_text(temp, t.foreground_color, t.background_color);
                                temp = String::new();
                                lines += 1;
                            }
                        }
                        else if i % parent_size.0 as i32 == 0 {
                            lines += 1;
                            i += 1;
                            temp.push(char);
                        }
                        else if char == '\t' {
                            let spaces: i32 = 4 - (i - 1) % 4;
                            temp.push_str(&*" ".repeat(spaces as usize));
                            i += spaces;
                        }
                        else {
                            temp.push(char);
                            i += 1;
                        }
                    }
                    border_builder.append_text(temp, t.foreground_color, t.background_color);
                }
            }




            if !(i % width == 0) {
                border_builder.append_text_default((&*" ".repeat((width - i % width) as usize)).to_string());
            }


            border_builder.build(true, (parent_size.0, lines))
        },
        vec![],
        |args: &Vec<Value>| {
            parse_vec_to_vec((*args.get(0).unwrap_or(&Value::Array(vec![])).as_array().unwrap_or(&vec![])).clone())
        }, "group".to_string()
    )
});