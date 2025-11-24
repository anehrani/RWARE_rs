//! Grid utilities for the warehouse simulation

/// Coordinate type used for positions
pub type Coord = i32;

/// Represents a location on the warehouse grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

impl Position {
    /// Create a new Position.
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    /// Compute Manhattan distance between two positions.
    pub fn manhattan(&self, other: &Self) -> Coord {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Check if a position is within bounds of a grid of given width and height.
pub fn in_bounds(pos: &Position, width: usize, height: usize) -> bool {
    let w = width as i32;
    let h = height as i32;
    pos.x >= 0 && pos.x < w && pos.y >= 0 && pos.y < h
}
