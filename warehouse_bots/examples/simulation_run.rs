use warehouse_sim::{Warehouse, Robot, step};
use warehouse_bots::{Agent, GreedyAgent};
use std::{thread, time};

fn main() {
    println!("=== Warehouse Simulation Runner ===");

    // Initialize map
    let map = "
    .R...
    .S.G.
    .....
    ";
    let mut warehouse = Warehouse::from_ascii(map).unwrap();
    let agent = GreedyAgent::new();
    let robot_id = 0;

    println!("Initial State:");
    print_state(&warehouse);

    for i in 1..=20 {
        println!("\nStep {}", i);
        
        // Agent decides action
        let action = agent.act(&warehouse, robot_id);
        println!("Agent chose: {:?}", action);

        // Apply action
        match step(&mut warehouse, robot_id, action) {
            Ok(_) => println!("Action successful."),
            Err(e) => println!("Action failed: {}", e),
        }

        print_state(&warehouse);

        // Check completion
        if warehouse.shelves.is_empty() && warehouse.goals.is_empty() {
            println!("\nSUCCESS: All tasks completed!");
            break;
        }

        // Optional: sleep for visualization
        thread::sleep(time::Duration::from_millis(200));
    }
}

fn print_state(w: &Warehouse) {
    if let Some(r) = w.robots.first() {
        println!("Robot at {:?}, Carrying: {:?}", r.pos, r.carrying);
    }
    println!("Shelves: {}, Goals: {}", w.shelves.len(), w.goals.len());
}
