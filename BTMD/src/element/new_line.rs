use serde_json::Value;

use crate::{content::{AdjustXAxisOptions, Content, Text}, element::Element, page::Page};

pub struct NewLine {
    args: Vec<Value>,
}

impl Element for NewLine {
    fn new(args: Vec<Value>) -> Box<dyn Element> {
        Box::new(NewLine{args})
    }

    fn render(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        Content::new(
            vec![Text::new_default("\n".to_string())],
            false,
            AdjustXAxisOptions::None,
            (0, 0)
        )
    }
    fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16)) -> Content {
        self.render(page, parent_size)
    }
}

// pub static NEW_LINE: LazyLock<Element> = LazyLock::new(||
//     Element::new_default(
//         |_, _, _, _| {
//             Content::new(
//                 vec![Text::new_default("\n".to_string())],
//                 false,
//                 AdjustXAxisOptions::None,
//                 (0, 0)
//             )
//         },
//     )
// );