use crate::render::run_page;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use std::io::{stdout, Write};
use BTMD::cursor::Cursor;
use BTMD::element::Element;
use BTMD::import_default_elements;

mod action;
mod render;

fn main() {
    // println!("{}", SetForegroundColor(Color::DarkGreen).to_string().chars().count());
    // println!("{}", SetForegroundColor(Color::Reset).to_string().chars().count());
    // if !std::env::args().any(|arg| arg == "--no-clear-on-run") {
    //     clearscreen::clear().expect("");
    // }
    let _ = stdout().execute(EnterAlternateScreen).unwrap();
    let filename: String = std::env::args()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .unwrap_or_else(|| {
            println!("Please specify a file to open.");
            std::process::exit(1);
        });
    let file_content: String = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut page: BTMD::page::Page = BTMD::parse::parse_str_to_page(&file_content);
    if std::env::args().any(|arg| arg == "--auto-exit") {
        let mut storage: Option<Element> = None;
        render::render_page(&mut page, &0, &mut storage);
    } else {
        run_page(&mut page);
    }
    stdout()
        .execute(crossterm::cursor::MoveTo(
            0,
            crossterm::terminal::size().unwrap_or((0, 0)).1,
        ))
        .unwrap();
    stdout()
        .execute(crossterm::cursor::SetCursorStyle::DefaultUserShape)
        .unwrap();
    let _ = stdout().execute(LeaveAlternateScreen).unwrap();
}
