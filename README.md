# RWARE_rs Workspace

A comprehensive Rust-based robotic warehouse simulation project. This workspace contains three specialized modules for simulation, bot logic, and reinforcement learning.

## Project Structure

The project is organized into three main crates:

| Module | Description | Core Technology |
| :--- | :--- | :--- |
| [**warehouse_sim**](./warehouse_sim) | The core simulation engine. | Vanilla Rust |
| [**warehouse_bots**](./warehouse_bots) | Rule-based and heuristic bot agents. | Trait-based Strategy |
| [**warehouse_rl**](./warehouse_rl) | Deep Reinforcement Learning agents. | Burn Framework (DQN) |

---

## 1. Core Simulation (`warehouse_sim`)

This is the foundation of the workspace. It provides a 2D discrete grid-based environment where robots, shelves, and goals exist.

### Key Features:
- Manhattan distance-based movement and collision detection.
- Multi-robot synchronization via a global clock.
- ASCII map initialization for easy layout design.

### How to use:
```bash
cd warehouse_sim
cargo run --example simple_movement
```

---

## 2. Bot Agents (`warehouse_bots`)

This module defines the `Agent` trait and provides several non-learning implementations for controlling robots.

### Available Agents:
- **RandomAgent**: Moves randomly, useful for baseline testing.
- **GreedyAgent**: Heuristically moves to the nearest shelf, picks it up, and delivers it to the matching goal.

### How to use:
These agents are designed to be used as dependencies by other crates or examples that require autonomous behavior.

---

## 3. Reinforcement Learning (`warehouse_rl`)

An advanced module using the **Burn** deep learning framework to train robots through experience.

### Key Features:
- **DQN Implementation**: Uses Deep Q-Networks to learn navigation policies.
- **Custom Reward System**: Penalizes collisions and time, rewards goal reaching and progress.
- **Experience Replay**: Improves training stability by learning from past transitions.

### How to use:
```bash
cd warehouse_rl
cargo run --example train_rl
```
*Note: This will output real-time training progress, including accumulated rewards and steps per episode.*

---

## Getting Started

To build the entire workspace:

```bash
cargo build
```

To run all tests across all modules:

```bash
cargo test
```

## Contributing

Each module has its own `README.md` with more specific technical details. Please refer to them for in-depth development information.
