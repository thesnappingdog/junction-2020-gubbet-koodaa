mod app;
mod direction;
mod game;
mod maze;
mod gui;
mod window;

use app::App;
use pixels::Error;

pub fn main() -> Result<(), Error> {
    env_logger::init();
    App::build().run("Maze Craze", 1280, 720, 16)
}
