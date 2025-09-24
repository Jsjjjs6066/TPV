use std::io::{stdout, Write};
use crossterm::{cursor, event, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde_json::json;
use crate::action::Action;
use BTMD::content::Content;
use BTMD::cursor::Cursor;
use BTMD::element::{Element, GROUP};
use BTMD::page::Page;

pub fn render_elements(page: &mut Page, elements: Vec<Element>, parent_size: &(u16, u16), timer: &u32) -> Vec<Content> {
    let mut rendered_content: Vec<Content> = Vec::new();
    for mut element in elements {
        rendered_content.push(element.rerender(page, parent_size, timer));
    }
    rendered_content
}

pub fn render_page(page: &mut Page, timer: &u32) -> String {
    let mut b: Element = GROUP.new_from(vec![page.body_raw.clone(), json!({"min-height": "max"})]);
    let rendered: String = b.render(page, &(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 1), timer).render(&(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 1));
    print!("{}", rendered);
    page.cursor.position.0 = crossterm::terminal::size().unwrap_or((0, 0)).0 / 2;
    page.cursor.position.1 = crossterm::terminal::size().unwrap_or((0, 0)).1 / 2;
    // let body_content: Vec<Content> = render_elements(page, body, &(crossterm::terminal::size().unwrap_or((0, 0)).0 - 2, crossterm::terminal::size().unwrap_or((0, 0)).1 - 3));
    stdout().execute(cursor::MoveTo(crossterm::terminal::size().unwrap_or((0, 0)).0 / 2, crossterm::terminal::size().unwrap_or((0, 0)).1 / 2)).expect("");

    // let mut line: u16 = 1;
    // let mut i: usize = 0;
    // for c in body_content {
    //     for char in c.text.chars() {
    //         if char == '\u{1b}' {
    //             write!(stdout(), "{}", char).expect("Failed to write character");
    //         }
    //         if char == '\n' {
    //             line += 1;
    //             stdout().execute(cursor::MoveTo(1, line)).expect("");
    //             i = 0;
    //             continue;
    //         }
    //         if i % (crossterm::terminal::size().unwrap_or((0, 0)).0 - 2) as usize == 0 && i != 0 {
    //             line += 1;
    //             stdout().execute(cursor::MoveTo(1, line)).expect("");
    //         }
    //         write!(stdout(), "{}", char).expect("Failed to write character");
    //         i += 1;
    //     }
    // }

    stdout().flush().expect("Failed to flush stdout");

    stdout().execute(cursor::SetCursorStyle::SteadyBlock).expect("");
    rendered
}
fn rerender_page(page: &mut Page, timer: &u32, last_text: &String) -> String {
    let mut b: Element = GROUP.new_from(vec![page.body_raw.clone(), json!({"min-height": "max"})]);
    let rendered: String = b.render(page, &(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 1), timer).render(&(crossterm::terminal::size().unwrap_or((0, 0)).0, crossterm::terminal::size().unwrap_or((0, 0)).1 - 1));
    if rendered == *last_text {
        return rendered;
    }
    clearscreen::clear().expect("");
    print!("{}", rendered);
    stdout().execute(cursor::MoveTo(page.cursor.position.0, page.cursor.position.1)).expect("");
    rendered
}

pub fn execute_page_tick(page: &mut Page, last_size: (u16, u16), timer: &u32, last_text: &String) -> Action {
    enable_raw_mode().unwrap();

    if event::poll(std::time::Duration::from_millis(5)).unwrap() {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read().unwrap() {
            // Check if the key event is a press (key-down event)
            if kind == event::KeyEventKind::Press {
                match code {
                    KeyCode::Char('q') => {
                        return Action::Exit
                    }
                    KeyCode::Down => {
                        page.cursor.move_down(1);
                    }
                    KeyCode::Up => {
                        page.cursor.move_up(1);
                    }
                    KeyCode::Left => {
                        page.cursor.move_left(1);
                    }
                    KeyCode::Right => {
                        page.cursor.move_right(1);
                    }
                    _ => {}
                }
                stdout().execute(cursor::MoveTo(page.cursor.position.0, page.cursor.position.1)).expect("");
            }
        }
    }

    let rerendered: String = rerender_page(page, timer, last_text);

    Action::None(rerendered)
}

pub fn run_page(page: &mut Page) {
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
    let mut last_text: String = render_page(page, &0);
    let mut last_size = crossterm::terminal::size().unwrap_or((0, 0));
    let mut timer: u32 = 0;
    loop {
        match execute_page_tick(page, last_size, &timer, &last_text) {
            Action::Exit => {
                disable_raw_mode().unwrap();
                return;
            },
            Action::None(new_last_text) => {
                last_text = new_last_text;
            }
        }
        last_size = crossterm::terminal::size().unwrap_or((0, 0));
        std::thread::sleep(std::time::Duration::from_millis(5));
        timer += 1;
    }
}
