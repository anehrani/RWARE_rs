use warehouse_sim::{Warehouse, Robot, Shelf, Position, Action, step_with_clock};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let width = 15;
    let height = 15;
    let num_shelves = 10;
    let num_robots = 3;

    println!("Clock-based Simulation Demo");
    println!("Grid: {}x{}, Shelves: {}, Robots: {}\n", width, height, num_shelves, num_robots);

    let mut warehouse = Warehouse::new(width, height);
    let mut rng = rand::thread_rng();

    // Place shelves randomly
    let mut occupied_shelves = HashSet::new();
    while warehouse.shelves.len() < num_shelves {
        let x = rng.gen_range(0..width) as i32;
        let y = rng.gen_range(0..height) as i32;
        let pos = Position::new(x, y);
        
        if !occupied_shelves.contains(&pos) {
            occupied_shelves.insert(pos);
            warehouse.add_shelf(Shelf {
                id: warehouse.shelves.len(),
                pos,
            });
        }
    }

    // Place robots randomly
    let mut robot_positions = HashSet::new();
    while warehouse.robots.len() < num_robots {
        let x = rng.gen_range(0..width) as i32;
        let y = rng.gen_range(0..height) as i32;
        let pos = Position::new(x, y);

        if !robot_positions.contains(&pos) {
            robot_positions.insert(pos);
            warehouse.add_robot(Robot {
                id: warehouse.robots.len(),
                pos,
                carrying: None,
            });
        }
    }

    let mut visited_shelves = HashSet::new();
    let max_time = 100;

    println!("Starting simulation at time t={}", warehouse.current_time());
    println!("Initial robot positions:");
    for robot in &warehouse.robots {
        println!("  Robot {} at {:?}", robot.id, robot.pos);
    }
    println!();

    // Simulation loop using clock
    while visited_shelves.len() < num_shelves && warehouse.current_time() < max_time {
        // Prepare actions for all robots
        let mut actions = Vec::new();
        
        for i in 0..warehouse.robots.len() {
            let robot_id = warehouse.robots[i].id;
            let robot_pos = warehouse.robots[i].pos;

            // Find nearest unvisited shelf
            let target = warehouse.shelves.iter()
                .filter(|s| !visited_shelves.contains(&s.id))
                .min_by_key(|s| s.pos.manhattan(&robot_pos));

            if let Some(target_shelf) = target {
                let target_pos = target_shelf.pos;
                
                // Determine best action
                if let Some(action) = choose_action(robot_pos, target_pos) {
                    actions.push((robot_id, action));
                }
            }
        }

        // Execute all actions simultaneously and advance clock
        let results = step_with_clock(&mut warehouse, actions);
        
        // Check for new visits
        let prev_count = visited_shelves.len();
        check_visits(&warehouse, &mut visited_shelves);
        let new_visits = visited_shelves.len() - prev_count;
        
        // Print status every 10 steps or when shelves are visited
        if warehouse.current_time() % 10 == 0 || new_visits > 0 {
            println!("t={:3} | Shelves visited: {}/{} | Successful moves: {}/{}", 
                warehouse.current_time(),
                visited_shelves.len(),
                num_shelves,
                results.iter().filter(|r| r.is_ok()).count(),
                results.len()
            );
            
            if new_visits > 0 {
                println!("       └─ {} new shelf(s) visited!", new_visits);
            }
        }
    }

    println!("\n=== Simulation Complete ===");
    println!("Final time: t={}", warehouse.current_time());
    println!("Shelves visited: {}/{}", visited_shelves.len(), num_shelves);
    println!("Average time per shelf: {:.2} steps", 
        warehouse.current_time() as f64 / visited_shelves.len() as f64);
    
    if visited_shelves.len() == num_shelves {
        println!("✓ All shelves visited successfully!");
    } else {
        println!("⚠ Simulation timeout - not all shelves visited");
    }
}

fn check_visits(warehouse: &Warehouse, visited: &mut HashSet<usize>) {
    for robot in &warehouse.robots {
        for shelf in &warehouse.shelves {
            if robot.pos == shelf.pos {
                visited.insert(shelf.id);
            }
        }
    }
}

fn choose_action(robot_pos: Position, target: Position) -> Option<Action> {
    let dx = target.x - robot_pos.x;
    let dy = target.y - robot_pos.y;

    if dx == 0 && dy == 0 {
        return None; // Already at target
    }

    // Prioritize axis with larger distance
    if dx.abs() > dy.abs() {
        if dx > 0 {
            Some(Action::MoveRight)
        } else {
            Some(Action::MoveLeft)
        }
    } else {
        if dy > 0 {
            Some(Action::MoveDown)
        } else {
            Some(Action::MoveUp)
        }
    }
}
