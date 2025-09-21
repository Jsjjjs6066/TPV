use crate::{content::{Content, Text}, element::Element};
use std::sync::LazyLock;

pub static NONE: LazyLock<Element> = LazyLock::new(||
	Element::new_default(
        |_, _, _, _, _| Content::new(
            vec![Text::new_default(String::new())], 
            false,
            (0, 0)
        ), "none".to_string()
    )
);