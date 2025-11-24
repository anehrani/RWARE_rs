use crate::Agent;
use warehouse_sim::{Warehouse, Action, Position};

/// An agent that greedily moves towards the nearest relevant target (shelf or goal).
pub struct GreedyAgent;

impl GreedyAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for GreedyAgent {
    fn act(&self, warehouse: &Warehouse, robot_id: usize) -> Action {
        let robot = match warehouse.robots.iter().find(|r| r.id == robot_id) {
            Some(r) => r,
            None => return Action::MoveUp, // Fallback if robot not found
        };

        if let Some(shelf_id) = robot.carrying {
            // 1. Carrying a shelf: Find matching goal
            if let Some(goal) = warehouse.goals.iter().find(|g| g.id == shelf_id) {
                if robot.pos == goal.pos {
                    return Action::Place;
                }
                return move_towards(robot.pos, goal.pos);
            }
            // If no matching goal found (shouldn't happen in valid tasks), stay put or move randomly
            Action::MoveUp 
        } else {
            // 2. Not carrying: Find nearest available shelf (one that isn't already at a goal)
            // Note: This simple greedy doesn't check if another robot is targeting it.
            let target_shelf = warehouse.shelves.iter()
                .filter(|s| !warehouse.goals.iter().any(|g| g.pos == s.pos && g.id == s.id)) // Filter out shelves already at goals
                .min_by_key(|s| robot.pos.manhattan(&s.pos));

            if let Some(shelf) = target_shelf {
                if robot.pos == shelf.pos {
                    return Action::Pick;
                }
                return move_towards(robot.pos, shelf.pos);
            }
            
            // No shelves left to move?
            Action::MoveUp
        }
    }
}

fn move_towards(current: Position, target: Position) -> Action {
    let dx = target.x - current.x;
    let dy = target.y - current.y;

    // Simple Manhattan movement: prioritize axis with larger distance
    if dx.abs() > dy.abs() {
        if dx > 0 { Action::MoveRight } else { Action::MoveLeft }
    } else {
        if dy > 0 { Action::MoveDown } else { Action::MoveUp }
    }
}
