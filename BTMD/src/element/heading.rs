use crossterm::style::Color;
use figlet_rs::FIGfont;
use serde_json::Value;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element};
use crate::page::Page;

pub struct Heading {
    args: Vec<Value>,
}

impl Element for Heading {
    fn new(args: Vec<Value>) -> Box<dyn Element> {
        Box::new(Heading{args})
    }

    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        let font: FIGfont = FIGfont::standard().unwrap();

        Content::new(
            vec![
                Text::new(
                    font.convert(
                        self.args.get(0)
                            .unwrap_or(&Value::String("".to_string())).
                            as_str().unwrap()).unwrap().to_string(),
                    Color::Reset,
                    Color::Reset
                )
            ],
            false,
            AdjustXAxisOptions::AutoScroll,
            (font.convert(
                self.args.get(0)
                    .unwrap_or(&Value::String("".to_string())).
                    as_str().unwrap()).unwrap().to_string().lines().nth(0).unwrap_or("").chars().count() as u16,
             font.convert(
                 self.args.get(0)
                     .unwrap_or(&Value::String("".to_string())).
                     as_str().unwrap()).unwrap().to_string().lines().count() as u16)
        )
    }
    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        self.render(page, parent_size)
    }
    
    fn new_from(&mut self, args: Vec<Value>) -> Box<dyn Element> where Self: Sized {
        Box::new(Heading{args})
    }
    
    fn clone_this(&self) -> Self where Self: Sized {
        Heading {args: self.args.clone()}
    }
}

// pub static HEADING: LazyLock<Element> = LazyLock::new(||
//     Element::new_default(
//         |_, _, args: Vec<Value>, parent_size: &(u16, u16)| {
//             let font: FIGfont = FIGfont::standard().unwrap();

//             Content::new(
//                 vec![
//                     Text::new(
//                         font.convert(
//                         args.get(0)
//                         .unwrap_or(&Value::String("".to_string())).
//                         as_str().unwrap()).unwrap().to_string(),
//                         Color::Reset,
//                         Color::Reset
//                     )
//                 ],
//                 false,
//                 AdjustXAxisOptions::AutoScroll,
//                 (font.convert(
//                     args.get(0)
//                         .unwrap_or(&Value::String("".to_string())).
//                         as_str().unwrap()).unwrap().to_string().lines().nth(0).unwrap_or("").chars().count() as u16,
//                  font.convert(
//                     args.get(0)
//                         .unwrap_or(&Value::String("".to_string())).
//                         as_str().unwrap()).unwrap().to_string().lines().count() as u16)
//             )
//         },
//     )
// );