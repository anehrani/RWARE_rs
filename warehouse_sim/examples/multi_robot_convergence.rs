use warehouse_sim::{Warehouse, Robot, Position, Action, Direction, step, step_with_clock};
use rand::Rng;
use std::{thread, time};

fn main() {
    let width = 10;
    let height = 10;
    let mut warehouse = Warehouse::new(width, height);
    let target = Position::new(5, 5);

    // Add 3 robots at random positions
    let mut rng = rand::thread_rng();
    let mut positions = Vec::new();

    while positions.len() < 3 {
        let x = rng.gen_range(0..width) as i32;
        let y = rng.gen_range(0..height) as i32;
        let pos = Position::new(x, y);
        
        // Ensure unique positions and not on target
        if !positions.contains(&pos) && pos != target {
            positions.push(pos);
            warehouse.add_robot(Robot {
                id: positions.len(), // Use 0-based index as ID
                pos,
                direction: Direction::North,
                carrying: None,
            });
        }
    }

    println!("Simulation started with 3 robots moving to {:?}", target);
    for robot in &warehouse.robots {
        println!("Robot {} starting at {:?}", robot.id, robot.pos);
    }

    // Simulation loop
    // Simulation loop
    for t in 0..50 {
        println!("\nStep {}", t);
        let mut all_at_target = true;
        let mut any_moved = false;
        
        // Iterate through robots by index
        for i in 0..warehouse.robots.len() {
            let robot_id = warehouse.robots[i].id;
            let robot_pos = warehouse.robots[i].pos;
            
            if robot_pos == target {
                println!("Robot {} is at target.", robot_id);
                continue;
            }
            
            all_at_target = false;

            let current_dist = robot_pos.manhattan(&target);
            let mut moved = false;

            // Define all possible moves and their resulting positions
            let candidates = [
                (Action::MoveUp, Position::new(robot_pos.x, robot_pos.y - 1)),
                (Action::MoveDown, Position::new(robot_pos.x, robot_pos.y + 1)),
                (Action::MoveLeft, Position::new(robot_pos.x - 1, robot_pos.y)),
                (Action::MoveRight, Position::new(robot_pos.x + 1, robot_pos.y)),
            ];

            // Try moves that strictly reduce the distance to the target
            for (action, next_pos) in candidates {
                if next_pos.manhattan(&target) < current_dist {
                    // Try to perform the action
                    if step(&mut warehouse, robot_id, action).is_ok() {
                        println!("Robot {} moved {:?} to {:?}", robot_id, action, warehouse.robots[i].pos);
                        moved = true;
                        any_moved = true;
                        break;
                    }
                }
            }

            if !moved {
                println!("Robot {} waiting at {:?} (blocked or no better move)", robot_id, robot_pos);
            }
        }

        print_grid(&warehouse, target);

        if all_at_target {
            println!("All robots reached the target!");
            break;
        }

        if !any_moved {
            println!("No robots moved this step. Simulation converged (or deadlocked).");
            break;
        }
        
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn print_grid(warehouse: &Warehouse, target: Position) {
    println!("  {}", "-".repeat(warehouse.width));
    for y in 0..warehouse.height {
        print!(" |");
        for x in 0..warehouse.width {
            let pos = Position::new(x as i32, y as i32);
            let mut char = '.';
            
            if pos == target {
                char = 'T';
            }
            
            // Check for robots
            for r in &warehouse.robots {
                if r.pos == pos {
                    // If multiple robots (collision bug?), just show the last one found
                    char = (b'0' + r.id as u8) as char;
                }
            }
            print!("{}", char);
        }
        println!("|");
    }
    println!("  {}", "-".repeat(warehouse.width));
}
