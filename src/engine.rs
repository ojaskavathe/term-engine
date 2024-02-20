use crate::input;
use crate::command::Command;

use std::time::{Duration, Instant};

use crossterm::{terminal, cursor, style, ExecutableCommand};

use std::io::Stdout;

pub struct Engine {
    stdout: Stdout,
    og_term_size: (u16, u16),
    width: u16,
    height: u16,
}

impl Engine {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let og_term_size: (u16, u16) = terminal::size().unwrap();
        Self {
            stdout,
            og_term_size,
            width,
            height
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        let frame_time = Duration::from_millis(30);


        self.prepare_ui();

        while !done {
            let now = Instant::now();
            self.render();

            while now.elapsed() < frame_time {
                // input polling and processing
                if let Some(command) = input::process_input(frame_time - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                    }
                }
            }
        }

        self.restore_ui();
    }

    fn render(&mut self) {
        for y in 0..self.height + 2 {
            self.stdout
                .execute(cursor::MoveTo(0, y)).unwrap()
                .execute(style::Print("║")).unwrap()
                .execute(cursor::MoveTo(self.width + 1, y)).unwrap()
                .execute(style::Print("║")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(cursor::MoveTo(x, 0)).unwrap()
                .execute(style::Print("═")).unwrap()
                .execute(cursor::MoveTo(x, self.height + 1)).unwrap()
                .execute(style::Print("═")).unwrap();
        }

        self.stdout
            .execute(cursor::MoveTo(0, 0)).unwrap()
            .execute(style::Print("╔")).unwrap()
            .execute(cursor::MoveTo(self.width + 1, 0)).unwrap()
            .execute(style::Print("╗")).unwrap()
            .execute(cursor::MoveTo(self.width + 1, self.height + 1)).unwrap()
            .execute(style::Print("╝")).unwrap()
            .execute(cursor::MoveTo(0, self.height + 1)).unwrap()
            .execute(style::Print("╚")).unwrap();
    }

    fn prepare_ui(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            .execute(terminal::SetSize(self.width, self.height)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(cursor::Hide).unwrap();
    }

    fn restore_ui(&mut self) {
        self.stdout
            .execute(terminal::SetSize(self.og_term_size.0, self.og_term_size.1)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(cursor::Show).unwrap()
            .execute(style::ResetColor).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
