mod app;
mod direction;
mod game;
mod gui;
mod maze;
mod window;

use app::App;
use log::error;
use pixels::Error;
use std::env;
use std::process::exit;

pub fn main() -> Result<(), Error> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let maze_size = args[1].parse::<i32>().expect("Invalid maze size");
            if maze_size < 0 || maze_size > 50 {
                error!("Invalid maze size, can't be less than 0 or larger than 50");
                exit(0);
            }
            App::build().run("Maze Craze", 1280, 720, maze_size)
        }
        _ => App::build().run("Maze Craze", 1280, 720, 16),
    }
}
