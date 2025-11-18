# AI learns to drive
AI learns to drive in a road-fighter inspired environment

The cars are controlled using a neural network, and are trained using a genetic algorithm.

Built with [Rust](https://www.rust-lang.org/) and [Bevy](https://bevyengine.org/) game engine

![gui](/gui.png)

# Demo
Here's the entire timelapse of the AI learning to drive

[![youtube](https://img.youtube.com/vi/H7RWcNgE-6s/0.jpg)](https://youtu.be/H7RWcNgE-6s)

# Devlog
Here's a devlog of how this was built

[![youtube](https://img.youtube.com/vi/Pq1sKS-q1sA/0.jpg)](https://youtu.be/Pq1sKS-q1sA)

## Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

## Usage
- Clone the repo
    ```
    git clone git@github.com:bones-ai/rust-drive-ai.git
    cd rust-drive-ai
    ```
- Build the project
    ```
    cargo build --release
    ```
- Run the simulation
    ```
    cargo run --release
    ```

## Configurations
- The project config file is located at `src/configs.rs`

## Technical Details
- Built with Rust and Bevy 0.10.1 game engine
- Uses genetic algorithms for AI training
- Neural network controlled cars with ray-cast sensors
- Physics simulation powered by Rapier2D
- Audio features disabled for cross-platform compatibility

## Forks
Here's a list of of forks that extend this project, let me know if you have an interesting fork to add:
- https://gitlab.com/ThibaultLemaire/rust-drive-ai


## Assets
- [https://www.spriters-resource.com/nes/roadfighter/sheet/57232/](https://www.spriters-resource.com/nes/roadfighter/sheet/57232/)
- Font - [https://code807.itch.io/magero](https://code807.itch.io/magero)
