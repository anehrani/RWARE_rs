//! Actions that a robot can perform in the warehouse simulation

/// Possible actions a robot may take during a simulation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Move one cell up (decrease y coordinate).
    MoveUp,
    /// Move one cell down (increase y coordinate).
    MoveDown,
    /// Move one cell left (decrease x coordinate).
    MoveLeft,
    /// Move one cell right (increase x coordinate).
    MoveRight,
    /// Pick up a shelf located at the robot's current position.
    Pick,
    /// Place the carried shelf onto a matching goal at the robot's position.
    Place,
    
    // --- New RWARE-style actions ---
    /// Rotate the robot 90 degrees left.
    TurnLeft,
    /// Rotate the robot 90 degrees right.
    TurnRight,
    /// Move one cell forward in the direction the robot is facing.
    Forward,
    /// Pick up or place a shelf.
    ToggleLoad,
}
