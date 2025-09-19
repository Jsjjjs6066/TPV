pub mod element;
pub mod page;
pub mod cursor;
pub mod content;
pub mod parse;

use std::collections::HashMap;

use element::registry;
use crate::element::{Border, Element, Group, Heading, Label, Line, NewLine, Paragraph, None};

pub fn default_elements() -> HashMap<String, Box<dyn Element>> {
    let mut map: HashMap<String, Box<dyn Element>> = HashMap::new();
    map.insert("none".to_string(), Box::new(None));
    
    map.insert("para".to_string(), Box::new(Paragraph::new(vec![])));
    map.insert("p".to_string(), Box::new(Paragraph::new(vec![])));
    map.insert("paragraph".to_string(), Box::new(Paragraph::new(vec![])));

    map.insert("label".to_string(), Box::new(Label::new(vec![])));
    map.insert("l".to_string(), Box::new(Label::new(vec![])));

    map.insert("line".to_string(), Box::new(Line::new(vec![])));

    map.insert("border".to_string(), Box::new(Border::new(vec![])));
    map.insert("b".to_string(), Box::new(Border::new(vec![])));

    map.insert("heading".to_string(), Box::new(Heading::new(vec![])));
    map.insert("h".to_string(), Box::new(Heading::new(vec![])));

    map.insert("new line".to_string(), Box::new(NewLine::new(vec![])));
    map.insert("nl".to_string(), Box::new(NewLine::new(vec![])));
    map.insert("newline".to_string(), Box::new(NewLine::new(vec![])));
    map.insert("new_line".to_string(), Box::new(NewLine::new(vec![])));
    
    map.insert("group".to_string(), Box::new(Group::new(vec![])));
    map.insert("g".to_string(), Box::new(Group::new(vec![])));
    map
}