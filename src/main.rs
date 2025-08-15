use std::io::{stdout};
use crossterm::ExecutableCommand;
use crate::render::run_page;
use crate::tprl::cursor::Cursor;
use crate::tprl::import_default_elements;

mod tprl;
mod render;
mod parse;
mod action;

fn main() {
	import_default_elements();
	if !std::env::args().any(|arg| arg == "--no-clear-on-run") {
		clearscreen::clear().expect("");
	}
	let filename: String = std::env::args().skip(1).find(| arg | !arg.starts_with("--")).unwrap_or_else(|| {
		println!("Please specify a file to open.");
		std::process::exit(1);
	});
	let file_content: String = std::fs::read_to_string(filename).expect("Failed to read file");
	let mut page: tprl::page::Page = parse::parse(&file_content);
	let mut cursor: Cursor = Cursor::new();
	run_page(&mut page, &mut cursor);
	stdout().execute(crossterm::cursor::MoveTo(0, crossterm::terminal::size().unwrap_or((0, 0)).1)).unwrap();
	stdout().execute(crossterm::cursor::SetCursorStyle::DefaultUserShape).unwrap();
}