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
}
