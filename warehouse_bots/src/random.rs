use crate::Agent;
use warehouse_sim::{Warehouse, Action};
use rand::Rng;

/// An agent that selects actions uniformly at random.
pub struct RandomAgent;

impl RandomAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for RandomAgent {
    fn act(&self, _warehouse: &Warehouse, _robot_id: usize) -> Action {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => Action::MoveUp,
            1 => Action::MoveDown,
            2 => Action::MoveLeft,
            3 => Action::MoveRight,
            4 => Action::Pick,
            _ => Action::Place,
        }
    }
}
