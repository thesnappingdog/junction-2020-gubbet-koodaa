use crate::app::CustomEvent;
use crate::direction::Direction;
use crate::maze::{Cell, MazeGrid};
use crate::window::AppWindow;
use euclid::Vector2D;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use raqote::{Color, IntPoint};
use uuid::Uuid;
use winit::event::{Event, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Clone)]
struct Player {
    id: Uuid,
    name: String,
    color: Color,
    pos: Vector2D<i32, i32>,
    size: i32,
}

impl Player {
    pub fn new(size: i32, pos: Vector2D<i32, i32>, name: String) -> Player {
        Player {
            id: Uuid::new_v4(),
            color: Color::new(
                255,
                rand::thread_rng().gen_range(0, 255) as u8,
                rand::thread_rng().gen_range(0, 255) as u8,
                rand::thread_rng().gen_range(0, 255) as u8,
            ),
            size,
            pos,
            name,
        }
    }

    pub fn move_to(&mut self, cell: &Cell) {
        self.pos = cell.pos()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct MazeGame {
    maze: MazeGrid,
    camera_pos: IntPoint,
    input: WinitInputHelper,
    cell_size: i32,
    players: Vec<Player>,
    wall_padding: i32,
    is_finished: bool,
    winner: Option<String>,
}

impl MazeGame {
    pub fn new(grid_size: i32, window: &AppWindow) -> MazeGame {
        let maze = MazeGrid::new(grid_size, (0, 0), (grid_size - 1, grid_size - 1));
        let (buffer_width, buffer_height) = window.size();
        let wall_padding = 2;
        // Just some math to get the grid fit height of window
        let cell_size = ((window.size().1 as f32 - 1.05 * grid_size as f32 * wall_padding as f32)
            / (1.05 * grid_size as f32)) as i32;
        let input = WinitInputHelper::new();
        let players = vec![];
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
            is_finished: false,
            winner: None,
        }
    }

    pub fn restart(&mut self) {
        let maze = MazeGrid::new(
            self.maze.size(),
            (0, 0),
            (self.maze.size() - 1, self.maze.size() - 1),
        );
        let mut players = vec![];
        for p in &self.players {
            let mut player = p.clone();
            player.pos = Vector2D::<i32, i32>::new(0, 0);
            players.push(player);
        }
        self.maze = maze;
        self.players = players;
        self.is_finished = false;
        self.winner = None;
    }

    fn add_player(&mut self, name: &str) {
        if self.players.iter().find(|p| &p.name == name).is_none() {
            self.players.push(Player::new(
                self.cell_size / 2,
                self.maze.start_pos(),
                name.to_string(),
            ));
        }
    }

    fn remove_player(&mut self, name: &str) {
        if self.players.iter().find(|p| &p.name == name).is_some() {
            let index = self.players.iter().position(|p| &p.name == name).unwrap();
            self.players.remove(index);
        }
    }

    pub fn handle_custom_events(&mut self, event: &Event<CustomEvent>) {
        match event {
            Event::UserEvent(event) => match event {
                CustomEvent::PlayerConnected(name) => {
                    self.add_player(name);
                    println!("Player connected: {}", name);
                }
                CustomEvent::PlayerDisconnected(name) => {
                    self.remove_player(name);
                    println!("Player disconnected: {}", name);
                }
                CustomEvent::PlayerMove(name, direction) => {
                    if self.players.iter().find(|p| &p.name == name).is_none() {
                        self.add_player(name);
                    }
                    self.try_move(name, *direction);
                    println!("Player move: {} {:?}", name, direction);
                }
            },
            _ => (),
        }
    }

    pub fn handle_input(&mut self, _window: &AppWindow, input: &WinitInputHelper) {
        self.input = input.clone();
    }

    fn try_move(&mut self, player: &str, dir: Direction) {
        let grid_dir = dir.grid_dir();
        let player_pos = self.get_player(player).pos;
        let target_cell = self
            .maze
            .cell_at(player_pos.x + grid_dir.0, player_pos.y + grid_dir.1)
            .cloned();
        let curr_cell = self
            .maze
            .cell_at(player_pos.x, player_pos.y)
            .unwrap()
            .clone();
        if let Some(new_cell) = target_cell {
            if curr_cell.has_link_to(&new_cell) {
                self.get_player(player).move_to(&new_cell);
                if new_cell.pos().x == self.maze.end_pos().x
                    && new_cell.pos().y == self.maze.end_pos().y
                {
                    self.is_finished = true;
                    self.winner = Some(player.to_string());
                }
            }
        }
    }

    fn get_player(&mut self, name: &str) -> &mut Player {
        self.players.iter_mut().find(|p| p.name == name).unwrap()
    }

    pub fn winner_name(&mut self) -> Option<String> {
        if let Some(winner) = self.winner.clone() {
            Some(format!("{}", self.get_player(&winner).name))
        } else {
            None
        }
    }

    pub fn update(&mut self, window: &mut AppWindow, _dt: f64) {
        if self.is_finished {
            return;
        }
        // Update game logic based on inputs here, then render
        self.render_grid(window);
        self.render_players(window);
    }

    fn render_players(&mut self, window: &mut AppWindow) {
        // Shuffle so they are sometimes rendered in different order to show players are in same cell
        self.players.shuffle(&mut thread_rng());
        for player in self.players.iter() {
            let start_x = self.camera_pos.x - self.maze.size() * self.wall_padding / 2
                + player.pos.x * (self.cell_size + self.wall_padding)
                + self.cell_size / 2 as i32
                - player.size / 2;
            let start_y = self.camera_pos.y - self.maze.size() * self.wall_padding / 2
                + player.pos.y * (self.cell_size + self.wall_padding)
                + self.cell_size / 2 as i32
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
                                    if cell.has_link_to(opposite) {
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
                                    if cell.has_link_to(opposite) {
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
                                    if cell.has_link_to(opposite) {
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
                                    if cell.has_link_to(opposite) {
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
