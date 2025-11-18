# Feature Documentation

This document provides detailed information about the features available in the AI Drive Simulation.

## Core Features

### 1. Neural Network Training
The simulation uses a genetic algorithm to train neural networks to drive cars:

**How it works:**
- Each car starts with a random neural network
- Cars receive input from 15 ray-cast sensors detecting obstacles
- The neural network outputs steering (left/right) and acceleration commands
- Fitness is calculated based on distance traveled
- After all cars crash, the best performers are selected
- Selected networks are cloned and mutated to create the next generation

**Configuration:**
- `NUM_AI_CARS`: Number of cars per generation (default: 100)
- `NUM_HIDDEN_NODES`: Size of hidden layer (default: 15)
- `NUM_OUPUT_NODES`: Output neurons (default: 3)

### 2. Save/Load System

#### Saving Neural Networks
**When to save:**
- After achieving a high score
- Before closing the application
- After multiple successful generations

**How to save:**
- Press `S` key
- Click "💾 Save Best Brain" button in GUI
- Saves to `saves/best_brain_YYYYMMDD_HHMMSS.json`

**What gets saved:**
- Complete neural network weights and biases
- Network architecture (layer sizes)
- Timestamp in filename for easy identification

#### Loading Neural Networks
**When to load:**
- Continue training from a previous session
- Test a previously trained brain
- Compare different training runs

**How to load:**
- Press `L` key
- Click "📁 Load Brain" button in GUI
- Automatically loads the most recent save from `saves/` directory

**After loading:**
- All existing cars are replaced
- New generation spawned with loaded brain (with mutations)
- Training continues from loaded state

### 3. Statistics Export

#### What Gets Exported
```json
{
  "num_cars_alive": 45,
  "fitness": [12.5, 45.3, 78.9, 120.5, ...],
  "generation_count": 42,
  "max_current_score": 156.3
}
```

**Fields:**
- `num_cars_alive`: Current number of active cars
- `fitness`: Array of max fitness per generation
- `generation_count`: Total generations completed
- `max_current_score`: Best score in current generation

#### Using Statistics
**Export:**
- Press `E` key
- Click "📊 Export Statistics" button
- Saves to `saves/stats_YYYYMMDD_HHMMSS.json`

**Analysis:**
- Import JSON into data analysis tools
- Plot fitness curves over generations
- Compare different training runs
- Identify plateaus and improvements

### 4. Real-Time Visualization

#### Neural Network Visualization
The left panel shows the neural network's current state:

**Components:**
- **Input Layer** (left): Ray-cast sensor readings
  - Green: Far from obstacles
  - Red: Close to obstacles
- **Hidden Layer** (middle): Intermediate processing
  - Color intensity shows activation level
- **Output Layer** (right): Steering decisions
  - Shows which direction the car is turning

**Connections:**
- Green lines: Positive correlations
- Red lines: Negative correlations
- Line thickness: Weight strength

#### Statistics Display
Right panel shows:
- **Generation Count**: Current generation number
- **Score**: Best score in current generation
- **Cars**: Number of cars still alive / total

#### Distance Progress
- Vertical progress bar on left
- Car icon shows best car's current position
- Flags mark start and finish points

### 5. Interactive Controls

#### Settings Panel
**Ray Casts:**
- Toggle sensor visualization on/off
- "Hide ray casts at start" reduces clutter

**Camera Follow:**
- Automatically follows best performing car
- Toggle to disable for overview

#### Control Panel
**Start Next Generation:**
- Force current generation to end
- Useful for skipping poor performers

**Restart Simulation:**
- Reset to generation 0
- Clear all statistics
- Start with fresh random brains

## Advanced Features

### Ray-Cast Sensors
Each car has 15 forward-facing sensors:
- Spread over 130-degree arc
- Detect walls, boundaries, and other vehicles
- Max range: 200 pixels
- Normalized values (0.0 = collision, 1.0 = clear)

**Configuration:**
```rust
NUM_RAY_CASTS = 15
RAYCAST_SPREAD_ANGLE_DEG = 130.0
RAYCAST_START_ANGLE_DEG = 20.0
RAYCAST_MAX_TOI = 200.0
```

### Fitness Function
Fitness is calculated as:
```rust
fn calc_fitness(transform: &Transform) -> f32 {
    let y = transform.translation.y;
    if y <= 600.0 {
        return 0.1;  // Penalty for not moving
    }
    return transform.translation.y / 340.0;
}
```

**Key points:**
- Primary factor: Distance traveled (Y position)
- Minimum fitness: 0.1 (encourages movement)
- Linear scaling with distance

### Genetic Algorithm Parameters
**Mutation:**
- Rate: 5.0% (probability a weight mutates)
- Variation: ±0.5 (amount of change)

**Selection:**
- Weighted selection based on fitness
- Higher fitness = more likely to be selected
- Selected brains are cloned for next generation

### Enemy Types
Three types of obstacles:
1. **Simple**: Static vehicles moving up the road
2. **Horizontal**: Vehicles moving side-to-side
3. **Truck**: Larger obstacles with bigger collision boxes

**Configuration:**
- `NUM_ENEMY_CARS`: Total number of enemies (default: 140)
- Randomly distributed along the track

## Performance Features

### Optimization Techniques
1. **Parallel Rendering**: Bevy's ECS handles multiple cars efficiently
2. **Collision Groups**: Cars and obstacles use optimized collision detection
3. **Dynamic Linking**: Faster compilation during development
4. **Release Mode**: Significant performance boost with `--release`

### Performance Tips
- **Reduce car count**: Lower `NUM_AI_CARS` for faster simulation
- **Disable ray visualization**: Reduces rendering overhead
- **Run in release mode**: 10-100x performance improvement
- **Limit frame rate**: Default settings are optimized

## Future Enhancement Ideas

### Potential Features
1. **Multiple Tracks**: Different difficulty levels
2. **Training Presets**: Quick-start configurations
3. **Replay System**: Record and playback runs
4. **Comparison Mode**: Side-by-side brain comparison
5. **Advanced Statistics**: More detailed analytics
6. **Custom Fitness Functions**: User-defined success metrics
7. **Network Architecture Editor**: GUI for designing networks
8. **Cloud Saves**: Share and download trained brains

### Experimental Features
Consider experimenting with:
- **Different activation functions**: ReLU, tanh, etc.
- **NEAT algorithm**: Evolving topology
- **Reinforcement learning**: Q-learning integration
- **Continuous learning**: Never-ending improvement
- **Multi-objective optimization**: Speed + safety

## API for Advanced Users

See `src/nn.rs` for neural network API:
```rust
// Create new network
let net = Net::new(vec![15, 15, 3]);

// Save/Load
net.save("mymodel.json")?;
let loaded = Net::load("mymodel.json")?;

// Predict
let outputs = net.predict(&inputs);

// Mutate
net.mutate();
```

For more details, see the code documentation in each module.
