mod command;
mod engine;
mod input;
mod vec;
mod surface;
mod noise;

use engine::Engine;

fn main() {
    Engine::new().run();
}
