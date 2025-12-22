use warehouse_sim::{Warehouse, Robot, Shelf, Goal, Position, Action, Direction, step};

fn main() {
    println!("=== Pick and Place Example ===");

    // Initialize a 10x10 warehouse
    let mut warehouse = Warehouse::new(10, 10);

    // Setup: Robot at (0,0), Shelf at (2,0), Goal at (4,0)
    let robot_id = 1;
    let shelf_id = 101;
    
    warehouse.add_robot(Robot { 
        id: robot_id, 
        pos: Position::new(0, 0), 
        direction: Direction::East,
        carrying: None 
    });
    
    warehouse.add_shelf(Shelf { 
        id: shelf_id, 
        pos: Position::new(2, 0) 
    });
    
    warehouse.add_goal(Goal { 
        id: shelf_id, 
        pos: Position::new(4, 0) 
    });

    println!("Setup:");
    println!("Robot at {:?}", warehouse.robots[0].pos);
    println!("Shelf {} at {:?}", warehouse.shelves[0].id, warehouse.shelves[0].pos);
    println!("Goal for Shelf {} at {:?}", warehouse.goals[0].id, warehouse.goals[0].pos);

    // Plan: Move Right x2 -> Pick -> Move Right x2 -> Place
    let plan = vec![
        Action::MoveRight,
        Action::MoveRight,
        Action::Pick,
        Action::MoveRight,
        Action::MoveRight,
        Action::Place,
    ];

    for (i, action) in plan.into_iter().enumerate() {
        println!("\nStep {}: Applying {:?}", i + 1, action);
        if let Err(e) = step(&mut warehouse, robot_id, action) {
            println!("Failed to execute action: {}", e);
            return;
        }
        
        let robot = &warehouse.robots[0];
        println!("Robot Pos: {:?}, Carrying: {:?}", robot.pos, robot.carrying);
    }

    println!("\n=== Mission Complete ===");
    println!("Shelves remaining: {}", warehouse.shelves.len());
    println!("Goals remaining: {}", warehouse.goals.len());
    
    if warehouse.shelves.is_empty() && warehouse.goals.is_empty() {
        println!("SUCCESS: Shelf delivered to goal!");
    } else {
        println!("FAILURE: Mission not accomplished.");
    }
}
