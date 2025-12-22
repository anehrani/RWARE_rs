use warehouse_sim::{Warehouse, Robot, Position, Action, Direction, step};

fn main() {
    println!("=== Simple Movement Example ===");
    
    // Initialize a 5x5 warehouse
    let mut warehouse = Warehouse::new(5, 5);
    
    // Add a robot at (0, 0)
    let robot_id = 0;
    warehouse.add_robot(Robot { 
        id: robot_id, 
        pos: Position::new(0, 0), 
        direction: Direction::South,
        carrying: None 
    });
    
    println!("Initial State: {:?}", warehouse.robots[0]);

    // Define a sequence of moves: Right, Right, Down, Down
    let actions = vec![
        Action::MoveRight,
        Action::MoveRight,
        Action::MoveDown,
        Action::MoveDown,
    ];

    for (i, action) in actions.into_iter().enumerate() {
        println!("\nStep {}: Applying {:?}", i + 1, action);
        match step(&mut warehouse, robot_id, action) {
            Ok(_) => println!("Success! New Pos: {:?}", warehouse.robots[0].pos),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!("\nFinal State: {:?}", warehouse.robots[0]);
}
