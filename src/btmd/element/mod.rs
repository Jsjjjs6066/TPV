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

use std::sync::Arc;

pub use border::BORDER;
pub use group::GROUP;
pub use heading::HEADING;
pub use label::LABEL;
pub use line::LINE;
pub use new_line::NEW_LINE;
pub use none::NONE;
pub use para::PARA;

static DEFAULT_ON_HOVER_REVERT_FUNC: fn(holder: Arc<parking_lot::RwLock<RawElement>>, page: &mut Page) =
    |holder: Arc<parking_lot::RwLock<RawElement>>, _| {
        let raw_args = holder.read().raw_args.clone();
        holder.write().args = raw_args;
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
    pub parent: Option<Arc<parking_lot::RwLock<RawElement>>>,
    pub children: Vec<Arc<parking_lot::RwLock<RawElement>>>,
    pub element_tag: &'static str,
    pub position: (u16, u16),
    pub size: Option<(u16, u16)>,
    pub hovered: bool,
    pub arg_parser: fn(
        parent_size: &(u16, u16),
    ) -> ArgParser,
    render_func: fn(
        holder: Arc<parking_lot::RwLock<RawElement>>,
        page: &mut Page,
        args: Vec<ValueTypes>,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content,
    prepare_children_func: fn(holder: Arc<parking_lot::RwLock<RawElement>>, args: &Vec<Value>, page: &Page) -> Vec<Arc<parking_lot::RwLock<RawElement>>>,
    on_hover_func: fn(holder: Arc<parking_lot::RwLock<RawElement>>, page: &mut Page),
    on_hover_revert_func: fn(holder: Arc<parking_lot::RwLock<RawElement>>, page: &mut Page),
}

pub trait ToElement {
    fn to_element(&self) -> Element;
}

impl ToElement for Arc<parking_lot::RwLock<RawElement>> {
    fn to_element(&self) -> Element {
        Element {
            raw_element: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Element {
    pub raw_element: Arc<parking_lot::RwLock<RawElement>>,
}

impl Element {
    pub fn read(&'_ self) -> parking_lot::RwLockReadGuard<'_, RawElement> {
        self.raw_element.read()
    }

    pub fn write(&'_ self) -> parking_lot::RwLockWriteGuard<'_, RawElement> {
        self.raw_element.write()
    }

    pub fn new(
        render_func: fn(
            holder: Arc<parking_lot::RwLock<RawElement>>,
            page: &mut Page,
            args: Vec<ValueTypes>,
            parent_size: &(u16, u16),
            timer: &u32,
            pos: (u32, u32),
        ) -> Content,
        args: Vec<Value>,
        parent: Option<Arc<parking_lot::RwLock<RawElement>>>,
        prepare_children_function: fn(holder: Arc<parking_lot::RwLock<RawElement>>, args: &Vec<Value>, page: &Page) -> Vec<Arc<parking_lot::RwLock<RawElement>>>,
        element_tag: &'static str,
        position: (u16, u16),
        arg_parser: fn(
            parent_size: &(u16, u16),
        ) -> ArgParser,
    ) -> Self {
        Element {
            raw_element: Arc::new(parking_lot::RwLock::new(RawElement {
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
            holder: Arc<parking_lot::RwLock<RawElement>>,
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
            raw_element: Arc::new(parking_lot::RwLock::new(RawElement {
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
                prepare_children_func: |_, _, _| -> Vec<Arc<parking_lot::RwLock<RawElement>>> { return Vec::new() },
                on_hover_func: |_, _| {},
                on_hover_revert_func: DEFAULT_ON_HOVER_REVERT_FUNC,
            })),
        }
    }
    pub fn new_from(&self, args: Vec<Value>, parent: Option<Arc<parking_lot::RwLock<RawElement>>>) -> Self {
        let mut new_element = self.clone();
        new_element.raw_element = Arc::new(parking_lot::RwLock::new(RawElement {
            args: args.clone(),
            raw_args: args.clone(),
            parent,
            children: Vec::new(),
            element_tag: self.raw_element.read().element_tag,
            position: (0, 0),
            size: None,
            hovered: false,
            arg_parser: self.raw_element.read().arg_parser,
            render_func: self.raw_element.read().render_func,
            prepare_children_func: self.raw_element.read().prepare_children_func,
            on_hover_func: self.raw_element.read().on_hover_func,
            on_hover_revert_func: self.raw_element.read().on_hover_revert_func,
        }));
        new_element
    }

    fn prepare_children(&self, page: &Page) {
        if self.raw_element.read().children.is_empty() {
            let data = (self.read().prepare_children_func)(self.raw_element.clone(), &self.raw_element.read().args, page);
            self.raw_element.write().children = data;
        }
    }

    pub fn render(
        &self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
        pos: (u32, u32),
    ) -> Content {
        // prepare children may mutate `children` but does its own locking
        self.prepare_children(page);

        // update position without holding a long-lived lock
        self.raw_element.write().position = (pos.0 as u16, pos.1 as u16);

        // compute arguments while only holding read locks
        let args: Vec<ValueTypes> = {
            let parser = (self.read().arg_parser)(parent_size);
            // note: `self.raw_element.read()` is a separate lock from `self.read()`
            parser.parse(&self.raw_element.read().args)
        };

        // grab render function pointer under a write guard, then drop it
        let render_fn: fn(
            Arc<parking_lot::RwLock<RawElement>>,
            &mut Page,
            Vec<ValueTypes>,
            &(u16, u16),
            &u32,
            (u32, u32),
        ) -> Content = {
            let guard = self.write();
            guard.render_func
        };

        // call the render function without holding the guard
        let c: Content = render_fn(
            self.raw_element.clone(),
            page,
            args,
            parent_size,
            timer,
            pos,
        );

        // store computed size
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
        // quickly update position
        self.write().position = (pos.0 as u16, pos.1 as u16);

        // compute args under read lock only
        let args: Vec<ValueTypes> = {
            let parser = (self.read().arg_parser)(parent_size);
            parser.parse(&self.read().args)
        };

        // take render function pointer
        let render_fn: fn(
            Arc<parking_lot::RwLock<RawElement>>,
            &mut Page,
            Vec<ValueTypes>,
            &(u16, u16),
            &u32,
            (u32, u32),
        ) -> Content = {
            let guard = self.write();
            guard.render_func
        };

        let c: Content = render_fn(
            self.raw_element.clone(),
            page,
            args,
            parent_size,
            timer,
            pos,
        );
        self.write().size = Some(c.size);
        c
    }

    pub fn on_hover(&self, page: &mut Page) {
        self.write().hovered = true;
        let on_hover_func = self.read().on_hover_func;
        on_hover_func(self.raw_element.clone(), page)
    }

    pub fn set_on_hover_func(&self, on_hover_func: fn(holder: Arc<parking_lot::RwLock<RawElement>>, page: &mut Page)) {
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
        let on_hover_revert_func = {
            let guard = self.read();
            guard.on_hover_revert_func
        };
        on_hover_revert_func(self.raw_element.clone(), page)
    }

    pub fn set_on_hover_revert_func(
        &self,
        on_hover_revert_func: fn(holder: Arc<parking_lot::RwLock<RawElement>>, page: &mut Page),
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
