use crate::command::Command;

use std::time::Duration;
use crossterm::event;

pub fn poll_inputs(timeout: Duration) -> Option<event::KeyEvent> {
    if event::poll(timeout).ok()? {
        let ev = event::read().ok()?;
        if let event::Event::Key(key_event) = ev {
            return Some(key_event);
        }
    }

    None
}

pub fn process_input(timout: Duration) -> Option<Command> {
    let key_event = poll_inputs(timout)?;

    match key_event.code {
        event::KeyCode::Char('q') | event::KeyCode::Char('Q') | event::KeyCode::Esc => Some(Command::Quit),
        _ => None
    }
}

