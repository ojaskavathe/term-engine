use crate::input;
use crate::command::Command;
use crate::vec::Vec2;
use crate::surface::{Surface, Element};

use std::time::{Duration, Instant};
use std::io::{self, BufWriter, Write};

use crossterm::{terminal, cursor, style, event, QueueableCommand};

pub struct Engine {
    writer: BufWriter<io::Stdout>,
    surface: Surface,

    light_pos: (u16, u16),
}

impl Engine {
    pub fn new() -> Self {
        Self {
            writer: BufWriter::with_capacity(
                (size().x * size().y) as usize * 50,
                io::stdout()
            ),
            surface: Surface::new(size()),
            light_pos: (20, 10),
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        let frame_time = Duration::from_millis(8);
        let mut dt;
        let mut now;

        self.prepare_ui();

        while !done {
            now = Instant::now();
            self.render();

            dt = now.elapsed();
            if let Some(remaining) = frame_time.checked_sub(dt) {
                // input polling and processing
                if let Some(command) = input::process_input(remaining) {
                    match command {
                        Command::Quit => {
                            done = true;
                        },
                        Command::Look(x, y) => {
                            self.render_pos(x, y);
                        }
                    };
                }
            }

            dt = now.elapsed();
            if let Some(remaining) = frame_time.checked_sub(dt) {
                std::thread::sleep(remaining);
            }

            self.writer.flush().unwrap();
        }

        self.restore_ui();
    }

    fn render(&mut self) {
        self.render_boundary();

        self.writer
            .queue(cursor::MoveTo(0, 0)).unwrap();

        for element in self.surface.state().iter() {
            self.writer
                .queue(style::Print(element.value)).unwrap();
        }

        self.render_light();
        self.writer.flush().unwrap();
    }

    fn render_pos(&mut self, x: u16, y: u16) {
        self.surface.clear();

        let out = format!("x: {}, y: {}", x, y);
        self.surface.print_str(&out, 1, 1);

        self.surface.draw_line(Vec2::new(20, 10), Vec2::new(x, y), Element{ value: '#' });
    }

    fn render_light(&mut self) {
        self.writer
            .queue(cursor::MoveTo(self.light_pos.0, self.light_pos.1)).unwrap()
            .queue(style::Print("")).unwrap();
    }
    
    fn render_boundary(&mut self) {
        // Bresenham 
        let size = self.surface.size();
        let mut elem: &mut Element; 
        for y in 0..self.surface.size().y {
            elem = self.surface.elem_mut(Vec2{ x: 0, y }).unwrap();
            elem.value = '║';
            elem = self.surface.elem_mut(Vec2{ x: self.surface.size().x - 1, y }).unwrap();
            elem.value = '║';
        }

        for x in 0..self.surface.size().x {
            elem = self.surface.elem_mut(Vec2{ x, y: 0 }).unwrap();
            elem.value = '═';
            elem = self.surface.elem_mut(Vec2{ x, y: self.surface.size().y - 1 }).unwrap();
            elem.value = '═';
        }

        self.surface.elem_mut(Vec2{ x: 0, y: 0 }).unwrap().value = '╔';
        self.surface.elem_mut(Vec2{ x: 0, y: size.y - 1 }).unwrap().value = '╚';
        self.surface.elem_mut(Vec2{ x: size.x - 1, y: size.y - 1 }).unwrap().value = '╝';
        self.surface.elem_mut(Vec2{ x: size.x - 1, y: 0 }).unwrap().value = '╗';
    }

    fn prepare_ui(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.writer
            .queue(terminal::EnterAlternateScreen).unwrap()
            .queue(terminal::Clear(terminal::ClearType::All)).unwrap()
            .queue(event::EnableMouseCapture).unwrap()
            .queue(cursor::Hide).unwrap();

        self.writer.flush().unwrap();
    }

    fn restore_ui(&mut self) {
        self.writer
            .queue(terminal::Clear(terminal::ClearType::All)).unwrap()
            .queue(event::DisableMouseCapture).unwrap()
            .queue(cursor::Show).unwrap()
            .queue(style::ResetColor).unwrap()
            .queue(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();

        self.writer.flush().unwrap();
    }
}

fn size() -> Vec2 {
    let (x, y) = terminal::size().unwrap();
    Vec2::new(x, y)
}
