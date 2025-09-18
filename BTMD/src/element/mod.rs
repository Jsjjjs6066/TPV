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
pub mod has_children;

pub use none::None;
pub use para::Paragraph;
pub use label::Label;
pub use line::Line;
pub use border::Border;
pub use heading::Heading;
pub use new_line::NewLine;
pub use group::Group;

//#[derive(Clone)]
pub trait Element: Send + Sync {
    fn new(args: Vec<Value>) -> Box<dyn Element> where Self: Sized;
    fn new_from(&mut self, args: Vec<Value>) -> Box<dyn Element> where Self: Sized;
    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content;
    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content;
    // fn clone_box(&self) -> Box<dyn Element + Send + Sync + 'static> {
    //     panic!("clone_box not implemented for this Element");
    // }
    fn clone_this(&self) -> Self where Self: Sized;
}

impl<T> Element for T
where
    T: Send + Sync + Clone + 'static,
{
    fn new(args: Vec<Value>) -> Box<dyn Element> {
        todo!()
    }

    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        todo!()
    }

    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        todo!()
    }

    // fn clone_box(&self) -> Box<dyn Element + Send + Sync + 'static> {
    //     Box::new(self.clone())
    // }
    
    fn new_from(&mut self, args: Vec<Value>) -> Box<dyn Element> where Self: Sized {
        todo!()
    }
    fn clone_this(&self) -> Self where Self: Sized {
        self.clone()
    }
}

// impl Clone for Box<dyn Element + Send + Sync> {
//     fn clone(&self) -> Box<dyn Element + Send + Sync> {
        
//     }
// }

impl Clone for Box<dyn Element> {
    fn clone(&self) -> Box<dyn Element> {
        self.to_owned().clone_this()
    }
}

//impl Element {
//    pub fn new(render_func: fn(holder: &mut Element, page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)) -> Content, args: Vec<Value>, prepare_children_function: fn(&Vec<Value>) -> Vec<Element>) -> Self {
//        Element {render_func, args, children: Vec::new(), prepare_children_func: prepare_children_function}
//    }
//    pub fn new_default(render_func: fn(holder: &mut Element, page: &mut Page, args: Vec<Value>, parent_size: &(u16, u16)) -> Content) -> Self {
//        Element {render_func, args: Vec::new(), children: Vec::new(), prepare_children_func: |args: &Vec<Value>| -> Vec<Element> {return Vec::new()}}
//    }
//    pub fn new_from(&self, args: Vec<Value>) -> Self {
//        Element {render_func: self.render_func, args, children: Vec::new(), prepare_children_func: self.prepare_children_func}
//    }

//    fn prepare_children(&mut self) {
//        self.children = (self.prepare_children_func)(&self.args);
//    }

//    pub fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
//        self.prepare_children();
//        (self.render_func)(self, page, self.args.clone(), parent_size)
//    }
//    pub fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
//        (self.render_func)(self, page, self.args.clone(), parent_size)
//    }
//}