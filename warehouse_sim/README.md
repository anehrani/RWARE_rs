# Warehouse Simulation Library

A minimal Rust library for simulating a robotic warehouse environment, inspired by `robotic-warehouse`. This library handles the movement, collision, and pick/place mechanics of robots in a grid-based warehouse.

## Features

- **Grid-based Simulation**: Discrete 2D grid environment.
- **Entities**: Robots, Shelves, and Goals.
- **Actions**: Move (Up, Down, Left, Right), Pick, Place.
- **Collision Detection**: Prevents robots from moving into walls or occupied cells.
- **ASCII Map Initialization**: Easily define layouts using string maps.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
warehouse_sim = { path = "." } # Or git repository URL
```

## Usage

### Basic Example

```rust
use warehouse_sim::{Warehouse, Robot, Position, Action, step};

fn main() {
    // Initialize a 5x5 warehouse
    let mut warehouse = Warehouse::new(5, 5);
    
    // Add a robot at (0, 0)
    warehouse.add_robot(Robot { 
        id: 0, 
        pos: Position::new(0, 0), 
        carrying: None 
    });

    // Move the robot
    match step(&mut warehouse, 0, Action::MoveRight) {
        Ok(_) => println!("Robot moved to {:?}", warehouse.robots[0].pos),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Using ASCII Maps

You can initialize a warehouse layout using an ASCII string:

- `.`: Empty space
- `R`: Robot
- `S`: Shelf
- `G`: Goal
- `X`: Shelf on a Goal

```rust
use warehouse_sim::Warehouse;

fn main() {
    let map = "
    .R...
    .S.G.
    .....
    ";
    let warehouse = Warehouse::from_ascii(map).unwrap();
    println!("Warehouse created with {} robots", warehouse.robots.len());
}
```

## Running Examples

The library comes with example binaries:

1.  **Simple Movement**:
    ```bash
    cargo run --example simple_movement
    ```

2.  **Pick and Place**:
    ```bash
    cargo run --example pick_and_place
    ```

## Testing

Run the test suite to verify correctness:

```bash
cargo test
```
