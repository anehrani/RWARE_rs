//! Entity definitions for the warehouse simulation

use crate::grid::Position;

/// Represents a robot in the warehouse.
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: usize,
    pub pos: Position,
    /// The id of the shelf the robot is currently carrying, if any.
    pub carrying: Option<usize>,
}

/// Represents a shelf that can be moved by robots.
#[derive(Debug, Clone)]
pub struct Shelf {
    pub id: usize,
    pub pos: Position,
}

/// Represents a goal location where a specific shelf should be placed.
#[derive(Debug, Clone)]
pub struct Goal {
    pub id: usize,
    pub pos: Position,
}

/// The main warehouse state.
#[derive(Debug, Clone)]
pub struct Warehouse {
    pub width: usize,
    pub height: usize,
    pub robots: Vec<Robot>,
    pub shelves: Vec<Shelf>,
    pub goals: Vec<Goal>,
}

impl Warehouse {
    /// Create a new empty warehouse with the given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            robots: Vec::new(),
            shelves: Vec::new(),
            goals: Vec::new(),
        }
    }

    /// Add a robot to the warehouse.
    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    /// Add a shelf to the warehouse.
    pub fn add_shelf(&mut self, shelf: Shelf) {
        self.shelves.push(shelf);
    }

    /// Add a goal to the warehouse.
    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
    }

    /// Create a warehouse from an ASCII representation.
    /// 
    /// Characters:
    /// - `.`: Empty
    /// - `R`: Robot
    /// - `S`: Shelf
    /// - `G`: Goal
    /// - `X`: Shelf + Goal (Goal with matching Shelf on it)
    /// 
    /// Note: IDs are assigned sequentially based on reading order (left-to-right, top-to-bottom).
    pub fn from_ascii(map: &str) -> Result<Self, String> {
        let lines: Vec<&str> = map.trim().lines().map(|l| l.trim()).collect();
        if lines.is_empty() {
            return Err("Empty map string".to_string());
        }
        let height = lines.len();
        let width = lines[0].len();

        let mut warehouse = Warehouse::new(width, height);
        let mut robot_id = 0;
        let mut shelf_id = 0;
        let mut goal_id = 0; // Goals usually match shelves, but for simple parsing we might need a strategy.
                             // For now, let's assume G creates a goal with a new ID, and S creates a shelf with a new ID.
                             // If we want them to match, we might need a more complex format or post-processing.
                             // Let's stick to simple sequential IDs for now.

        for (y, line) in lines.iter().enumerate() {
            if line.len() != width {
                return Err(format!("Inconsistent line length at line {}", y));
            }
            for (x, char) in line.chars().enumerate() {
                let pos = Position::new(x as i32, y as i32);
                match char {
                    '.' => {},
                    'R' => {
                        warehouse.add_robot(Robot { id: robot_id, pos, carrying: None });
                        robot_id += 1;
                    },
                    'S' => {
                        warehouse.add_shelf(Shelf { id: shelf_id, pos });
                        shelf_id += 1;
                    },
                    'G' => {
                        warehouse.add_goal(Goal { id: goal_id, pos });
                        goal_id += 1;
                    },
                    'X' => {
                        warehouse.add_shelf(Shelf { id: shelf_id, pos });
                        warehouse.add_goal(Goal { id: shelf_id, pos }); // Match ID for X
                        shelf_id += 1;
                        // goal_id += 1; // Skip goal_id increment or sync it? 
                        // Let's keep goal_id independent if we see standalone Gs, but for X we match shelf.
                    },
                    _ => return Err(format!("Unknown character '{}' at ({}, {})", char, x, y)),
                }
            }
        }
        Ok(warehouse)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ascii() {
        let map = "
        .R.
        .S.
        .G.
        ";
        let warehouse = Warehouse::from_ascii(map).unwrap();
        assert_eq!(warehouse.width, 3);
        assert_eq!(warehouse.height, 3);
        assert_eq!(warehouse.robots.len(), 1);
        assert_eq!(warehouse.shelves.len(), 1);
        assert_eq!(warehouse.goals.len(), 1);
        
        assert_eq!(warehouse.robots[0].pos, Position::new(1, 0));
        assert_eq!(warehouse.shelves[0].pos, Position::new(1, 1));
        assert_eq!(warehouse.goals[0].pos, Position::new(1, 2));
    }
}
