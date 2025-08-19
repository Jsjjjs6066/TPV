use crate::tprl::{content::Content, page::Page};
pub mod registry;
use serde_json::Value;

#[derive(Clone)]
pub struct Element {
    pub render_func: fn(page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)) -> Content,
    args: Vec<Value>
}

impl Element {
    pub fn new(render_func: fn(page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)) -> Content, args: Vec<Value>) -> Self {
        Element {render_func, args}
    }

    pub fn render(&self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        (self.render_func)(page, self.args.clone(), parent_size)
    }
}