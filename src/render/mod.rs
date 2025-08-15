use std::io::{stdout, Write};
use crossterm::{cursor, event, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::action::Action;
use crate::tprl::cursor::Cursor;
use crate::tprl::element::Element;
use crate::tprl::page::Page;

pub fn render_elements(elements: Vec<Element>, parent_size: &(u16, u16)) -> String {
    let mut rendered_content = String::new();
    for element in elements {
        rendered_content.push_str(&element.render(&mut Page::new("".to_string(), vec![]), parent_size));
    }
    rendered_content
}

pub fn render_page(page: &mut Page) {
    let body = page.body.clone();

    let body_content: String = render_elements(body, &(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 2));

    println!("{}", body_content);

    stdout().execute(cursor::SetCursorStyle::SteadyBlock).expect("");
}

pub fn execute_page_tick(page: &mut Page, last_size: (u16, u16), cursor: &mut Cursor) -> Action {
    enable_raw_mode().unwrap();

    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
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