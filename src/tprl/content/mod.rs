pub mod adjust_x_axis_options;

pub use adjust_x_axis_options::AdjustXAxisOptions;

pub struct Content {
    pub text: String,
    pub rerender_needed: bool,
    pub adjust_x_axis: AdjustXAxisOptions,
}

impl Content {
    pub fn new(text: String, rerender_needed: bool, 
            adjust_x_axis: AdjustXAxisOptions) -> Content {
        Content {text, rerender_needed, adjust_x_axis}
    }
}