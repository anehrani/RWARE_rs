//! Enhanced simulation step with clock management

use crate::action::Action;
use crate::entity::Warehouse;
use crate::simulation::step as basic_step;

/// Execute a single simulation step for all robots, advancing the clock.
/// This is a higher-level function that manages the global clock.
///
/// # Arguments
/// * `warehouse` - The warehouse state
/// * `actions` - A vector of (robot_id, action) pairs to execute simultaneously
///
/// # Returns
/// A vector of results for each action, in the same order as the input
pub fn step_with_clock(
    warehouse: &mut Warehouse,
    actions: Vec<(usize, Action)>,
) -> Vec<Result<(), String>> {
    let mut results = Vec::new();
    
    // Execute all actions
    for (robot_id, action) in actions {
        let result = basic_step(warehouse, robot_id, action);
        results.push(result);
    }
    
    // Advance the clock after all actions are processed
    warehouse.tick();
    
    results
}

/// Execute a single action and advance the clock.
/// Use this for sequential robot control.
pub fn step_single_with_clock(
    warehouse: &mut Warehouse,
    robot_id: usize,
    action: Action,
) -> Result<(), String> {
    let result = basic_step(warehouse, robot_id, action);
    warehouse.tick();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Warehouse, Robot, Position, Action};

    #[test]
    fn test_clock_advances() {
        let mut warehouse = Warehouse::new(5, 5);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), carrying: None });
        
        assert_eq!(warehouse.current_time(), 0);
        
        step_single_with_clock(&mut warehouse, 0, Action::MoveRight).unwrap();
        assert_eq!(warehouse.current_time(), 1);
        
        step_single_with_clock(&mut warehouse, 0, Action::MoveRight).unwrap();
        assert_eq!(warehouse.current_time(), 2);
    }

    #[test]
    fn test_simultaneous_actions() {
        let mut warehouse = Warehouse::new(5, 5);
        warehouse.add_robot(Robot { id: 0, pos: Position::new(1, 1), carrying: None });
        warehouse.add_robot(Robot { id: 1, pos: Position::new(3, 3), carrying: None });
        
        let actions = vec![
            (0, Action::MoveRight),
            (1, Action::MoveLeft),
        ];
        
        let results = step_with_clock(&mut warehouse, actions);
        
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert_eq!(warehouse.current_time(), 1);
        assert_eq!(warehouse.robots[0].pos, Position::new(2, 1));
        assert_eq!(warehouse.robots[1].pos, Position::new(2, 3));
    }
}
