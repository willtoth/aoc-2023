#[derive(Debug, Clone, Copy)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    pub fn rotate_cw(&self) -> Direction {
        match *self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH,
        }
    }

    pub fn rotate_ccw(&self) -> Direction {
        match *self {
            Self::NORTH => Self::WEST,
            Self::EAST => Self::NORTH,
            Self::SOUTH => Self::EAST,
            Self::WEST => Self::SOUTH,
        }
    }
}
