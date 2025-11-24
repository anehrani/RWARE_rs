//! Warehouse Simulation Library

pub mod grid;
pub mod entity;
pub mod action;
pub mod simulation;

pub use crate::grid::Position;
pub use crate::entity::{Warehouse, Robot, Shelf, Goal};
pub use crate::action::Action;
pub use crate::simulation::step;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_and_pick_place() {
        let mut warehouse = Warehouse::new(5, 5);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), carrying: None });
        warehouse.add_shelf(Shelf { id: 0, pos: Position::new(2, 1) });
        warehouse.add_goal(Goal { id: 0, pos: Position::new(3, 1) });

        // Move right to shelf
        step(&mut warehouse, 0, Action::MoveRight).unwrap();
        // Pick shelf
        step(&mut warehouse, 0, Action::Pick).unwrap();
        // Move right to goal
        step(&mut warehouse, 0, Action::MoveRight).unwrap();
        // Place shelf
        step(&mut warehouse, 0, Action::Place).unwrap();

        assert!(warehouse.shelves.is_empty());
        assert!(warehouse.goals.is_empty());
        assert!(warehouse.robots[0].carrying.is_none());
        assert_eq!(warehouse.robots[0].pos, Position::new(3, 1));
    }
}
