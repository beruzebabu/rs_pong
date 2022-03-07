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