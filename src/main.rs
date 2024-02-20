mod command;
mod engine;
mod input;

use engine::Engine;

use std::io::stdout;

const WIDTH: u16 = 30;
const HEIGHT: u16 = 10;

fn main() {
    Engine::new(stdout(), WIDTH, HEIGHT).run();
}
