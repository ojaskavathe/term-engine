use crate::command::Command;

use std::time::Duration;
use crossterm::event;

fn poll_inputs(timeout: Duration) -> Option<event::Event> {
    if event::poll(timeout).ok()? {
        return event::read().ok()
    }
    None
}

pub fn process_input(timeout: Duration) -> Option<Command> {
    let ev = poll_inputs(timeout)?;

    match ev {
        event::Event::Key(key_event) => {
            match key_event.code {
                event::KeyCode::Char('q') | event::KeyCode::Char('Q') | event::KeyCode::Esc => Some(Command::Quit),
                _ => None
            }   
        },
        event::Event::Mouse(mouse_event) => {
            match mouse_event.kind {
                event::MouseEventKind::Drag(event::MouseButton::Left) => Some(Command::Look(mouse_event.column, mouse_event.row)),
                _ => None    
            }
        },
        _ => None
    }
}
