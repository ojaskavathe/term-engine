mod command;
mod engine;
mod input;
mod vec;
mod surface;

use engine::Engine;

fn main() {
    Engine::new().run();
}
