use crate::direction::Direction;
use crate::maze::MazeGrid;
use crate::window::AppWindow;
use euclid::Vector2D;
use rand::Rng;
use raqote::{Color, IntPoint};
use rayon::prelude::*;
use std::borrow::BorrowMut;
use uuid::Uuid;
use winit_input_helper::WinitInputHelper;

struct Player {
    id: Uuid,
    is_turn: bool,
    color: Color,
    pos: Vector2D<f32, f32>,
    size: i32,
}

impl Player {
    pub fn new(size: i32, pos: Vector2D<f32, f32>) -> Player {
        Player {
            id: Uuid::new_v4(),
            is_turn: false,
            color: Color::new(
                255,
                rand::thread_rng().gen_range(0, 255) as u8,
                rand::thread_rng().gen_range(0, 255) as u8,
                rand::thread_rng().gen_range(0, 255) as u8,
            ),
            size,
            pos,
        }
    }
    pub fn set_turn(&mut self) {
        self.is_turn = true
    }

    pub fn end_turn(&mut self) {
        self.is_turn = false
    }
}

pub struct MazeGame {
    #[allow(dead_code)]
    maze: MazeGrid,
    camera_pos: IntPoint,
    input: WinitInputHelper,
    cell_size: i32,
    players: Vec<Player>,
    wall_padding: i32,
}

impl MazeGame {
    pub fn new(grid_size: i32, window: &AppWindow) -> MazeGame {
        let maze = MazeGrid::new(grid_size, (0, 0));
        let (buffer_width, buffer_height) = window.size();
        let wall_padding = 2;
        // Just some math to get the grid fit height of window
        let cell_size = ((window.size().1 as f32 - 1.05 * grid_size as f32 * wall_padding as f32)
            / (1.05 * grid_size as f32)) as i32;
        let input = WinitInputHelper::new();
        let mut players = vec![
            Player::new(cell_size / 2, Vector2D::<f32, f32>::new(0., 0.)),
            Player::new(cell_size / 2, Vector2D::<f32, f32>::new(1., 0.)),
        ];
        players[0].set_turn();
        MazeGame {
            maze,
            camera_pos: IntPoint::new(
                buffer_width as i32 / 2 - grid_size as i32 / 2 * cell_size,
                buffer_height as i32 / 2 - grid_size as i32 / 2 * cell_size,
            ),
            input,
            cell_size,
            players,
            wall_padding,
        }
    }

    pub fn init(&mut self) {
        //insert resources here etc...
    }

    pub fn handle_input(&mut self, _window: &AppWindow, input: &WinitInputHelper) {
        self.input = input.clone();
        // Save inputs here
    }

    #[allow(dead_code)]
    fn resolve_input(&mut self) {
        // Handle input state here
    }

    pub fn update(&mut self, window: &mut AppWindow, _dt: f64) {
        // Update game logic based on inputs here, then render
        self.render_grid(window);
        self.render_players(window);
    }

    fn render_players(&mut self, window: &mut AppWindow) {
        for player in self.players.iter() {
            let start_x = self.camera_pos.x - self.maze.size() * self.wall_padding / 2
                + player.pos.x as i32 * (self.cell_size + self.wall_padding)
                + self.cell_size / 2
                - player.size / 2;
            let start_y = self.camera_pos.y - self.maze.size() * self.wall_padding / 2
                + player.pos.y as i32 * (self.cell_size + self.wall_padding)
                + self.cell_size / 2
                - player.size / 2;
            self.color_rect(
                window,
                start_x,
                start_y,
                player.size,
                player.size,
                player.color,
            );
        }
    }

    fn render_grid(&mut self, window: &mut AppWindow) {
        for maze_y in 0..self.maze.size() {
            for maze_x in 0..self.maze.size() {
                if let Some(cell) = self.maze.cell_at(maze_x, maze_y) {
                    let start_x = self.camera_pos.x - self.maze.size() * self.wall_padding / 2
                        + maze_x * (self.cell_size + self.wall_padding);
                    let start_y = self.camera_pos.y - self.maze.size() * self.wall_padding / 2
                        + maze_y * (self.cell_size + self.wall_padding);
                    // Render cell
                    self.color_rect(
                        window,
                        start_x,
                        start_y,
                        self.cell_size,
                        self.cell_size,
                        cell.color(),
                    );
                    // Render doors
                    cell.available_directions().iter().for_each(|dir| {
                        let mut start_x = start_x;
                        let mut start_y = start_y;
                        match dir {
                            Direction::Up => {
                                if let Some(opposite) = self.maze.cell_at(maze_x, maze_y - 1) {
                                    if opposite.available_directions().contains(&Direction::Down) {
                                        start_y = start_y - self.wall_padding;
                                        self.color_rect(
                                            window,
                                            start_x,
                                            start_y,
                                            self.cell_size,
                                            self.wall_padding,
                                            cell.color(),
                                        );
                                    }
                                }
                            }
                            Direction::Right => {
                                if let Some(opposite) = self.maze.cell_at(maze_x + 1, maze_y) {
                                    if opposite.available_directions().contains(&Direction::Left) {
                                        start_x = start_x + self.cell_size;
                                        self.color_rect(
                                            window,
                                            start_x,
                                            start_y,
                                            self.wall_padding,
                                            self.cell_size,
                                            cell.color(),
                                        );
                                    }
                                }
                            }
                            Direction::Left => {
                                if let Some(opposite) = self.maze.cell_at(maze_x - 1, maze_y) {
                                    if opposite.available_directions().contains(&Direction::Right) {
                                        start_x = start_x - self.wall_padding;
                                        self.color_rect(
                                            window,
                                            start_x,
                                            start_y,
                                            self.wall_padding,
                                            self.cell_size,
                                            cell.color(),
                                        );
                                    }
                                }
                            }
                            Direction::Down => {
                                if let Some(opposite) = self.maze.cell_at(maze_x, maze_y + 1) {
                                    if opposite.available_directions().contains(&Direction::Up) {
                                        start_y = start_y + self.cell_size;
                                        self.color_rect(
                                            window,
                                            start_x,
                                            start_y,
                                            self.cell_size,
                                            self.wall_padding,
                                            cell.color(),
                                        );
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    }

    fn color_rect(
        &self,
        window: &mut AppWindow,
        start_x: i32,
        start_y: i32,
        rect_width: i32,
        rect_height: i32,
        color: Color,
    ) {
        let (width, height) = window.size();
        let framebuffer = window.framebuffer();
        for y in start_y..(start_y + rect_height) {
            for x in start_x..(start_x + rect_width) {
                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }
                let pixel_index = (y * width as i32 * 4 + x * 4) as usize;
                framebuffer[pixel_index] = color.r();
                framebuffer[pixel_index + 1] = color.g();
                framebuffer[pixel_index + 2] = color.b();
                framebuffer[pixel_index + 3] = color.a();
            }
        }
    }
}
