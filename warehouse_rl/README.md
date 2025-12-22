# Warehouse RL

A Reinforcement Learning (RL) extension for the `warehouse_sim` library, powered by the [Burn](https://burn.dev/) deep learning framework. This crate provides the infrastructure to train robots to navigate complex warehouse environments using Deep Q-Learning (DQN).

## Overview

`warehouse_rl` implements a standard RL loop where an agent learns to move a robot from a starting position to a target goal (point A to B) while avoiding obstacles and maximizing efficiency.

### Key Components

- **Environment (`WarehouseEnv`)**: A wrapper around `warehouse_sim` that normalizes the warehouse state into tensors and provides a reward signal.
- **DQN Model (`DqnModel`)**: A multi-layer perceptron neural network implemented in Burn that predicts the "value" (Q-value) of each possible action (Up, Down, Left, Right).
- **Agent**: Manages exploration (moving randomly) and exploitation (using the neural network), and handles experience replay.
- **Training Loop**: Orchestrates episodes, captures transitions, and performs backpropagation to update the model weights.

## Reward System

To encourage efficient and safe navigation, the agent receives:
- **Success Reward**: `+10.0` when the robot reaches the target goal.
- **Progress Reward**: `+0.5` for moving closer to the target (Manhattan distance).
- **Penalty**: `-0.5` for moving away from the target.
- **Collision Penalty**: `-1.0` for attempting an illegal move (hitting walls or boundaries).
- **Time Penalty**: `-0.1` for every step taken, encouraging the shortest path.

## Getting Started

### Prerequisites

- Rust (latest stable or nightly)
- Metal, Vulkan, or CUDA (depending on your `burn` backend configuration)

### Running the Training

You can run the included training example to see the agent learn in real-time:

```bash
cargo run --example train_rl
```

### Monitoring Progress

During training, the console output will show the following metrics:
- **Reward**: The total score accumulated in the episode. Higher is better.
- **Steps**: How many moves the robot made. Lower is better.
- **Epsilon**: The current exploration rate. It starts at `1.0` (random movement) and decays towards `0.05`.

## Configuration

The training parameters can be adjusted in `src/train.rs`:
- `episodes`: Total number of training runs.
- `batch_size`: Number of experiences sampled for each learning step.
- `hidden_size`: Number of neurons in the neural network's hidden layers.
- `max_steps`: Maximum number of moves allowed before an episode is terminated.

## Project Structure

- `src/env.rs`: RL environment wrapper.
- `src/model.rs`: Burn neural network architecture.
- `src/agent.rs`: DQN Agent and Experience Replay buffer.
- `src/train.rs`: Core training loop and weight update logic.
- `examples/train_rl.rs`: Entry point for training.
