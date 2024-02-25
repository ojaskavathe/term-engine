use crate::input;
use crate::command::Command;
use crate::vec::Vec2;
use crate::surface::{Surface, Element};
use crate::noise::perlin_noise;

use std::time::{Duration, Instant};
use std::io::{self, BufWriter, Write};

use crossterm::{terminal, cursor, style, event, QueueableCommand};

pub struct Engine {
    writer: BufWriter<io::Stdout>,
    surface: Surface,

    height_map: Surface,
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
            height_map: Surface::new(size()),
            light_pos: (20, 10),
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        let frame_time = Duration::from_millis(8);
        let mut dt;
        let mut now;

        self.prepare_ui();
        self.generate_heightmap();

        self.render_heightmap();

        while !done {
            now = Instant::now();

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

        // let last_bg = self.surface.dd

        for element in self.surface.state().iter() {
            self.writer
                .queue(style::SetBackgroundColor(element.background)).unwrap()
                .queue(style::Print(element.value)).unwrap();
        }

        self.writer.flush().unwrap();
    }

    fn render_heightmap(&mut self) {
        self.writer
            .queue(cursor::MoveTo(0, 0)).unwrap();

        for element in self.height_map.state().iter() {
            self.writer
                .queue(style::SetBackgroundColor(element.background)).unwrap()
                .queue(style::Print(element.value)).unwrap();
        }

        self.writer.flush().unwrap();
    }

    fn render_pos(&mut self, x: u16, y: u16) {
        let out = format!("x: {}, y: {}    ", x, y);
        self.surface.print_str(&out, 1, 1);

        // set light
        // self.surface.set(Vec2::new(x, y), Element{ value: '☼', background: style::Color::Black });

        // self.surface.draw_line(Vec2::new(20, 10), Vec2::new(x, y), Element{ value: '#' });
        self.writer.flush().unwrap();
    }

    fn generate_heightmap(&mut self) {
        let size = size();
        for x in (1..size.x).step_by(2) {
            for y in 1..size.y {
                let mut n = 0.0;
                let mut a = 1.0;
                let mut f = 0.001;

                for _o in 0..8 {
                    let v = a * perlin_noise(x as f64 * f, y as f64 * f);
                    n += v;
                    
                    a *= 0.5;
                    f *= 2.0;
                };

                n += 1.0;
                n *= 0.5;

                let mut d = if size.x < size.y { size.x as f64 } else { size.y as f64 };
                d = (d * d) / 4.0;
                let dx = (size.x as f64 / 2.0) - x as f64;
                let dy = (size.y as f64 / 2.0) - y as f64;
                let dsqr = (dx * dx * 0.1) + (dy * dy);
                let outer_scaling = 1.0 - (1.0 / d) * dsqr;
                n *= outer_scaling;

                if n < 0.3 { 
                    self.height_map.set(Vec2{x, y}, Element{ value:' ', background: style::Color::Rgb{r: 0, g: 105, b: 148}});
                    self.height_map.set(Vec2{x:x+1, y}, Element{ value:' ', background: style::Color::Rgb{r: 0, g: 105, b: 148} });
                } else if n < 0.4 {
                    self.height_map.set(Vec2{x, y}, Element{ value:' ', background: style::Color::Rgb{r: 194, g: 174, b: 128} });
                    self.height_map.set(Vec2{x:x+1, y}, Element{ value:' ', background: style::Color::Rgb{r: 194, g: 174, b: 128} });
                } else if n < 0.55 {
                    self.height_map.set(Vec2{x, y}, Element{ value:' ', background: style::Color::Rgb{r: 2, g: 48, b: 32} });
                    self.height_map.set(Vec2{x:x+1, y}, Element{ value:' ', background: style::Color::Rgb{r: 2, g: 48, b: 32} });
                } else {
                    self.height_map.set(Vec2{x, y}, Element{ value:' ', background: style::Color::White });
                    self.height_map.set(Vec2{x:x+1, y}, Element{ value:' ', background: style::Color::White });
                }
            }
        }
    }
    
    fn render_boundary(&mut self) {
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
            .queue(style::ResetColor).unwrap()
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
