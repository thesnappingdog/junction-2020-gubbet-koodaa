use crate::window::AppWindow;
use raqote::{IntPoint, Color};
use rayon::prelude::*;
use winit_input_helper::WinitInputHelper;
use crate::maze::MazeGrid;
use crate::direction::Direction;

pub struct MazeGame {
    #[allow(dead_code)]
    maze: MazeGrid,
    camera_pos: IntPoint,
    grid_cursor: IntPoint,
    input: WinitInputHelper,
    cell_size: i32,
}

impl MazeGame {
    pub fn new(grid_size: i32, window: &AppWindow) -> MazeGame {
        let maze = MazeGrid::new(grid_size);
        let (buffer_width, buffer_height) = window.size();
        let cell_size = 38;
        let input = WinitInputHelper::new();
        MazeGame {
            maze,
            camera_pos: IntPoint::new(
                buffer_width as i32 / 2 - grid_size as i32 / 2 * cell_size,
                buffer_height as i32 / 2 - grid_size as i32 / 2 * cell_size,
            ),
            grid_cursor: IntPoint::new(0, 0),
            input,
            cell_size,
        }
    }

    pub fn init(&mut self) {
        //insert resources here etc...
    }

    pub fn handle_input(&mut self, window: &AppWindow, input: &WinitInputHelper) {
        self.input = input.clone();
        if input.mouse().is_some() {
            let hidpi = window.window().scale_factor() as f32;
            let (mouse_x, mouse_y) = {
                let mouse = input.mouse().unwrap();
                (mouse.0 / hidpi, mouse.1 / hidpi)
            };
            let grid_cursor =
                self.cursor_to_game_coords(IntPoint::new(mouse_x as i32, mouse_y as i32));
            self.grid_cursor = grid_cursor;
        }
    }

    pub fn cursor_to_game_coords(&self, mouse_pos: IntPoint) -> IntPoint {
        IntPoint::new(
            (mouse_pos.x - self.camera_pos.x) / self.cell_size,
            (mouse_pos.y - self.camera_pos.y) / self.cell_size,
        )
    }

    #[allow(dead_code)]
    fn resolve_input(&mut self) {}

    pub fn update(&mut self, window: &mut AppWindow, _dt: f64) {
        self.render_grid(window);
    }

    fn render_grid(&mut self, window: &mut AppWindow) {
        let padding = 4;
        for maze_y in 0..self.maze.size() {
            for maze_x in 0..self.maze.size() {
                if let Some(cell) = self.maze.cell_at(maze_x, maze_y) {
                    let start_x = self.camera_pos.x - self.maze.size() * padding / 2 + maze_x * (self.cell_size + padding);
                    let start_y = self.camera_pos.y - self.maze.size() * padding / 2 + maze_y * (self.cell_size + padding);
                    // Render cell
                    self.color_rect(window, start_x, start_y, self.cell_size, self.cell_size, cell.color());
                    // Render doors
                    cell.available_directions().iter().for_each(|dir| {
                        let mut start_x = start_x;
                        let mut start_y = start_y;
                        match dir {
                            Direction::Up => {
                                if let Some(opposite) = self.maze.cell_at(maze_x, maze_y - 1) {
                                    if opposite.available_directions().contains(&Direction::Down) {
                                        start_y = start_y - padding;
                                        self.color_rect(window, start_x, start_y, self.cell_size, padding, cell.color());
                                    }
                                }
                            }
                            Direction::Right => {
                                if let Some(opposite) = self.maze.cell_at(maze_x + 1, maze_y) {
                                    if opposite.available_directions().contains(&Direction::Left) {
                                        start_x = start_x + self.cell_size;
                                        self.color_rect(window, start_x, start_y, padding, self.cell_size, cell.color());
                                    }
                                }
                            }
                            Direction::Left => {
                                if let Some(opposite) = self.maze.cell_at(maze_x - 1, maze_y) {
                                    if opposite.available_directions().contains(&Direction::Right) {
                                        start_x = start_x - padding;
                                        self.color_rect(window, start_x, start_y, padding, self.cell_size, cell.color());
                                    }
                                }
                            }
                            Direction::Down => {
                                if let Some(opposite) = self.maze.cell_at(maze_x, maze_y + 1) {
                                    if opposite.available_directions().contains(&Direction::Up) {
                                        start_y = start_y + self.cell_size;
                                        self.color_rect(window, start_x, start_y, self.cell_size, padding, cell.color());
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    }

    fn color_rect(&self, window: &mut AppWindow, start_x: i32, start_y: i32, rect_width: i32, rect_height: i32, color: Color) {
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
