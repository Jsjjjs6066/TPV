use crate::btmd::args::ArgParser;
use crate::btmd::values::ValueTypes;
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

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use border::BORDER;
pub use group::GROUP;
pub use heading::HEADING;
pub use label::LABEL;
pub use line::LINE;
pub use new_line::NEW_LINE;
pub use none::NONE;
pub use para::PARA;

static DEFAULT_ON_HOVER_REVERT_FUNC: fn(holder: Arc<RwLock<RawElement>>, page: &mut Page) =
    |holder: Arc<RwLock<RawElement>>, _| {
        holder.write().unwrap().args = holder.read().unwrap().raw_args.clone();
    };

/// The `RawElement` is an element owned by a shared pointer.
/// It only stores the arguments and the children.
/// Functionality is not defined here. It is only a container.
/// A developer has to do the functionality by themselves or 
/// use the default functionality in the `Element` struct.
/// 
/// The choice of functionality not being defined here is 
/// because most of inner and outside functions need a shared pointer
/// and to make it easier to implement new functionality.
/// 
/// The `RawElement` is the base struct for all elements.
/// It is not meant to be used directly.
#[derive(Clone, Debug)]
pub struct RawElement {
    pub args: Vec<Value>,
    pub raw_args: Vec<Value>,
    pub parent: Option<Arc<RwLock<RawElement>>>,
    pub children: Vec<Arc<RwLock<RawElement>>>,
    pub element_tag: &'static str,
    pub position: (u16, u16),
    pub size: Option<(u16, u16)>,
    pub hovered: bool,
    pub arg_parser: fn(
        parent_size: &(u16, u16),
    ) -> ArgParser,
    render_func: fn(
        holder: Arc<RwLock<RawElement>>,
        page: &mut Page,
        args: Vec<ValueTypes>,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content,
    prepare_children_func: fn(holder: Arc<RwLock<RawElement>>, args: &Vec<Value>, page: &Page) -> Vec<Arc<RwLock<RawElement>>>,
    on_hover_func: fn(holder: Arc<RwLock<RawElement>>, page: &mut Page),
    on_hover_revert_func: fn(holder: Arc<RwLock<RawElement>>, page: &mut Page),
}

pub trait ToElement {
    fn to_element(&self) -> Element;
}

impl ToElement for Arc<RwLock<RawElement>> {
    fn to_element(&self) -> Element {
        Element {
            raw_element: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Element {
    pub raw_element: Arc<RwLock<RawElement>>,
}

impl Element {
    pub fn read(&'_ self) -> RwLockReadGuard<'_, RawElement> {
        self.raw_element.read().unwrap()
    }

    pub fn write(&'_ self) -> RwLockWriteGuard<'_, RawElement> {
        self.raw_element.write().unwrap()
    }   

    pub fn new(
        render_func: fn(
            holder: Arc<RwLock<RawElement>>,
            page: &mut Page,
            args: Vec<ValueTypes>,
            parent_size: &(u16, u16),
            timer: &u32,
            pos: (u32, u32),
        ) -> Content,
        args: Vec<Value>,
        parent: Option<Arc<RwLock<RawElement>>>,
        prepare_children_function: fn(holder: Arc<RwLock<RawElement>>, args: &Vec<Value>, page: &Page) -> Vec<Arc<RwLock<RawElement>>>,
        element_tag: &'static str,
        position: (u16, u16),
        arg_parser: fn(
            parent_size: &(u16, u16),
        ) -> ArgParser,
    ) -> Self {
        Element {
            raw_element: Arc::new(RwLock::new(RawElement {
                args: args.clone(),
                raw_args: args.clone(),
                parent,
                children: Vec::new(),
                element_tag,
                position,
                size: None,
                hovered: false,
                arg_parser,
                render_func,
                prepare_children_func: prepare_children_function,
                on_hover_func: |_, _| {},
                on_hover_revert_func: DEFAULT_ON_HOVER_REVERT_FUNC,
            })),
        }
    }
    pub fn new_default(
        render_func: fn(
            holder: Arc<RwLock<RawElement>>,
            page: &mut Page,
            args: Vec<ValueTypes>,
            parent_size: &(u16, u16),
            timer: &u32,
            pos: (u32, u32),
        ) -> Content,
        element_tag: &'static str,
        arg_parser: fn(
            parent_size: &(u16, u16),
        ) -> ArgParser,
    ) -> Self {
        Element {
            raw_element: Arc::new(RwLock::new(RawElement {
                args: Vec::new(),
                raw_args: Vec::new(),
                parent: None,
                children: Vec::new(),
                element_tag,
                position: (0, 0),
                size: None,
                hovered: false,
                arg_parser,
                render_func,
                prepare_children_func: |_, _, _| -> Vec<Arc<RwLock<RawElement>>> { return Vec::new() },
                on_hover_func: |_, _| {},
                on_hover_revert_func: DEFAULT_ON_HOVER_REVERT_FUNC,
            })),
        }
    }
    pub fn new_from(&self, args: Vec<Value>, parent: Option<Arc<RwLock<RawElement>>>) -> Self {
        let mut new_element = self.clone();
        new_element.raw_element = Arc::new(RwLock::new(RawElement {
            args: args.clone(),
            raw_args: args.clone(),
            parent,
            children: Vec::new(),
            element_tag: self.raw_element.read().unwrap().element_tag,
            position: (0, 0),
            size: None,
            hovered: false,
            arg_parser: self.raw_element.read().unwrap().arg_parser,
            render_func: self.raw_element.read().unwrap().render_func,
            prepare_children_func: self.raw_element.read().unwrap().prepare_children_func,
            on_hover_func: self.raw_element.read().unwrap().on_hover_func,
            on_hover_revert_func: self.raw_element.read().unwrap().on_hover_revert_func,
        }));
        new_element
    }

    fn prepare_children(&self, page: &Page) {
        if self.raw_element.read().unwrap().children.is_empty() {
            self.raw_element.write().unwrap().children = (self.read().prepare_children_func)(self.raw_element.clone(), &self.raw_element.read().unwrap().args, page);
        }
    }

    pub fn render(
        &self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content {
        self.prepare_children(page);
        self.raw_element.write().unwrap().position = (pos.0 as u16, pos.1 as u16);
        let c: Content = (self.write().render_func)(
            self.raw_element.clone(),
            page,
            (self.raw_element.read().unwrap().arg_parser)(parent_size).parse(&self.raw_element.read().unwrap().args),
            parent_size,
            timer,
            pos,
        );
        self.write().size = Some(c.size);
        c
    }
    pub fn rerender(
        &self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content {
        self.write().position = (pos.0 as u16, pos.1 as u16);
        let c: Content = (self.write().render_func)(
            self.raw_element.clone(),
            page,
            (self.read().arg_parser)(parent_size).parse(&self.read().args),
            parent_size,
            timer,
            pos,
        );
        self.write().size = Some(c.size);
        c
    }

    pub fn on_hover(&self, page: &mut Page) {
        self.write().hovered = true;
        (self.read().on_hover_func)(self.raw_element.clone(), page)
    }

    pub fn set_on_hover_func(&self, on_hover_func: fn(holder: Arc<RwLock<RawElement>>, page: &mut Page)) {
        self.write().on_hover_func = on_hover_func;
    }

    pub fn get_size(&self) -> Option<(u16, u16)> {
        self.read().size
    }

    pub fn get_position(&self) -> (u16, u16) {
        self.read().position
    }

    pub fn is_hovered(&self) -> bool {
        self.read().hovered
    }

    pub fn on_hover_revert(&self, page: &mut Page) {
        (self.read().on_hover_revert_func)(self.raw_element.clone(), page)
    }

    pub fn set_on_hover_revert_func(
        &self,
        on_hover_revert_func: fn(holder: Arc<RwLock<RawElement>>, page: &mut Page),
    ) {
        self.write().on_hover_revert_func = on_hover_revert_func;
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("args", &self.read().args)
            .field("children", &self.read().children)
            .field("element_tag", &self.read().element_tag)
            .field("size", &self.read().size)
            .field("position", &self.read().position)
            .field("raw_args", &self.read().raw_args)
            .finish()
    }
}
