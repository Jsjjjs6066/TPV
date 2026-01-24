use crate::action::Action;
use clearscreen::clear;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, ExecutableCommand, QueueableCommand};
use serde_json::json;
use std::fs::File;
use std::io::{stdout, Write};
use BTMD::content::Content;
use BTMD::element::{Element, GROUP};
use BTMD::page::Page;

pub fn render_elements<'a>(
    page: &mut Page,
    elements: &'a mut [Element<'a>],
    parent_size: &(u16, u16),
    timer: &u32,
) -> Vec<Content<'a>> {
    let mut rendered_content: Vec<Content> = Vec::new();
    for element in elements {
        rendered_content.push(element.rerender(page, parent_size, timer));
    }
    rendered_content
}

pub fn render_page<'a, 'b>(
    page: &mut Page,
    timer: &u32,
    storage: &'b mut Option<Element<'a>>,
) -> Content<'b> {
    let b: Element = GROUP.new_from(vec![page.body_raw.clone(), json!({"min-height": "max"})]);
    *storage = Some(b);
    let rendered_c: Content<'b> = storage.as_mut().unwrap().render(
        page,
        &(
            crossterm::terminal::size().unwrap_or((0, 0)).0,
            crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
        ),
        timer,
    );
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

    stdout().flush().expect("Failed to flush stdout");

    stdout()
        .execute(cursor::SetCursorStyle::SteadyBlock)
        .expect("");
    rendered_c
}
fn rerender_page<'a, 'b>(
    page: &mut Page,
    timer: &u32,
    last_render_string: &str,
    storage: &'b mut Option<Element<'a>>,
) -> Content<'b> {
    let b: Element = GROUP.new_from(vec![page.body_raw.clone(), json!({"min-height": "max"})]);
    *storage = Some(b);
    let rendered_c: Content<'b> = storage.as_mut().unwrap().render(
        page,
        &(
            crossterm::terminal::size().unwrap_or((0, 0)).0,
            crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
        ),
        timer,
    );
    let new_render_string = rendered_c.render(&(
        crossterm::terminal::size().unwrap_or((0, 0)).0,
        crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
    ));
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

pub fn execute_page_tick<'a, 'b>(
    page: &mut Page,
    _last_size: (u16, u16),
    timer: &u32,
    last_render_string: &str,
    next_storage: &'b mut Option<Element<'a>>,
) -> Action<'b> {
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

                stdout()
                    .execute(cursor::MoveTo(
                        page.cursor.position.0,
                        page.cursor.position.1,
                    ))
                    .expect("");
            }
        }
    }

    if true {
        // *page
        //     .body
        //     .first_mut()
        //     .unwrap()
        //     .children
        //     .last_mut()
        //     .unwrap()
        //     .args
        //     .first_mut()
        //     .unwrap() = json!("Test2");
        let mut file = File::create("output.txt").unwrap();
        file.write_all(format!("{:?}", page.body.first().unwrap()).as_bytes()).unwrap();
    }

    let rerendered: Content<'b> = rerender_page(page, timer, last_render_string, next_storage);

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
    let mut use_a_next = true;

    // Initial render into storage_a
    let content = render_page(page, &0, &mut storage_a);
    let mut last_render_string = content.render(&(
        crossterm::terminal::size().unwrap_or((0, 0)).0,
        crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
    ));
    drop(content);

    let mut last_size = crossterm::terminal::size().unwrap_or((0, 0));
    let mut timer: u32 = 0;

    use_a_next = false;

    loop {
        let next_storage = if use_a_next {
            &mut storage_a
        } else {
            &mut storage_b
        };

        match execute_page_tick(page, last_size, &timer, &last_render_string, next_storage) {
            Action::Exit => {
                disable_raw_mode().unwrap();
                return;
            }
            Action::None(new_content) => {
                last_render_string = new_content.render(&(
                    crossterm::terminal::size().unwrap_or((0, 0)).0,
                    crossterm::terminal::size().unwrap_or((0, 0)).1 - 1,
                ));
                use_a_next = !use_a_next;
            }
        }
        last_size = crossterm::terminal::size().unwrap_or((0, 0));
        std::thread::sleep(std::time::Duration::from_millis(5));
        timer += 1;
    }
}
