use serde_json::Value;

use crate::tprl::element::Element;

#[derive(Clone)]
pub struct Page {
	pub title: String,
	pub body: Vec<Element>,
	pub body_raw: Value,
	pub cursor: (u16, u16),
}

impl Page {
	pub fn new(title: String, body: Vec<Element>, body_raw: Value) -> Self {
		Page {title, body, body_raw, cursor: (0, 0)}
	}
}