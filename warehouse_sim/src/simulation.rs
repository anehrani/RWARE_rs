//! Core simulation step implementation

use crate::action::Action;
use crate::entity::Warehouse;
use crate::grid::{Position, in_bounds};

/// Apply a single action for the robot with `robot_id`.
/// Returns `Ok(())` on success or an `Err` with a message describing why the action is illegal.
pub fn step(warehouse: &mut Warehouse, robot_id: usize, action: Action) -> Result<(), String> {
    // Find the index of the robot
    let robot_idx = warehouse
        .robots
        .iter()
        .position(|r| r.id == robot_id)
        .ok_or_else(|| format!("Robot with id {} not found", robot_id))?;

    match action {
        Action::MoveUp => {
            let pos = warehouse.robots[robot_idx].pos;
            try_move(warehouse, robot_idx, Position::new(pos.x, pos.y - 1))
        }
        Action::MoveDown => {
            let pos = warehouse.robots[robot_idx].pos;
            try_move(warehouse, robot_idx, Position::new(pos.x, pos.y + 1))
        }
        Action::MoveLeft => {
            let pos = warehouse.robots[robot_idx].pos;
            try_move(warehouse, robot_idx, Position::new(pos.x - 1, pos.y))
        }
        Action::MoveRight => {
            let pos = warehouse.robots[robot_idx].pos;
            try_move(warehouse, robot_idx, Position::new(pos.x + 1, pos.y))
        }
        Action::Pick => {
            if warehouse.robots[robot_idx].carrying.is_some() {
                return Err("Robot already carrying a shelf".to_string());
            }
            // Find a shelf at the robot's position
            let shelf_idx = warehouse
                .shelves
                .iter()
                .position(|s| s.pos == warehouse.robots[robot_idx].pos)
                .ok_or_else(|| "No shelf at robot position to pick".to_string())?;
            let shelf = warehouse.shelves.remove(shelf_idx);
            warehouse.robots[robot_idx].carrying = Some(shelf.id);
            Ok(())
        }
        Action::Place => {
            let carried = warehouse.robots[robot_idx]
                .carrying
                .ok_or_else(|| "Robot is not carrying any shelf".to_string())?;
            // Find a matching goal at the robot's position
            let goal_idx = warehouse
                .goals
                .iter()
                .position(|g| g.id == carried && g.pos == warehouse.robots[robot_idx].pos)
                .ok_or_else(|| "No matching goal at robot position for the carried shelf".to_string())?;
            // Delivery successful – remove the goal and clear carrying
            warehouse.goals.remove(goal_idx);
            warehouse.robots[robot_idx].carrying = None;
            Ok(())
        }
    }
}

/// Helper to attempt moving a robot to a new position.
fn try_move(warehouse: &mut Warehouse, robot_idx: usize, new_pos: Position) -> Result<(), String> {
    // Bounds check
    if !in_bounds(&new_pos, warehouse.width, warehouse.height) {
        return Err("Move would leave warehouse bounds".to_string());
    }
    // Collision with other robots
    let robot_id = warehouse.robots[robot_idx].id;
    if warehouse
        .robots
        .iter()
        .any(|other| other.id != robot_id && other.pos == new_pos)
    {
        return Err("Another robot occupies the target cell".to_string());
    }
    // Move is legal
    warehouse.robots[robot_idx].pos = new_pos;
    Ok(())
}
