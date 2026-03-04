use std::{cell::RefCell, cmp::min};

use crate::{args_parser, btmd::{
    content::Content, element::Element, values::{ConfigType, ValueTypes}
}, config_preset};
use btmd_macro::unwrap_val;

use std::sync::LazyLock;

pub static PARA: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<ValueTypes>, parent_size: &(u16, u16), _, _| {
            let text = unwrap_val!(args.first().unwrap(), Text);
            Content::new(
                vec![text.0.clone()],
                false,
                (
                    min(
                        (text.0.text.chars().count() as u16).saturating_sub_signed(1),
                        parent_size.0,
                    ),
                    text.0.text.lines().count() as u16,
                ),
                RefCell::new(holder.to_owned()),
            )
        },
        "para",
        |_| {
            args_parser!(
                ValueTypes::Text(Default::default()),
                ValueTypes::Config(ConfigType(config_preset!(

                ), Default::default()))
            )
        },
    )
});
