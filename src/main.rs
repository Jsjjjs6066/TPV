use std::io::{stdout};
use crossterm::ExecutableCommand;
use crate::render::run_page;
use BTMD::cursor::Cursor;
use BTMD::import_default_elements;

mod render;
mod action;

fn main() {
	// println!("{}", SetForegroundColor(Color::DarkGreen).to_string().chars().count());
	// println!("{}", SetForegroundColor(Color::Reset).to_string().chars().count());
	import_default_elements();
	if !std::env::args().any(|arg| arg == "--no-clear-on-run") {
		clearscreen::clear().expect("");
	}
	let filename: String = std::env::args().skip(1).find(| arg | !arg.starts_with("--")).unwrap_or_else(|| {
		println!("Please specify a file to open.");
		std::process::exit(1);
	});
	let file_content: String = std::fs::read_to_string(filename).expect("Failed to read file");
	let mut page: BTMD::page::Page = BTMD::parse::parse_str_to_page(&file_content);
	let mut cursor: Cursor = Cursor::new();
	if std::env::args().any(|arg| arg == "--auto-exit") {
		render::render_page(&mut page);
	}
	else {
		run_page(&mut page, &mut cursor);
	}
	stdout().execute(crossterm::cursor::MoveTo(0, crossterm::terminal::size().unwrap_or((0, 0)).1)).unwrap();
	stdout().execute(crossterm::cursor::SetCursorStyle::DefaultUserShape).unwrap();
}