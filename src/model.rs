use graphics::types::Color;

pub struct Pallet {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
    pub randomness: f64,
    pub divisions: u8,
    pub speed: f64,
}

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
    pub speed: f64,
    pub target: [f64; 2],
}

pub enum Direction {
    Left,
    Right,
}

impl Pallet {
    pub fn right_bound(&self) -> f64 {
        return self.x + self.size;
    }

    pub fn left_bound(&self) -> f64 {
        return self.x - self.size;
    }

    pub fn top_bound(&self) -> f64 {
        return self.y - self.size;
    }

    pub fn bottom_bound(&self) -> f64 {
        return self.y + self.size;
    }
}

impl Ball {
    pub fn right_bound(&self) -> f64 {
        return self.x + self.size;
    }

    pub fn left_bound(&self) -> f64 {
        return self.x - self.size;
    }

    pub fn top_bound(&self) -> f64 {
        return self.y - self.size;
    }

    pub fn bottom_bound(&self) -> f64 {
        return self.y + self.size;
    }

    pub fn direction(&self) -> Direction {
        if self.x > self.target[0] {
            return Direction::Left;
        } else {
            return Direction::Right;
        }
    }
}