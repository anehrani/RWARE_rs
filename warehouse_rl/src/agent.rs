use crate::model::{DqnModel, DqnModelConfig};
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;
use rand::Rng;
use std::collections::VecDeque;

pub struct ReplayBuffer {
    buffer: VecDeque<(Vec<f32>, usize, f32, Vec<f32>, bool)>,
    capacity: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, state: Vec<f32>, action: usize, reward: f32, next_state: Vec<f32>, done: bool) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back((state, action, reward, next_state, done));
    }

    pub fn sample(&self, batch_size: usize) -> Vec<(Vec<f32>, usize, f32, Vec<f32>, bool)> {
        let mut rng = rand::thread_rng();
        let mut sample = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            let idx = rng.gen_range(0..self.buffer.len());
            sample.push(self.buffer[idx].clone());
        }
        sample
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

pub struct Agent<B: Backend> {
    pub model: DqnModel<B>,
    pub target_model: DqnModel<B>,
    pub device: B::Device,
    pub epsilon: f32,
    pub epsilon_min: f32,
    pub epsilon_decay: f32,
    pub gamma: f32,
}

impl<B: Backend> Agent<B> {
    pub fn new(config: DqnModelConfig, device: B::Device) -> Self {
        let model = config.init(&device);
        let target_model = config.init(&device);
        Self {
            model,
            target_model,
            device,
            epsilon: 1.0,
            epsilon_min: 0.05,
            epsilon_decay: 0.995,
            gamma: 0.99,
        }
    }

    pub fn act(&self, state: &Vec<f32>) -> usize {
        let mut rng = rand::thread_rng();
        if rng.r#gen::<f32>() < self.epsilon {
            return rng.gen_range(0..4);
        }

        let state_tensor = Tensor::<B, 1>::from_data(
            burn::tensor::TensorData::from(&state[..]),
            &self.device,
        ).reshape([1, 4]);
        let q_values = self.model.forward(state_tensor);
        let action = q_values.argmax(1).into_data().to_vec::<i32>().unwrap()[0] as usize;
        action
    }

    pub fn decay_epsilon(&mut self) {
        if self.epsilon > self.epsilon_min {
            self.epsilon *= self.epsilon_decay;
        }
    }

    pub fn sync_target(&mut self) {
        self.target_model = self.model.clone();
    }
}
