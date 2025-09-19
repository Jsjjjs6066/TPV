use std::cmp::min;
use serde_json::Value;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element, page::Page};

pub struct Paragraph {
    args: Vec<Value>,
}

impl Element for Paragraph {
    fn new(args: Vec<Value>) -> Box<dyn Element> {
        Box::new(Paragraph{args})
    }

    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        let text: String = self.args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string() + if self.args.len() > 0 {"\n"} else {""};
        Content::new(
            vec![Text::new_default(text.clone())],
            false,
            AdjustXAxisOptions::None,
            (min(text.chars().count() as u16 - 1, parent_size.0), text.lines().count() as u16)
        )
    }
    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        self.render(page, parent_size)
    }
    
    fn new_from(&mut self, args: Vec<Value>) -> Box<dyn Element> where Self: Sized {
        Box::new(Paragraph{args})
    }
    
    fn clone_this(&self) -> Box<dyn Element> {
        Box::new(Paragraph {args: self.args.clone()})
    }
}

// pub static PARA: LazyLock<Element> = LazyLock::new(||
//     Element::new_default(
//         |_, _, args: Vec<Value>, parent_size: &(u16, u16)| {
//             let text: String = args.get(0).unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").to_string() + if args.len() > 0 {"\n"} else {""};
//             Content::new(
//                 vec![Text::new_default(text.clone())],
//                 false,
//                 AdjustXAxisOptions::None,
//                 (min(text.chars().count() as u16 - 1, parent_size.0), text.lines().count() as u16)
//             )
//         },
//     )
// );