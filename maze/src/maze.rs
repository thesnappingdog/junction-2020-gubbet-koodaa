use crate::direction::Direction;
use euclid::Vector2D;
use rand;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use raqote::Color;

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Vector2D<i32, i32>,
    available_directions: Vec<Direction>,
    color: Color,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell {
            pos: Vector2D::<i32, i32>::new(x, y),
            color: Color::new(255, 100, 100, 100),
            available_directions: vec![],
        }
    }

    pub fn has_link_to(&self, other: &Cell) -> bool {
        if let Some(dir_to_other) = match other.pos.x - self.pos.x {
            0 => match other.pos.y - self.pos.y {
                -1 => Some(Direction::Up),
                1 => Some(Direction::Down),
                _ => None,
            },
            -1 => match other.pos.y - self.pos.y {
                0 => Some(Direction::Left),
                _ => None,
            },
            1 => match other.pos.y - self.pos.y {
                0 => Some(Direction::Right),
                _ => None,
            },
            _ => None,
        } {
            self.available_directions.contains(&dir_to_other)
                && other
                    .available_directions
                    .contains(&dir_to_other.opposite())
        } else {
            false
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn pos(&self) -> Vector2D<i32, i32> {
        self.pos
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn available_directions(&self) -> &Vec<Direction> {
        &self.available_directions
    }

    pub fn available_directions_mut(&mut self) -> &mut Vec<Direction> {
        &mut self.available_directions
    }
}

#[derive(Debug, Clone)]
pub struct MazeGrid {
    grid: Vec<Vec<Cell>>,
    size: i32,
    start: (i32, i32),
}

impl MazeGrid {
    pub fn new(size: i32, start_pos: (i32, i32)) -> MazeGrid {
        let mut grid = vec![];
        for y in 0..size {
            let mut row = vec![];
            for x in 0..size {
                row.push(Cell::new(x, y));
            }
            grid.push(row);
        }
        MazeGrid {
            grid,
            size,
            start: start_pos,
        }
        .generate_maze(start_pos)
    }

    fn generate_maze(mut self, start: (i32, i32)) -> Self {
        if start.0 >= 0 && start.0 < self.size && start.1 >= 0 && start.1 < self.size {
            self.generate_from(start.0, start.1);
        }
        self.grid[start.1 as usize][start.0 as usize].set_color(Color::new(255, 0, 255, 0));
        self
    }

    fn generate_from(&mut self, cur_x: i32, cur_y: i32) {
        let mut directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);
        for dir in directions.iter() {
            let (dir_x, dir_y) = dir.grid_dir();
            let (new_x, new_y) = (cur_x + dir_x, cur_y + dir_y);
            if self.cell_is_unvisited(new_x, new_y) {
                self.cell_link_to(cur_x, cur_y, new_x, new_y, *dir);
                self.generate_from(new_x, new_y);
            }
        }
    }

    // Adds direction to cell's available directions and a corresponding opposite available direction to the other cell
    fn cell_link_to(
        &mut self,
        cell_x: i32,
        cell_y: i32,
        cell_to_x: i32,
        cell_to_y: i32,
        dir: Direction,
    ) {
        let n_vec = self.grid[cell_to_y as usize][cell_to_x as usize].available_directions_mut();
        n_vec.push(dir.opposite());
        n_vec.sort_by(|a, b| a.to_int().cmp(&b.to_int()));
        n_vec.dedup();
        let c_vec = self
            .cell_mut_at(cell_x, cell_y)
            .unwrap()
            .available_directions_mut();
        c_vec.push(dir);
        c_vec.sort_by(|a, b| a.to_int().cmp(&b.to_int()));
        c_vec.dedup();
    }

    fn cell_is_unvisited(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.size
            && y >= 0
            && y < self.size
            && self.grid[y as usize][x as usize]
                .available_directions()
                .len()
                == 0
    }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            None
        } else {
            Some(&self.grid[y as usize][x as usize])
        }
    }

    #[allow(dead_code)]
    pub fn cell_mut_at(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            None
        } else {
            Some(&mut self.grid[y as usize][x as usize])
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}
