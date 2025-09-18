use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element};
use serde_json::Value;
use crate::page::Page;

pub struct None;

impl Element for None {
    fn new(args: Vec<Value>) -> Box<dyn Element> {
        Box::new(None)
    }

    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        Content::new(
            vec![Text::new_default(String::new())], 
            false, 
            AdjustXAxisOptions::None,
            (0, 0)
        )
    }
    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        Content::new(
            vec![Text::new_default(String::new())], 
            false, 
            AdjustXAxisOptions::None,
            (0, 0)
        )
    }
    
    fn new_from(&mut self, args: Vec<Value>) -> Box<dyn Element> where Self: Sized {
        Box::new(None)
    }
    
    fn clone_this(&self) -> Self where Self: Sized {
        None
    }
}