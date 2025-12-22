//! Warehouse Simulation Library

pub mod grid;
pub mod entity;
pub mod action;
pub mod simulation;
pub mod clock_sim;
pub mod types;

pub use crate::grid::Position;
pub use crate::entity::{Warehouse, Robot, Shelf, Goal};
pub use crate::action::Action;
pub use crate::simulation::step;
pub use crate::clock_sim::{step_with_clock, step_single_with_clock};
pub use crate::types::Direction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_and_pick_place() {
        let mut warehouse = Warehouse::new(5, 5);
        warehouse.add_robot(Robot { 
            id: 0, 
            pos: Position::new(1, 1), 
            direction: Direction::East, 
            carrying: None 
        });
        warehouse.add_shelf(Shelf { id: 0, pos: Position::new(2, 1) });
        warehouse.add_goal(Goal { id: 0, pos: Position::new(3, 1) });

        // Robot starts at (1,1) facing East. 
        // Forward should move to (2,1)
        step(&mut warehouse, 0, Action::Forward).unwrap();
        assert_eq!(warehouse.robots[0].pos, Position::new(2, 1));

        // ToggleLoad should pick shelf at (2,1)
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        assert_eq!(warehouse.robots[0].carrying, Some(0));
        assert!(warehouse.shelves.is_empty());

        // Forward to (3,1)
        step(&mut warehouse, 0, Action::Forward).unwrap();
        assert_eq!(warehouse.robots[0].pos, Position::new(3, 1));

        // ToggleLoad should place shelf at (3,1) goal
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        assert!(warehouse.goals.is_empty());
        assert!(warehouse.robots[0].carrying.is_none());
    }
}
