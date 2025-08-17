use std::fs::{File, OpenOptions};
use std::io::{stdout, Write};
use crossterm::{cursor, event, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde_json::json;
use crate::action::Action;
use crate::tprl::cursor::Cursor;
use crate::tprl::element::{registry, Element};
use crate::tprl::element::registry::get_element;
use crate::tprl::page::Page;

pub fn render_elements(page: &mut Page, elements: Vec<Element>, parent_size: &(u16, u16)) -> String {
    let mut rendered_content = String::new();
    for element in elements {
        rendered_content.push_str(&element.render(page, parent_size));
    }
    rendered_content
}

pub fn render_page(page: &mut Page) {
    let body = page.body.clone();
    if !std::env::args().any(|arg| arg == "--no-border") {
        print!("{}", Element::new(get_element("b").render_func, vec![json!([]), json!({"min-height": "max"})]).render(page, &(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 1)));
    }
    let body_content: String = render_elements(page, body, &(crossterm::terminal::size().unwrap_or((0, 0)).0 - 2, crossterm::terminal::size().unwrap_or((0, 0)).1 - 2));
    stdout().execute(cursor::MoveTo(1, 1)).expect("");

    let mut line: u16 = 1;
    let mut i: usize = 0;
    for char in body_content.chars() {
        if char == '\n' {
            line += 1;
            stdout().execute(cursor::MoveTo(1, line)).expect("");
            i = 0;
            continue;
        }
        if i % (crossterm::terminal::size().unwrap_or((0, 0)).0 - 2) as usize == 0 && i != 0 {
            line += 1;
            stdout().execute(cursor::MoveTo(1, line)).expect("");
        }
        write!(stdout(), "{}", char).expect("Failed to write character");
        i += 1;
    }

    stdout().flush().expect("Failed to flush stdout");

    stdout().execute(cursor::SetCursorStyle::SteadyBlock).expect("");
    if std::env::args().any(|arg| arg == "--log") {
        let mut file: File = OpenOptions::new().append(true).open("log.txt").unwrap();
        file.write(body_content.as_ref()).expect("TODO: panic message");
    }
}

pub fn execute_page_tick(page: &mut Page, last_size: (u16, u16), cursor: &mut Cursor) -> Action {
    enable_raw_mode().unwrap();

    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read().unwrap() {
            // Check if the key event is a press (key-down event)
            if kind == event::KeyEventKind::Press {
                match code {
                    KeyCode::Char('q') => {
                        return Action::Exit
                    }
                    KeyCode::Down => {
                        cursor.move_down(1);
                    }
                    KeyCode::Up => {
                        cursor.move_up(1);
                    }
                    KeyCode::Left => {
                        cursor.move_left(1);
                    }
                    KeyCode::Right => {
                        cursor.move_right(1);
                    }
                    _ => {}
                }
                stdout().execute(cursor::MoveTo(cursor.position.0, cursor.position.1)).expect("");
            }
        }
    }

    if crossterm::terminal::size().unwrap_or((0, 0)) != last_size {
        disable_raw_mode().unwrap();
        clearscreen::clear().expect("");
        stdout().execute(cursor::MoveTo(0, 0)).expect("");
        render_page(page);
    }

    Action::None
}

pub fn run_page(page: &mut Page, cursor: &mut Cursor) {
    #[cfg(target_os = "windows")]
    {
        let title_command = format!("title {}", page.title.clone());
        let _ = std::process::Command::new("cmd")
            .args(&["/C", &title_command])
            .output();
    }
    #[cfg(not(target_os = "windows"))]
    {
        // ANSI escape code for setting the title on Unix-like systems
        let title_command = format!("\x1b]0;{}\x07", page.title);
        print!("{}", title_command);
        stdout().flush().unwrap(); // Ensure the command is sent to the terminal
    }
    render_page(page);
    let mut last_size = crossterm::terminal::size().unwrap_or((0, 0));
    loop {
        match execute_page_tick(page, last_size, cursor) {
            Action::Exit => {
                disable_raw_mode().unwrap();
                return;
            },
            Action::None => {}
        }
        last_size = crossterm::terminal::size().unwrap_or((0, 0));
    }
}