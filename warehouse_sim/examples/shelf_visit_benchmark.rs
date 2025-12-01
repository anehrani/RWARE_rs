use warehouse_sim::{Warehouse, Robot, Shelf, Position, Action, step};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let width = 20;
    let height = 20;
    let num_shelves = 20;
    let robot_counts = [1, 2, 4, 8, 16];
    let num_trials = 20;

    println!("Benchmark: Visiting {} shelves on a {}x{} grid.", num_shelves, width, height);
    println!("Averaging over {} trials for each robot count.\n", num_trials);

    for &num_robots in &robot_counts {
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
            println!("Robots: {:2} | Average Steps: {:.2} (Success rate: {}/{})", num_robots, avg_steps, successes, num_trials);
        } else {
            println!("Robots: {:2} | Failed to complete any trial within limit.", num_robots);
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
                carrying: None,
            });
        }
    }

    let mut visited_shelves = HashSet::new();
    let mut steps = 0;
    let max_steps = 2000; 

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
        // Calculate next pos to verify it reduces distance (or at least doesn't increase, but we want strict for convergence)
        // Actually, the primary moves constructed above ALWAYS reduce Manhattan distance if successful.
        // e.g. if dx > 0, MoveRight reduces x distance by 1.
        
        if step(warehouse, robot_id, action).is_ok() {
            return true;
        }
    }
    
    // If blocked, we could try other moves, but for now let's stick to simple greedy.
    // Adding random moves or waiting might help with deadlocks, but let's see if this suffices.
    false
}
