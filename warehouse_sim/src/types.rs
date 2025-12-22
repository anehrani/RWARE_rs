//! Common types for the warehouse simulation

/// Represents the direction a robot is facing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns the unit vector for the direction.
    /// In our grid, North is -y, South is +y, East is +x, West is -x.
    pub fn vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_vectors() {
        assert_eq!(Direction::North.vector(), (0, -1));
        assert_eq!(Direction::South.vector(), (0, 1));
        assert_eq!(Direction::East.vector(), (1, 0));
        assert_eq!(Direction::West.vector(), (-1, 0));
    }

    #[test]
    fn test_direction_turns() {
        assert_eq!(Direction::North.turn_left(), Direction::West);
        assert_eq!(Direction::North.turn_right(), Direction::East);
        
        assert_eq!(Direction::East.turn_left(), Direction::North);
        assert_eq!(Direction::East.turn_right(), Direction::South);
    }
}
