use crate::tprl::element::Element;

#[derive(Clone)]
pub struct Page {
	pub title: String,
	pub body: Vec<Element>,
	pub cursor: (u16, u16),
}

impl Page {
	pub fn new(title: String, body: Vec<Element>) -> Self {
		Page {title, body, cursor: (0, 0)}
	}
}