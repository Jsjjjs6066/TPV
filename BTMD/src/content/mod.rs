pub mod adjust_x_axis_options;
pub mod text;
pub mod content_builder;

pub use adjust_x_axis_options::AdjustXAxisOptions;
pub use text::Text;
pub use content_builder::ContentBuilder;

pub struct Content {
    pub text: Vec<Text>,
    pub rerender_needed: bool,
    pub adjust_x_axis: AdjustXAxisOptions,
    pub size: (u16, u16),
}

impl Content {
    pub fn new(text: Vec<Text>, rerender_needed: bool, 
            adjust_x_axis: AdjustXAxisOptions, size: (u16, u16)) -> Content {
        Content {text, rerender_needed, adjust_x_axis, size}
    }

    pub fn render(&self) -> String {
        self.text.iter().map(|content_type| content_type.render()).collect::<Vec<String>>().join("")
    }
}