use warehouse_sim::{Warehouse, Robot, Shelf, Position, Action, Direction, step, step_with_clock};
use rand::Rng;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Default)]
struct RobotMetrics {
    total_moves: usize,
    total_waits: usize,
    total_distance: i32,
    shelves_visited: usize,
}

#[derive(Debug)]
struct SimulationMetrics {
    total_steps: usize,
    robot_metrics: HashMap<usize, RobotMetrics>,
}

impl SimulationMetrics {
    fn new(num_robots: usize) -> Self {
        let mut robot_metrics = HashMap::new();
        for i in 0..num_robots {
            robot_metrics.insert(i, RobotMetrics::default());
        }
        Self {
            total_steps: 0,
            robot_metrics,
        }
    }

    fn print_summary(&self) {
        println!("\n=== Simulation Summary ===");
        println!("Total Steps: {}", self.total_steps);
        
        let mut total_moves = 0;
        let mut total_waits = 0;
        let mut total_distance = 0;
        
        for (robot_id, metrics) in &self.robot_metrics {
            println!("\nRobot {}:", robot_id);
            println!("  Moves: {}", metrics.total_moves);
            println!("  Waits: {}", metrics.total_waits);
            println!("  Distance Traveled: {}", metrics.total_distance);
            println!("  Shelves Visited: {}", metrics.shelves_visited);
            println!("  Wait Ratio: {:.2}%", 
                (metrics.total_waits as f64 / self.total_steps as f64) * 100.0);
            println!("  Utilization: {:.2}%", 
                (metrics.total_moves as f64 / self.total_steps as f64) * 100.0);
            
            total_moves += metrics.total_moves;
            total_waits += metrics.total_waits;
            total_distance += metrics.total_distance;
        }
        
        println!("\n=== Aggregate Statistics ===");
        println!("Total Moves: {}", total_moves);
        println!("Total Waits: {}", total_waits);
        println!("Total Distance: {}", total_distance);
        println!("Average Wait Ratio: {:.2}%", 
            (total_waits as f64 / (self.total_steps * self.robot_metrics.len()) as f64) * 100.0);
        println!("Average Utilization: {:.2}%", 
            (total_moves as f64 / (self.total_steps * self.robot_metrics.len()) as f64) * 100.0);
    }
}

fn main() {
    let width = 20;
    let height = 20;
    let num_shelves = 30;
    let num_robots = 5;

    println!("Running detailed metrics simulation:");
    println!("Grid: {}x{}, Shelves: {}, Robots: {}\n", width, height, num_shelves, num_robots);

    let metrics = run_simulation(width, height, num_shelves, num_robots);
    metrics.print_summary();
}

fn run_simulation(width: usize, height: usize, num_shelves: usize, num_robots: usize) -> SimulationMetrics {
    let mut warehouse = Warehouse::new(width, height);
    let mut rng = rand::thread_rng();
    let mut metrics = SimulationMetrics::new(num_robots);

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
    let max_steps = 2000;

    // Initial check
    check_visits(&warehouse, &mut visited_shelves, &mut metrics);

    while visited_shelves.len() < num_shelves && metrics.total_steps < max_steps {
        metrics.total_steps += 1;

        // Move each robot
        for i in 0..warehouse.robots.len() {
            let robot_id = warehouse.robots[i].id;
            let robot_pos = warehouse.robots[i].pos;

            // Find nearest unvisited shelf
            let target = warehouse.shelves.iter()
                .filter(|s| !visited_shelves.contains(&s.id))
                .min_by_key(|s| s.pos.manhattan(&robot_pos));

            if let Some(target_shelf) = target {
                let target_pos = target_shelf.pos;
                
                // Try to move towards target
                let moved = attempt_move(&mut warehouse, i, target_pos);
                
                if moved {
                    // Track the move
                    let new_pos = warehouse.robots[i].pos;
                    let distance = robot_pos.manhattan(&new_pos);
                    
                    if let Some(robot_metrics) = metrics.robot_metrics.get_mut(&robot_id) {
                        robot_metrics.total_moves += 1;
                        robot_metrics.total_distance += distance;
                    }
                } else {
                    // Robot waited (blocked or at target)
                    if let Some(robot_metrics) = metrics.robot_metrics.get_mut(&robot_id) {
                        robot_metrics.total_waits += 1;
                    }
                }
            } else {
                // No unvisited shelves, robot waits
                if let Some(robot_metrics) = metrics.robot_metrics.get_mut(&robot_id) {
                    robot_metrics.total_waits += 1;
                }
            }
        }
        
        check_visits(&warehouse, &mut visited_shelves, &mut metrics);
    }

    metrics
}

fn check_visits(warehouse: &Warehouse, visited: &mut HashSet<usize>, metrics: &mut SimulationMetrics) {
    for robot in &warehouse.robots {
        for shelf in &warehouse.shelves {
            if robot.pos == shelf.pos && !visited.contains(&shelf.id) {
                visited.insert(shelf.id);
                if let Some(robot_metrics) = metrics.robot_metrics.get_mut(&robot.id) {
                    robot_metrics.shelves_visited += 1;
                }
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
