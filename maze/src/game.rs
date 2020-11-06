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
        let cell_size = 10;
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
        let (width, _) = window.size();
        let framebuffer = window.framebuffer();
        // ToDo Render grid here
    }
}
