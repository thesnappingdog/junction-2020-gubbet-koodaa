use euclid::Vector2D;
use raqote::Color;

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Vector2D<f32, f32>,
    color: Color,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell {
            pos: Vector2D::<f32, f32>::new(x as f32, y as f32),
            color: Color::new(255, x as u8, y as u8, x as u8),
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct MazeGrid {
    grid: Vec<Vec<Cell>>,
    size: i32,
}

impl MazeGrid {
    pub fn new(size: i32) -> MazeGrid {
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
        }
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