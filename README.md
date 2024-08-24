# Traffic Simulator

## Overview

**Traffic Simulator** is a Rust-based, multi-threaded framework designed to simulate traffic flow management using customizable queuing policies. The system is built to test and manage data packet flows in a controlled environment with adjustable delay parameters and policy-driven queue management. The simulator is capable of handling traffic generation, administration, and consumption, while providing the flexibility to implement various queue management strategies such as Threshold and Random Early Detection (RED).

## Features

- **Multi-threaded Traffic Simulation**: Leverage the power of Rust’s concurrency model to simulate real-time traffic flow using multiple threads.
- **Customizable Delay Parameters**: Adjust producer and consumer delays to simulate different traffic conditions.
- **Policy-Driven Queue Management**: Implement and test queue management policies including:
  - **Threshold**: Discards packets when the queue size exceeds a specified threshold.
  - **Random Early Detection (RED)**: Dynamically manages queue congestion using probabilistic packet dropping based on the queue size and defined thresholds.
- **Real-time Monitoring**: Print detailed logs of packet generation, queuing decisions, and consumption in real-time.

## Getting Started

### Prerequisites

To compile and run the Traffic Simulator, ensure you have the following installed:

- Rust (stable)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/traffic-simulator.git
   cd traffic-simulator
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

### Usage

1. Create an instance of the `Traffic` simulator with specified delay parameters:
   ```rust
   let simulator = Traffic::new(1000, 500); // 1000ms producer delay, 500ms consumer delay
   ```

2. Define your queue management policy:
   - **Threshold Policy**:
     ```rust
     let threshold_policy = simulator.define_threshold_policy(512); // Max queue size of 512
     let policy = Arc::new(RwLock::new(threshold_policy));
     ```
   - **RED Policy**:
     ```rust
     let red_policy = simulator.define_red_policy(0.0..1.0, 0.02, 0.1); // RED with custom parameters
     let policy = Arc::new(RwLock::new(red_policy));
     ```

3. Start the simulation:
   ```rust
   simulator.simulate(policy);
   ```

### Example

Here's a simple example to demonstrate how to use the Traffic Simulator with a Threshold policy:

```rust
use std::sync::{Arc, RwLock};
use traffic_simulator::Traffic;

fn main() {
    // Initialize the traffic simulator
    let simulator = Traffic::new(1000, 500);

    // Define a threshold policy
    let threshold_policy = simulator.define_threshold_policy(512);
    let policy = Arc::new(RwLock::new(threshold_policy));

    // Run the simulation
    simulator.simulate(policy);
}
```

## Custom Policies

Traffic Simulator is extensible, allowing you to create and implement your own queue management policies by implementing the `Policy` trait. Here's an example outline:

```rust
pub struct MyCustomPolicy;

impl Policy for MyCustomPolicy {
    fn allow(&mut self) -> bool {
        // Your custom logic here
    }
}

// Use it in the simulation
let custom_policy = Arc::new(RwLock::new(MyCustomPolicy));
simulator.simulate(custom_policy);
```

## Logging

Throughout the simulation, detailed logs are provided, indicating the flow of packets through the system:

- `[PRODUCER]`: Logs the packet being generated and sent to the queue.
- `[POLICY]`: Logs the decision to allow or discard a packet based on the active policy.
- `[CONSUMER]`: Logs the packet being consumed from the queue.

These logs help in understanding the behavior of the system under different policies and traffic conditions.

## Contribution

Contributions are welcome! Please fork the repository, make your changes, and submit a pull request. For major changes, open an issue first to discuss what you would like to change.
