use crate::btmd::{content::Content, page::Page};

pub mod registry;
use serde_jsonc::Value;
use std::fmt::Debug;

pub mod border;
pub mod group;
pub mod heading;
pub mod label;
pub mod line;
pub mod new_line;
pub mod none;
pub mod para;

use std::sync::{Arc, RwLock};

pub use border::BORDER;
pub use group::GROUP;
pub use heading::HEADING;
pub use label::LABEL;
pub use line::LINE;
pub use new_line::NEW_LINE;
pub use none::NONE;
pub use para::PARA;

static DEFAULT_ON_HOVER_REVERT_FUNC: fn(holder: &mut Element, page: &mut Page) =
    |holder: &mut Element, _| {
        holder.args = holder.raw_args.clone();
    };

#[derive(Clone)]
pub struct Element {
    render_func: fn(
        holder: &mut Element,
        page: &mut Page,
        args: Vec<Value>,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content,
    pub args: Vec<Value>,
    pub children: Vec<Arc<RwLock<Element>>>,
    prepare_children_func: fn(&Vec<Value>, &Page) -> Vec<Arc<RwLock<Element>>>,
    pub element_tag: &'static str,
    on_hover_func: fn(holder: &mut Element, page: &mut Page),
    pub size: Option<(u16, u16)>,
    pub position: (u16, u16),
    raw_args: Vec<Value>,
    hovered: bool,
    on_hover_revert_func: fn(holder: &mut Element, page: &mut Page),
}

impl Element {
    pub fn new(
        render_func: fn(
            holder: &mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
            pos: (u32, u32),
        ) -> Content,
        args: Vec<Value>,
        prepare_children_function: fn(&Vec<Value>, &Page) -> Vec<Arc<RwLock<Element>>>,
        element_tag: &'static str,
        position: (u16, u16),
    ) -> Self {
        Element {
            render_func,
            args: args.clone(),
            children: Vec::new(),
            prepare_children_func: prepare_children_function,
            element_tag,
            on_hover_func: |_, _| {},
            size: None,
            position: position,
            raw_args: args,
            hovered: false,
            on_hover_revert_func: DEFAULT_ON_HOVER_REVERT_FUNC,
        }
    }
    pub fn new_default(
        render_func: fn(
            holder: &mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
            pos: (u32, u32),
        ) -> Content,
        element_tag: &'static str,
    ) -> Self {
        Element {
            render_func,
            args: Vec::new(),
            children: Vec::new(),
            prepare_children_func: |_, _| -> Vec<Arc<RwLock<Element>>> { return Vec::new() },
            element_tag,
            on_hover_func: |_, _| {},
            size: None,
            position: (0, 0),
            raw_args: Vec::new(),
            hovered: false,
            on_hover_revert_func: DEFAULT_ON_HOVER_REVERT_FUNC,
        }
    }
    pub fn new_from(&self, args: Vec<Value>) -> Self {
        let mut new_element = self.clone();
        new_element.args = args.clone();
        new_element.raw_args = args;
        new_element.children = Vec::new();
        new_element
    }

    fn prepare_children(&mut self, page: &Page) {
        if self.children.is_empty() {
            self.children = (self.prepare_children_func)(&self.args, page);
        }
    }

    pub fn render(
        &mut self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content {
        self.prepare_children(page);
        self.position = (pos.0 as u16, pos.1 as u16);
        let c: Content = (self.render_func)(self, page, self.args.clone(), parent_size, timer, pos);
        self.size = Some(c.size);
        c
    }
    pub fn rerender(
        &mut self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content {
        self.position = (pos.0 as u16, pos.1 as u16);
        let c: Content = (self.render_func)(self, page, self.args.clone(), parent_size, timer, pos);
        self.size = Some(c.size);
        c
    }

    pub fn on_hover(&mut self, page: &mut Page) {
        self.hovered = true;
        (self.on_hover_func)(self, page)
    }

    pub fn set_on_hover_func(&mut self, on_hover_func: fn(holder: &mut Element, page: &mut Page)) {
        self.on_hover_func = on_hover_func;
    }

    pub fn get_size(&self) -> Option<(u16, u16)> {
        self.size
    }

    pub fn get_position(&self) -> (u16, u16) {
        self.position
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn on_hover_revert(&mut self, page: &mut Page) {
        (self.on_hover_revert_func)(self, page)
    }

    pub fn set_on_hover_revert_func(
        &mut self,
        on_hover_revert_func: fn(holder: &mut Element, page: &mut Page),
    ) {
        self.on_hover_revert_func = on_hover_revert_func;
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("args", &self.args)
            .field("children", &self.children)
            .field("element_tag", &self.element_tag)
            .field("size", &self.size)
            .field("position", &self.position)
            .field("raw_args", &self.raw_args)
            .finish()
    }
}
