use crate::command::Command;

use std::time::Duration;
use crossterm::event;

fn poll_inputs(timeout: Duration) -> Option<event::KeyEvent> {
    if event::poll(timeout).ok()? {
        let ev = event::read().ok()?;
        if let event::Event::Key(key_event) = ev {
            return Some(key_event);
        }
    }

    None
}

fn poll_mouse(timeout: Duration) -> Option<event::MouseEvent> {
    if event::poll(timeout).ok()? {
        let ev = event::read().ok()?;
        if let event::Event::Mouse(mouse_event) = ev {
            return Some(mouse_event);
        }
    }

    None
}

pub fn process_input(timeout: Duration) -> Option<Command> {
    let key_event = poll_inputs(timeout)?;

    match key_event.code {
        event::KeyCode::Char('q') | event::KeyCode::Char('Q') | event::KeyCode::Esc => Some(Command::Quit),
        _ => None
    }
}

pub fn get_direction(timeout: Duration) -> Option<(u16, u16)> {
    let mouse_event = poll_mouse(timeout)?;
    match mouse_event.kind {
        event::MouseEventKind::Drag(event::MouseButton::Left) => Some((mouse_event.column, mouse_event.row)),
        _ => None    
    }
}
