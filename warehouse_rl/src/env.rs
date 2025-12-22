use warehouse_sim::{Warehouse, Robot, Goal, Position, Action, step};
use rand::Rng;

pub struct WarehouseEnv {
    warehouse: Warehouse,
    robot_id: usize,
    target_pos: Position,
    max_steps: usize,
    current_step: usize,
}

impl WarehouseEnv {
    pub fn new(width: usize, height: usize, max_steps: usize) -> Self {
        let mut warehouse = Warehouse::new(width, height);
        let robot_id = 0;
        let target_pos = Position::new((width - 1) as i32, (height - 1) as i32);
        
        // Initial setup
        warehouse.add_robot(Robot { id: robot_id, pos: Position::new(0, 0), carrying: None });
        warehouse.add_goal(Goal { id: robot_id, pos: target_pos });

        Self {
            warehouse,
            robot_id,
            target_pos,
            max_steps,
            current_step: 0,
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
        vec![
            robot.pos.x as f32 / self.warehouse.width as f32,
            robot.pos.y as f32 / self.warehouse.height as f32,
            self.target_pos.x as f32 / self.warehouse.width as f32,
            self.target_pos.y as f32 / self.warehouse.height as f32,
        ]
    }

    pub fn step(&mut self, action_idx: usize) -> (Vec<f32>, f32, bool) {
        let action = match action_idx {
            0 => Action::MoveUp,
            1 => Action::MoveDown,
            2 => Action::MoveLeft,
            3 => Action::MoveRight,
            _ => Action::MoveUp,
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
