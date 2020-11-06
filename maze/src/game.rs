use crate::window::AppWindow;
use raqote::IntPoint;
use rayon::prelude::*;
use winit_input_helper::WinitInputHelper;
use crate::maze::MazeGrid;

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
        let cell_size = 15;
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
        let (width, height) = window.size();
        let padding = 1;
        let framebuffer = window.framebuffer();
        for maze_y in 0..self.maze.size() {
            for maze_x in 0..self.maze.size() {
                if let Some(cell) = self.maze.cell_at(maze_x, maze_y) {
                    let color = cell.color();
                    let start_x = self.camera_pos.x + maze_x * (self.cell_size + padding);
                    let start_y = self.camera_pos.y + maze_y * (self.cell_size + padding);
                    for y in start_y..(start_y + self.cell_size) {
                        for x in start_x..(start_x + self.cell_size) {
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
        }
    }
}
