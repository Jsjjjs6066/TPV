use crate::btmd;

use serde_jsonc::Value;

use btmd::{
    cursor::Cursor,
    element::{Element, registry::ElementRegistry},
    hovered_vec::{Finished, HoveredVec},
};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct Page {
    pub title: String,
    pub body: Vec<Arc<RwLock<Element>>>,
    pub body_raw: Value,
    pub cursor: Cursor,
    pub registry: ElementRegistry,
    timer: u32,
    hovered: HoveredVec,
}

impl Page {
    pub fn new(
        title: String,
        body: Vec<Arc<RwLock<Element>>>,
        body_raw: Value,
        registry: ElementRegistry,
    ) -> Self {
        Page {
            title,
            body,
            body_raw,
            cursor: Cursor::new(),
            registry,
            timer: 0,
            hovered: Default::default(),
        }
    }

    pub fn get_timer(&self) -> u32 {
        self.timer
    }

    pub fn tick(&mut self) {
        self.timer += 1;
    }

    fn inside(&self, start_x: u16, size_x: u16, start_y: u16, size_y: u16) -> bool {
        let x: u16 = self.cursor.position.0;
        let y: u16 = self.cursor.position.1;

        x >= start_x && x <= start_x + size_x - 1 && y >= start_y && y <= start_y + size_y - 1
    }

    pub fn handle_cursor_interaction(&mut self) {
        let body = self.body.clone();
        let mut queue = VecDeque::new();
        for element in body.iter() {
            queue.push_back(element.clone());
        }

        let mut hovered = HoveredVec::new();
        self.revert_hovered();

        while let Some(node_rc) = queue.pop_front() {
            let mut node = node_rc.write().unwrap();
            if let Some(size) = node.get_size() {
                let pos = node.get_position();
                if self.inside(pos.0, size.0, pos.1, size.1) {
                    node.on_hover(self);
                    hovered.add_element(node_rc.clone());
                    for neighbor in node.children.iter() {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        self.set_hovered(hovered.finish());
        self.body = body;
    }

    pub fn move_up(&mut self, steps: u16) {
        if self.cursor.position.1 < steps {
            self.cursor.position.1 = 0;
        } else {
            self.cursor.position.1 -= steps;
        }
        self.handle_cursor_interaction();
    }

    pub fn move_down(&mut self, steps: u16) {
        if self.cursor.position.1 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).1 {
            self.cursor.position.1 = crossterm::terminal::size().unwrap_or((0, 0)).1 - 1;
        } else {
            self.cursor.position.1 += steps;
        }
        self.handle_cursor_interaction();
    }

    pub fn move_left(&mut self, steps: u16) {
        if self.cursor.position.0 < steps {
            self.cursor.position.0 = 0;
        } else {
            self.cursor.position.0 -= steps;
        }
        self.handle_cursor_interaction();
    }

    pub fn move_right(&mut self, steps: u16) {
        if self.cursor.position.0 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).0 {
            self.cursor.position.0 = crossterm::terminal::size().unwrap_or((0, 0)).0 - 1;
        } else {
            self.cursor.position.0 += steps;
        }
        self.handle_cursor_interaction();
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.cursor.position = (x, y);
        self.handle_cursor_interaction();
    }

    pub fn set_hovered(&mut self, hovered_vec: HoveredVec<Finished>) {
        self.hovered = hovered_vec;
    }

    pub fn revert_hovered(&mut self) {
        for element in self.hovered.get_vec().to_owned() {
            element.write().unwrap().on_hover_revert(self);
        }
    }
}
