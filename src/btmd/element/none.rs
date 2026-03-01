use crate::btmd::{content::{Content, Text}, element::Element};
use std::{cell::RefCell, sync::LazyLock};

pub static NONE: LazyLock<Element> = LazyLock::new(||
	Element::new_default(
        |holder, _, _, _, _, _| Content::new(
            vec![Text::new_default(String::new())], 
            false,
            (0, 0),
            RefCell::new(holder.to_owned()),
        ), "none"
    )
);