pub mod text;
pub mod content_builder;

pub use text::Text;
pub use content_builder::ContentBuilder;

pub struct Content {
    pub text: Vec<Text>,
    pub rerender_needed: bool,
    pub size: (u16, u16),
}

impl Content {
    pub fn new(text: Vec<Text>, rerender_needed: bool, 
            size: (u16, u16)) -> Content {
        Content {text, rerender_needed, size}
    }

    pub fn render(&self, parent_size: &(u16, u16)) -> String {
        self.text.iter().map(|content_type: &Text| content_type.render()).collect::<Vec<String>>().join("")
    }

    pub fn update(&mut self, timer: &u32) {
        
    }
}