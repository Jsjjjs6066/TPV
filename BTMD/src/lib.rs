pub mod element;
pub mod page;
pub mod cursor;
pub mod content;
pub mod parse;

use element::registry;
use crate::element::{Border, Element, Group, Heading, Label, Line, NewLine, Paragraph, None};

pub fn import_default_elements() {
    registry::register_element("none".to_string(), Box::new(None));
    
    registry::register_element("para".to_string(), Box::new(Paragraph::new(vec![])));
    registry::add_alias("p".to_string(), "para");
    registry::add_alias("paragraph".to_string(), "para");

    registry::register_element("label".to_string(), Box::new(Label::new(vec![])));
    registry::add_alias("l".to_string(), "label");

    registry::register_element("line".to_string(), Box::new(Line::new(vec![])));

    registry::register_element("border".to_string(), Box::new(Border::new(vec![])));
    registry::add_alias("b".to_string(), "border");

    registry::register_element("heading".to_string(), Box::new(Heading::new(vec![])));
    registry::add_alias("h".to_string(), "heading");

    registry::register_element("new line".to_string(), Box::new(NewLine::new(vec![])));
    registry::add_alias("nl".to_string(), "new line");
    registry::add_alias("newline".to_string(), "new line");
    registry::add_alias("new_line".to_string(), "new line");
    
    registry::register_element("group".to_string(), Box::new(Group::new(vec![])));
    registry::add_alias("g".to_string(), "group");
}