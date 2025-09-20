use crate::{content::Content, page::Page};
pub mod registry;
use serde_json::Value;

pub mod none;
pub mod label;
pub mod para;
pub mod line;
pub mod border;
pub mod heading;
pub mod new_line;
pub mod group;

pub use none::NONE;
pub use para::PARA;
pub use label::LABEL;
pub use line::LINE;
pub use border::BORDER;
pub use heading::HEADING;
pub use new_line::NEW_LINE;
pub use group::GROUP;

#[derive(Clone)]
pub struct Element {
    render_func: fn(holder: &mut Element,  page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32) -> Content,
    args: Vec<Value>,
    pub children: Vec<Element>,
    prepare_children_func: fn(&Vec<Value>) -> Vec<Element>,
    element_tag: String,
}

impl Element {
    pub fn new(render_func: fn(holder: &mut Element, page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32) -> Content, args: Vec<Value>, prepare_children_function: fn(&Vec<Value>) -> Vec<Element>, element_tag: String) -> Self {
        Element {render_func, args, children: Vec::new(), prepare_children_func: prepare_children_function, element_tag}
    }
    pub fn new_default(render_func: fn(holder: &mut Element, page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16), timer: &u32) -> Content, element_tag: String) -> Self {
        Element {render_func, args: Vec::new(), children: Vec::new(), prepare_children_func: |args: &Vec<Value>| -> Vec<Element> {return Vec::new()}, element_tag}
    }
    pub fn new_from(&self, args: Vec<Value>) -> Self {
        Element {render_func: self.render_func, args, children: Vec::new(), prepare_children_func: self.prepare_children_func, element_tag: self.element_tag.clone()}
    }

    fn prepare_children(&mut self) {
        self.children = (self.prepare_children_func)(&self.args);
    }

    pub fn render(&mut self, page: &mut Page, parent_size: &(u16, u16), timer: &u32) -> Content {
        self.prepare_children();
        (self.render_func)(self, page, self.args.clone(), parent_size, timer)
    }
    pub fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16), timer: &u32) -> Content {
        (self.render_func)(self, page, self.args.clone(), parent_size, timer)
    }
}