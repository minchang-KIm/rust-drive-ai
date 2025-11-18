# AI learns to drive

AI learns to drive in a road-fighter inspired environment. The cars are controlled using neural networks and are trained using genetic algorithms.

Built with [Rust](https://www.rust-lang.org/) and [Bevy](https://bevyengine.org/) game engine

![gui](/gui.png)

# Demo
Here's the entire timelapse of the AI learning to drive

[![youtube](https://img.youtube.com/vi/H7RWcNgE-6s/0.jpg)](https://youtu.be/H7RWcNgE-6s)

# Devlog
Here's a devlog of how this was built

[![youtube](https://img.youtube.com/vi/Pq1sKS-q1sA/0.jpg)](https://youtu.be/Pq1sKS-q1sA)

## Features

✨ **Production-Ready Features:**
- 🧠 **Neural Network Save/Load**: Save and load trained neural networks to continue training later
- 📊 **Statistics Export**: Export training statistics to JSON for analysis
- ⌨️ **Keyboard Shortcuts**: Quick access to common operations (S, L, E)
- 🎮 **Interactive GUI**: Real-time visualization of neural network decisions
- 📈 **Live Statistics**: Track generation count, score, and active cars
- 🔧 **Configurable**: Easily adjust simulation parameters
- ⚡ **Performance Optimized**: Efficient genetic algorithm implementation

### Keyboard Shortcuts
- **S** - Save best brain
- **L** - Load previously saved brain
- **E** - Export statistics to JSON
- **Esc** - Exit application

## Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

## Installation & Usage

### Quick Start
```bash
# Clone the repository
git clone git@github.com:bones-ai/rust-drive-ai.git
cd rust-drive-ai

# Build the project
cargo build --release

# Run the simulation
cargo run --release
```

### Using Saved Brains
The application automatically saves trained brains to the `saves/` directory with timestamps:
- `saves/best_brain_YYYYMMDD_HHMMSS.json` - Saved neural networks
- `saves/stats_YYYYMMDD_HHMMSS.json` - Training statistics

To load a previously saved brain, press `L` or use the "Load Brain" button in the GUI.

## Configuration

### Main Configuration
Edit `src/configs.rs` to adjust:
- Number of AI cars (`NUM_AI_CARS`)
- Neural network architecture (`NUM_HIDDEN_NODES`, `NUM_OUPUT_NODES`)
- Ray-cast sensors (`NUM_RAY_CASTS`, `RAYCAST_SPREAD_ANGLE_DEG`)
- Car physics (`MAX_SPEED`, `CAR_THRUST`, `FRICTION`)
- Window dimensions (`WINDOW_WIDTH`, `WINDOW_HEIGHT`)

### Neural Network Configuration
The default architecture:
- **Input Layer**: 15 ray-cast sensors
- **Hidden Layer**: 15 neurons
- **Output Layer**: 3 neurons (acceleration, left/right steering)

## How It Works

### Genetic Algorithm
1. **Initialization**: 100 cars with random neural networks
2. **Evaluation**: Cars drive until they crash
3. **Selection**: Best performing cars are selected based on distance traveled
4. **Reproduction**: Selected cars' brains are cloned and mutated
5. **Repeat**: Process continues with new generation

### Neural Network
Each car has:
- **15 ray-cast sensors** detecting obstacles
- **Feed-forward neural network** processing sensor data
- **3 output neurons** controlling steering (left/right) and acceleration

## Technical Details

### Architecture
- **Game Engine**: Bevy 0.10.1
- **Physics**: Rapier2D
- **Neural Networks**: Custom implementation with sigmoid activation
- **Serialization**: Serde with JSON format
- **GUI**: egui for interactive controls

### Performance
- Runs 100 cars simultaneously
- Real-time neural network visualization
- Efficient collision detection
- Cross-platform compatible (audio disabled)

## Development

### Project Structure
```
src/
├── main.rs           # Application entry point
├── lib.rs            # Module exports
├── car.rs            # Car entity and control systems
├── configs.rs        # Configuration constants
├── enemy.rs          # Obstacle generation
├── gui.rs            # User interface and visualization
├── nn.rs             # Neural network implementation
├── population.rs     # Genetic algorithm logic
├── resources.rs      # Bevy resources and state
└── road.rs           # Road rendering
```

### Building for Development
```bash
# Run in debug mode with fast compilation
cargo run

# Run with optimizations
cargo run --release

# Check for errors without building
cargo check

# Run tests (if available)
cargo test
```

## Troubleshooting

### Common Issues

**Build fails with ALSA errors**
- This has been fixed! Audio features are disabled by default.

**Performance issues**
- Run in release mode: `cargo run --release`
- Reduce `NUM_AI_CARS` in `src/configs.rs`
- Disable ray visualization in the GUI

**Can't load saved brain**
- Ensure `saves/` directory exists
- Check that brain files have `.json` extension
- Verify file isn't corrupted

## Contributing

Contributions are welcome! Areas for improvement:
- Additional neural network architectures
- Different selection strategies
- More sophisticated fitness functions
- Better visualization options
- Performance optimizations

## Forks
Here's a list of forks that extend this project, let me know if you have an interesting fork to add:
- https://gitlab.com/ThibaultLemaire/rust-drive-ai

## Assets
- Sprites: [https://www.spriters-resource.com/nes/roadfighter/sheet/57232/](https://www.spriters-resource.com/nes/roadfighter/sheet/57232/)
- Font: [https://code807.itch.io/magero](https://code807.itch.io/magero)

## License
See [LICENSE](LICENSE) file for details.

## Acknowledgments
- Bevy game engine community
- Genetic algorithm and neural network research papers
- Road Fighter (NES) for visual inspiration
