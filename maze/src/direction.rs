#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(d: usize) -> Direction {
        match d {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::Up,
        }
    }
    pub fn to_int(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
    pub fn next(&self) -> Direction {
        let dir = self.to_int();
        let mut new_dir = dir + 1;
        if new_dir > 3 {
            new_dir = 0
        }
        Direction::new(new_dir)
    }
    #[allow(dead_code)]
    pub fn prev(&self) -> Direction {
        let dir = self.to_int();
        if dir == 0 {
            Direction::new(3)
        } else {
            Direction::new(dir - 1)
        }
    }

    pub fn grid_dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    pub fn grid_dir_opposite(&self) -> (i32, i32) {
        self.opposite().grid_dir()
    }

    #[allow(dead_code)]
    pub fn opposite(&self) -> Direction {
        self.next().next()
    }
}
