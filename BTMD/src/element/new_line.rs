use serde_json::Value;
use std::sync::LazyLock;

use crate::{content::{Content, Text}, element::Element};

pub static NEW_LINE: LazyLock<Element> = LazyLock::new(||
    Element::new_default(
        |_, _, _, _, _| {
            Content::new(
                vec![Text::new_default("\n".to_string())],
                false,
                (0, 0)
            )
        }, "new_line".to_string()
    )
);