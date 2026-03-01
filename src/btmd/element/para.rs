use serde_jsonc::Value;
use std::{cell::RefCell, cmp::min};

use crate::{args_parser, config_preset, btmd::{
    content::Content, element::Element, values::{ConfigType, TextType, ValueTypes}
}};
use btmd_macro::unwrap_val;

use std::sync::LazyLock;

pub static PARA: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<Value>, parent_size: &(u16, u16), _, _| {
            let config_preset = config_preset!();
            let arg_parser = args_parser!(ValueTypes::Text(Default::default()), ValueTypes::Config(ConfigType(config_preset, Default::default())));
            let args_parsed = arg_parser.parse(args);
            let text: TextType = unwrap_val!(args_parsed.first().unwrap(), Text);
            let _config: ConfigType = unwrap_val!(args_parsed.get(1).unwrap(), Config);
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
    )
});
