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
        Action::Pick => pick_shelf(warehouse, robot_idx),
        Action::Place => place_shelf(warehouse, robot_idx),
        
        // --- RWARE actions ---
        Action::TurnLeft => {
            warehouse.robots[robot_idx].direction = warehouse.robots[robot_idx].direction.turn_left();
            Ok(())
        }
        Action::TurnRight => {
            warehouse.robots[robot_idx].direction = warehouse.robots[robot_idx].direction.turn_right();
            Ok(())
        }
        Action::Forward => {
            let dir = warehouse.robots[robot_idx].direction;
            let pos = warehouse.robots[robot_idx].pos;
            let (dx, dy) = dir.vector();
            try_move(warehouse, robot_idx, Position::new(pos.x + dx, pos.y + dy))
        }
        Action::ToggleLoad => {
            if warehouse.robots[robot_idx].carrying.is_some() {
                place_shelf(warehouse, robot_idx)
            } else {
                pick_shelf(warehouse, robot_idx)
            }
        }
    }
}

fn pick_shelf(warehouse: &mut Warehouse, robot_idx: usize) -> Result<(), String> {
    if warehouse.robots[robot_idx].carrying.is_some() {
        return Err("Robot already carrying a shelf".to_string());
    }
    let shelf_idx = warehouse
        .shelves
        .iter()
        .position(|s| s.pos == warehouse.robots[robot_idx].pos)
        .ok_or_else(|| "No shelf at robot position to pick".to_string())?;
    let shelf = warehouse.shelves.remove(shelf_idx);
    warehouse.robots[robot_idx].carrying = Some(shelf.id);
    Ok(())
}

fn place_shelf(warehouse: &mut Warehouse, robot_idx: usize) -> Result<(), String> {
    let carried = warehouse.robots[robot_idx]
        .carrying
        .ok_or_else(|| "Robot is not carrying any shelf".to_string())?;
    
    // Check if there is a matching goal at the Current Position
    let goal_idx = warehouse
        .goals
        .iter()
        .position(|g| g.id == carried && g.pos == warehouse.robots[robot_idx].pos);

    if let Some(idx) = goal_idx {
        // Delivery successful – remove the goal and clear carrying
        warehouse.goals.remove(idx);
        warehouse.robots[robot_idx].carrying = None;
        Ok(())
    } else {
        // If no matching goal, maybe we can just drop it? 
        // For now, let's keep it strict or allow dropping if no other shelf is there
        if warehouse.shelves.iter().any(|s| s.pos == warehouse.robots[robot_idx].pos) {
            return Err("Cannot drop shelf: cell already has a shelf".to_string());
        }
        let shelf_id = warehouse.robots[robot_idx].carrying.take().unwrap();
        warehouse.shelves.push(crate::entity::Shelf {
            id: shelf_id,
            pos: warehouse.robots[robot_idx].pos,
        });
        Ok(())
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

    // Collision with shelves
    // IF the robot is carrying a shelf, it CANNOT move into a cell with another shelf.
    // IF the robot is NOT carrying a shelf, it CAN move under shelves.
    if warehouse.robots[robot_idx].carrying.is_some() {
        if warehouse.shelves.iter().any(|s| s.pos == new_pos) {
            return Err("Robot carrying a shelf cannot move through another shelf".to_string());
        }
    }

    // Move is legal
    warehouse.robots[robot_idx].pos = new_pos;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Warehouse, Robot, Shelf, Goal};
    use crate::grid::Position;
    use crate::types::Direction;

    #[test]
    fn test_rotation_actions() {
        let mut warehouse = Warehouse::new(3, 3);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), direction: Direction::North, carrying: None });
        
        step(&mut warehouse, 0, Action::TurnRight).unwrap();
        assert_eq!(warehouse.robots[0].direction, Direction::East);
        
        step(&mut warehouse, 0, Action::TurnLeft).unwrap();
        assert_eq!(warehouse.robots[0].direction, Direction::North);
    }

    #[test]
    fn test_forward_movement() {
        let mut warehouse = Warehouse::new(3, 3);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), direction: Direction::South, carrying: None });
        
        step(&mut warehouse, 0, Action::Forward).unwrap();
        assert_eq!(warehouse.robots[0].pos, Position::new(1, 2));
    }

    #[test]
    fn test_shelf_dynamics_unloaded() {
        let mut warehouse = Warehouse::new(3, 3);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(0, 0), direction: Direction::East, carrying: None });
        warehouse.add_shelf(Shelf { id: 10, pos: Position::new(1, 0) });
        
        // Unloaded robot CAN move under shelf
        step(&mut warehouse, 0, Action::Forward).unwrap();
        assert_eq!(warehouse.robots[0].pos, Position::new(1, 0));
    }

    #[test]
    fn test_shelf_dynamics_loaded() {
        let mut warehouse = Warehouse::new(3, 3);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(0, 0), direction: Direction::East, carrying: Some(1) });
        warehouse.add_shelf(Shelf { id: 10, pos: Position::new(1, 0) });
        
        // Loaded robot CANNOT move through another shelf
        let res = step(&mut warehouse, 0, Action::Forward);
        assert!(res.is_err());
        assert_eq!(warehouse.robots[0].pos, Position::new(0, 0));
    }

    #[test]
    fn test_toggle_load_flow() {
        let mut warehouse = Warehouse::new(3, 3);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), direction: Direction::North, carrying: None });
        warehouse.add_shelf(Shelf { id: 5, pos: Position::new(1, 1) });
        warehouse.add_goal(Goal { id: 5, pos: Position::new(2, 2) });

        // Pick
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        assert_eq!(warehouse.robots[0].carrying, Some(5));
        assert_eq!(warehouse.shelves.len(), 0);

        // Move to empty spot and drop (since it's not the goal)
        warehouse.robots[0].pos = Position::new(0, 0);
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        assert_eq!(warehouse.robots[0].carrying, None);
        assert_eq!(warehouse.shelves.len(), 1);
        assert_eq!(warehouse.shelves[0].pos, Position::new(0, 0));

        // Pick again
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        
        // Move to goal and drop (delivery)
        warehouse.robots[0].pos = Position::new(2, 2);
        step(&mut warehouse, 0, Action::ToggleLoad).unwrap();
        assert_eq!(warehouse.robots[0].carrying, None);
        assert_eq!(warehouse.goals.len(), 0);
        assert_eq!(warehouse.shelves.len(), 0); // Delivered shelf is removed from world in this model
    }
}
