//! Warehouse Bots Library

use warehouse_sim::{Warehouse, Action};

/// A trait for agents that can control a robot in the warehouse simulation.
pub trait Agent {
    /// Decide on an action given the current warehouse state and the robot's ID.
    fn act(&self, warehouse: &Warehouse, robot_id: usize) -> Action;
}

pub mod random;
pub mod greedy;

pub use crate::random::RandomAgent;
pub use crate::greedy::GreedyAgent;
