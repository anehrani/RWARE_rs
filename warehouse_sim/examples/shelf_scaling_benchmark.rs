use warehouse_sim::{Warehouse, Robot, Shelf, Position, Action, Direction, step, step_with_clock};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let width = 20;
    let height = 20;
    let num_robots = 5;
    let shelf_counts = [10, 20, 50, 100, 200];
    let num_trials = 20;

    println!("Benchmark: 5 Robots on a {}x{} grid.", width, height);
    println!("Averaging over {} trials for varying shelf counts.\n", num_trials);

    for &num_shelves in &shelf_counts {
        let mut total_steps = 0;
        let mut successes = 0;
        
        for _ in 0..num_trials {
            if let Some(steps) = run_trial(width, height, num_shelves, num_robots) {
                total_steps += steps;
                successes += 1;
            }
        }

        if successes > 0 {
            let avg_steps = total_steps as f64 / successes as f64;
            println!("Shelves: {:3} | Average Steps: {:.2} (Success rate: {}/{})", num_shelves, avg_steps, successes, num_trials);
        } else {
            println!("Shelves: {:3} | Failed to complete any trial within limit.", num_shelves);
        }
    }
}

fn run_trial(width: usize, height: usize, num_shelves: usize, num_robots: usize) -> Option<usize> {
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
                direction: Direction::North,
                carrying: None,
            });
        }
    }

    let mut visited_shelves = HashSet::new();
    let mut steps = 0;
    let max_steps = 5000; // Increased limit for higher shelf counts

    // Initial check
    check_visits(&warehouse, &mut visited_shelves);

    while visited_shelves.len() < num_shelves {
        if steps >= max_steps {
            return None; // Timeout
        }
        steps += 1;

        // Move each robot
        for i in 0..warehouse.robots.len() {
            let robot_pos = warehouse.robots[i].pos;

            // Find nearest unvisited shelf
            let target = warehouse.shelves.iter()
                .filter(|s| !visited_shelves.contains(&s.id))
                .min_by_key(|s| s.pos.manhattan(&robot_pos));

            if let Some(target_shelf) = target {
                let target_pos = target_shelf.pos;
                
                // Try to move towards target
                attempt_move(&mut warehouse, i, target_pos);
            }
        }
        
        check_visits(&warehouse, &mut visited_shelves);
    }

    Some(steps)
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

fn attempt_move(warehouse: &mut Warehouse, robot_idx: usize, target: Position) -> bool {
    let robot_id = warehouse.robots[robot_idx].id;
    let robot_pos = warehouse.robots[robot_idx].pos;
    let current_dist = robot_pos.manhattan(&target);

    if current_dist == 0 {
        return false; // Already there
    }

    let dx = target.x - robot_pos.x;
    let dy = target.y - robot_pos.y;

    // Prioritize axis with larger distance
    let primary_moves = if dx.abs() > dy.abs() {
        let x_action = if dx > 0 { Action::MoveRight } else { Action::MoveLeft };
        let y_action = if dy > 0 { Action::MoveDown } else { Action::MoveUp };
        vec![x_action, y_action]
    } else {
        let y_action = if dy > 0 { Action::MoveDown } else { Action::MoveUp };
        let x_action = if dx > 0 { Action::MoveRight } else { Action::MoveLeft };
        vec![y_action, x_action]
    };

    // Try primary moves first
    for action in primary_moves {
        if step(warehouse, robot_id, action).is_ok() {
            return true;
        }
    }
    
    false
}
