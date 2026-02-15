use crate::action::Action;
use btmd::content::Content;
use btmd::element::{Element, GROUP};
use btmd::logger;
use btmd::page::Page;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, ExecutableCommand, QueueableCommand};
use serde_json::json;
use std::io::{stdout, Write};

pub fn render_page(page: &mut Page, timer: &u32, storage: &mut Option<Element>) -> Content {
    if storage.is_none() {
        let b: Element = GROUP.new_from(vec![json!([]), json!({"min-height": "max"})]);
        *storage = Some(b);
    }

    let root = storage.as_mut().unwrap();
    root.children = std::mem::take(&mut page.body);

    let rendered_c: Content = root.render(
        page,
        &(
            crossterm::terminal::size().unwrap_or((0, 0)).0,
            crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
        ),
        timer,
        (0, 0),
    );

    page.body = std::mem::take(&mut root.children);

    page.cursor.position.0 = crossterm::terminal::size().unwrap_or((0, 0)).0 / 2;
    page.cursor.position.1 = crossterm::terminal::size().unwrap_or((0, 0)).1 / 2;
    stdout()
        .execute(cursor::MoveTo(
            crossterm::terminal::size().unwrap_or((0, 0)).0 / 2,
            crossterm::terminal::size().unwrap_or((0, 0)).1 / 2,
        ))
        .expect("");

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

    logger::write_page(&page.body).expect("Failed to write page");

    stdout().flush().expect("Failed to flush stdout");

    stdout()
        .execute(cursor::SetCursorStyle::SteadyBlock)
        .expect("");
    rendered_c
}
fn rerender_page(
    page: &mut Page,
    last_render_string: &str,
    storage: &mut Option<Element>,
) -> Content {
    if storage.is_none() {
        let b: Element = GROUP.new_from(vec![json!([]), json!({"min-height": "max"})]);
        *storage = Some(b);
    }

    let root = storage.as_mut().unwrap();
    root.children = std::mem::take(&mut page.body);

    let rendered_c: Content = root.render(
        page,
        &(
            crossterm::terminal::size().unwrap_or((0, 0)).0,
            crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
        ),
        &page.get_timer(),
        (0, 0),
    );

    page.body = std::mem::take(&mut root.children);

    let new_render_string = rendered_c.render();
    if new_render_string == last_render_string {
        return rendered_c;
    }

    let _ = stdout().queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ));
    let _ = stdout().queue(cursor::Hide);
    let _ = stdout().queue(cursor::MoveTo(0, 0));
    // clearscreen::clear().expect("");
    // print!("{}", rendered);
    let _ = stdout().queue(Print(&new_render_string));
    let _ = stdout()
        .queue(cursor::MoveTo(
            page.cursor.position.0,
            page.cursor.position.1,
        ))
        .expect("");
    let _ = stdout().queue(cursor::Show);
    let _ = stdout().flush();
    rendered_c
}

pub fn execute_page_tick<'a>(
    page: &mut Page,
    _last_size: (u16, u16),
    last_render_string: &'a str,
    next_storage: &mut Option<Element>,
) -> Action {
    enable_raw_mode().unwrap();

    if event::poll(std::time::Duration::from_millis(5)).unwrap() {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read().unwrap() {
            // Check if the key event is a press (key-down event)
            if kind == event::KeyEventKind::Press {
                match code {
                    KeyCode::Char('q') => {
                        return Action::Exit;
                    }
                    KeyCode::Down => {
                        page.move_down(1);
                    }
                    KeyCode::Up => {
                        page.move_up(1);
                    }
                    KeyCode::Left => {
                        page.move_left(1);
                    }
                    KeyCode::Right => {
                        page.move_right(1);
                    }
                    _ => {}
                }

                stdout()
                    .execute(cursor::MoveTo(
                        page.cursor.position.0,
                        page.cursor.position.1,
                    ))
                    .expect("");
            }
        }
    }

    let rerendered: Content = rerender_page(page, last_render_string, next_storage);

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
    let mut storage_a: Option<Element> = None;
    let mut storage_b: Option<Element> = None;
    let mut use_a_next = false;

    // Initial render into storage_a
    let content = render_page(page, &0, &mut storage_a);
    let mut last_render_string = content.render();
    stdout().execute(cursor::MoveTo(0, 0)).expect("");
    stdout().execute(Print(&last_render_string)).expect("");
    stdout()
        .execute(cursor::MoveTo(
            page.cursor.position.0,
            page.cursor.position.1,
        ))
        .expect("");
    stdout().flush().unwrap();
    drop(content);

    let mut last_size = crossterm::terminal::size().unwrap_or((0, 0));

    loop {
        let next_storage = if use_a_next {
            &mut storage_a
        } else {
            &mut storage_b
        };

        match execute_page_tick(page, last_size, &last_render_string, next_storage) {
            Action::Exit => {
                disable_raw_mode().unwrap();
                return;
            }
            Action::None(new_content) => {
                last_render_string = new_content.render();
                use_a_next = !use_a_next;
            }
        }
        last_size = crossterm::terminal::size().unwrap_or((0, 0));
        std::thread::sleep(std::time::Duration::from_millis(5));
        page.tick();
        stdout()
            .execute(cursor::MoveTo(
                page.cursor.position.0,
                page.cursor.position.1,
            ))
            .expect("");
        stdout().flush().unwrap();
    }
}
