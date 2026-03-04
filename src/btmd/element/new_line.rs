use std::{cell::RefCell, sync::LazyLock};

use crate::{args_parser, btmd::{content::{Content, Text}, element::Element}};

pub static NEW_LINE: LazyLock<Element> = LazyLock::new(||
    Element::new_default(
        |holder, _, _, _, _, _| {
            Content::new(
                vec![Text::new_default("\n".to_string())],
                false,
                (0, 0),
                RefCell::new(holder.to_owned()),
            )
        }, "new_line",
        |_| args_parser!(),
    )
);