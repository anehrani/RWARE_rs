use crate::env::WarehouseEnv;
use crate::agent::{Agent, ReplayBuffer};
use crate::model::DqnModelConfig;
use burn::tensor::backend::AutodiffBackend;
use burn::tensor::Tensor;
use burn::optim::AdamConfig;
use burn::optim::Optimizer;
use burn::optim::GradientsParams;

pub fn train<B: AutodiffBackend>(
    device: B::Device,
    episodes: usize,
    batch_size: usize,
    max_steps: usize,
) {
    let mut env = WarehouseEnv::new(10, 10, max_steps);
    let config = DqnModelConfig {
        input_size: 4,
        hidden_size: 64,
        output_size: 4,
    };
    
    let mut agent = Agent::<B>::new(config, device.clone());
    let mut buffer = ReplayBuffer::new(10000);
    let mut optim = AdamConfig::new().init::<B, _>();

    for episode in 0..episodes {
        let mut state = env.reset();
        let mut total_reward = 0.0;
        let mut steps = 0;
        let mut done = false;

        while !done {
            let action = agent.act(&state);
            let (next_state, reward, is_done) = env.step(action);
            
            buffer.push(state.clone(), action, reward, next_state.clone(), is_done);
            state = next_state;
            total_reward += reward;
            steps += 1;
            done = is_done;

            if buffer.len() > batch_size {
                let batch = buffer.sample(batch_size);
                
                // Convert batch to tensors
                let states: Vec<f32> = batch.iter().flat_map(|s| s.0.clone()).collect();
                let actions: Vec<i32> = batch.iter().map(|s| s.1 as i32).collect();
                let rewards: Vec<f32> = batch.iter().map(|s| s.2).collect();
                let next_states: Vec<f32> = batch.iter().flat_map(|s| s.3.clone()).collect();
                let dones: Vec<f32> = batch.iter().map(|s| if s.4 { 1.0 } else { 0.0 }).collect();

                let state_tensor = Tensor::<B, 1>::from_data(
                    burn::tensor::TensorData::from(&states[..]), 
                    &device
                ).reshape([batch_size, 4]);
                let reward_tensor = Tensor::<B, 1>::from_data(
                    burn::tensor::TensorData::from(&rewards[..]), 
                    &device
                );
                let next_state_tensor = Tensor::<B, 1>::from_data(
                    burn::tensor::TensorData::from(&next_states[..]), 
                    &device
                ).reshape([batch_size, 4]);
                let done_tensor = Tensor::<B, 1>::from_data(
                    burn::tensor::TensorData::from(&dones[..]), 
                    &device
                );

                // DQN Update: Q(s,a) = r + gamma * max(Q(s', a'))
                let q_values = agent.model.forward(state_tensor);
                
                // Simplified loss: we want q_values to approach target (reward + gamma * max_next_q)
                // In a full implementation, we'd select the specific action index.
                let next_q_values = agent.target_model.forward(next_state_tensor);
                let max_next_q = next_q_values.max_dim(1).reshape([batch_size]);
                let targets = reward_tensor + (max_next_q * agent.gamma * (Tensor::ones_like(&done_tensor) - done_tensor));

                // Compute Mean Squared Error Loss
                let loss = (q_values.mean_dim(1).reshape([batch_size]) - targets).powf_scalar(2.0).mean();

                // Backprop (Burn simplified)
                let grads = loss.backward();
                let grads = GradientsParams::from_grads(grads, &agent.model);
                agent.model = optim.step(1e-3, agent.model, grads);
            }
        }

        if episode % 20 == 0 {
            agent.sync_target();
        }

        if episode % 10 == 0 {
            println!("Episode {:3}: Reward = {:7.2}, Steps = {:3}, Epsilon = {:.4}", 
                episode, total_reward, steps, agent.epsilon);
        }
    }
}
