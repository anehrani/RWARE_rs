use warehouse_sim::{Warehouse, Robot, Goal, Position, Action, Direction, step};
use rand::Rng;

pub struct WarehouseEnv {
    warehouse: Warehouse,
    robot_id: usize,
    target_pos: Position,
    max_steps: usize,
    current_step: usize,
    obs_radius: usize,
}

impl WarehouseEnv {
    pub fn new(width: usize, height: usize, max_steps: usize) -> Self {
        let mut warehouse = Warehouse::new(width, height);
        let robot_id = 0;
        let target_pos = Position::new((width - 1) as i32, (height - 1) as i32);
        
        // Initial setup
        warehouse.add_robot(Robot { 
            id: robot_id, 
            pos: Position::new(0, 0), 
            direction: Direction::South,
            carrying: None 
        });
        warehouse.add_goal(Goal { id: 100, pos: target_pos }); // Use unique ID for goal

        Self {
            warehouse,
            robot_id,
            target_pos,
            max_steps,
            current_step: 0,
            obs_radius: 1, // 1 radius = 3x3 grid
        }
    }

    pub fn reset(&mut self) -> Vec<f32> {
        self.current_step = 0;
        let width = self.warehouse.width;
        let height = self.warehouse.height;
        
        // Randomize robot position for better training
        let mut rng = rand::thread_rng();
        let rx = rng.gen_range(0..width) as i32;
        let ry = rng.gen_range(0..height) as i32;
        
        self.warehouse.robots[0].pos = Position::new(rx, ry);
        self.warehouse.reset_clock();
        
        self.get_observation()
    }

    pub fn get_observation(&self) -> Vec<f32> {
        let robot = &self.warehouse.robots[0];
        let mut obs = Vec::new();

        // 1. Robot's own state
        obs.push(if robot.carrying.is_some() { 1.0 } else { 0.0 });
        match robot.direction {
            Direction::North => { obs.extend_from_slice(&[1.0, 0.0, 0.0, 0.0]); }
            Direction::South => { obs.extend_from_slice(&[0.0, 1.0, 0.0, 0.0]); }
            Direction::East => { obs.extend_from_slice(&[0.0, 0.0, 1.0, 0.0]); }
            Direction::West => { obs.extend_from_slice(&[0.0, 0.0, 0.0, 1.0]); }
        }

        // 2. Local grid (3x3 if radius=1)
        let r = self.obs_radius as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                let check_pos = Position::new(robot.pos.x + dx, robot.pos.y + dy);
                
                // Features for each cell:
                // [is_wall, has_other_robot, has_shelf, is_goal]
                
                if check_pos.x < 0 || check_pos.x >= self.warehouse.width as i32 || 
                   check_pos.y < 0 || check_pos.y >= self.warehouse.height as i32 {
                    obs.extend_from_slice(&[1.0, 0.0, 0.0, 0.0]); // Wall
                    continue;
                }
                
                let mut cell_feat = [0.0, 0.0, 0.0, 0.0];
                
                // Other robots
                if self.warehouse.robots.iter().any(|other| other.id != robot.id && other.pos == check_pos) {
                    cell_feat[1] = 1.0;
                }
                
                // Shelves
                if self.warehouse.shelves.iter().any(|s| s.pos == check_pos) {
                    cell_feat[2] = 1.0;
                }
                
                // Goals
                if self.warehouse.goals.iter().any(|g| g.pos == check_pos) {
                    cell_feat[3] = 1.0;
                }
                
                obs.extend_from_slice(&cell_feat);
            }
        }

        obs
    }

    pub fn step(&mut self, action_idx: usize) -> (Vec<f32>, f32, bool) {
        let action = match action_idx {
            0 => Action::TurnLeft,
            1 => Action::TurnRight,
            2 => Action::Forward,
            3 => Action::ToggleLoad,
            _ => Action::Forward,
        };

        let prev_dist = self.warehouse.robots[0].pos.manhattan(&self.target_pos);
        let result = step(&mut self.warehouse, self.robot_id, action);
        self.current_step += 1;

        let mut reward = -0.1; // Small penalty for each step
        let mut done = false;

        match result {
            Ok(_) => {
                let new_dist = self.warehouse.robots[0].pos.manhattan(&self.target_pos);
                if new_dist < prev_dist {
                    reward += 0.5; // Reward for moving closer
                } else if new_dist > prev_dist {
                    reward -= 0.5; // Penalty for moving away
                }
                
                if new_dist == 0 {
                    reward += 10.0;
                    done = true;
                }
            }
            Err(_) => {
                reward -= 1.0; // Penalty for illegal move (wall/collision)
            }
        }

        if self.current_step >= self.max_steps {
            done = true;
        }

        (self.get_observation(), reward, done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse_sim::Shelf;

    #[test]
    fn test_observation_size() {
        let env = WarehouseEnv::new(10, 10, 100);
        let obs = env.get_observation();
        // 1 (carrying) + 4 (direction one-hot) + 9 (cells) * 4 (features per cell) = 1 + 4 + 36 = 41
        assert_eq!(obs.len(), 41);
    }

    #[test]
    fn test_observation_encoding_walls() {
        // Create env where robot is at (0,0)
        let mut env = WarehouseEnv::new(10, 10, 100);
        env.warehouse.robots[0].pos = Position::new(0, 0);
        env.warehouse.robots[0].direction = Direction::North;
        
        let obs = env.get_observation();
        
        // At (0,0), radius 1 (3x3 grid)
        // Neighbors: (-1,-1), (0,-1), (1,-1), (-1,0), (0,0), (1,0), (-1,1), (0,1), (1,1)
        // Walls are at: (-1,-1), (0,-1), (1,-1), (-1,0), (-1,1) -> 5 cells
        
        // Count cells marked as walls (first feature in cell is 1.0)
        let mut wall_count = 0;
        for i in 0..9 {
            let offset = 5 + i * 4;
            if obs[offset] == 1.0 {
                wall_count += 1;
            }
        }
        assert_eq!(wall_count, 5);
    }

    #[test]
    fn test_env_reward_movement() {
        let mut env = WarehouseEnv::new(10, 10, 100);
        env.warehouse.robots[0].pos = Position::new(5, 5);
        env.warehouse.robots[0].direction = Direction::East;
        env.target_pos = Position::new(7, 5);
        
        // Move Forward (to 6,5) -> gets closer
        let (_, reward, _) = env.step(2); // 2 = Forward
        assert!(reward > 0.0); // -0.1 (step) + 0.5 (closer) = 0.4
    }

    #[test]
    fn test_env_reward_illegal() {
        let mut env = WarehouseEnv::new(10, 10, 100);
        env.warehouse.robots[0].pos = Position::new(0, 0);
        env.warehouse.robots[0].direction = Direction::North;
        
        // Move Forward into wall
        let (_, reward, _) = env.step(2);
        assert!(reward < -1.0); // -0.1 (step) - 1.0 (illegal) = -1.1
    }
}
