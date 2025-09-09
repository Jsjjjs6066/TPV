pub mod element;
pub mod page;
pub mod cursor;
pub mod content;
pub mod parse;

use element::registry;

pub fn import_default_elements() {
    registry::register_element("none".to_string(), &*element::NONE);
    
    registry::register_element("para".to_string(), &*element::PARA);
    registry::add_alias("p".to_string(), "para");
    registry::add_alias("paragraph".to_string(), "para");

    registry::register_element("label".to_string(), &*element::LABEL);
    registry::add_alias("l".to_string(), "label");

    registry::register_element("line".to_string(), &*element::LINE);

    registry::register_element("border".to_string(), &*element::BORDER);
    registry::add_alias("b".to_string(), "border");

    registry::register_element("heading".to_string(), &*element::HEADING);
    registry::add_alias("h".to_string(), "heading");

    registry::register_element("new line".to_string(), &*element::NEW_LINE);
    registry::add_alias("nl".to_string(), "new line");
    registry::add_alias("newline".to_string(), "new line");
    registry::add_alias("new_line".to_string(), "new line");
    
    registry::register_element("group".to_string(), &*element::GROUP);
    registry::add_alias("g".to_string(), "group");
}