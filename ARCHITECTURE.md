# SystemVision Architecture

This document provides a high-level overview of the SystemVision architecture, explaining the key components, their interactions, and design decisions.

## Table of Contents

- [Overview](#overview)
- [Architecture Diagram](#architecture-diagram)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Threading and Concurrency Model](#threading-and-concurrency-model)
- [State Management](#state-management)
- [Design Decisions](#design-decisions)
- [Module Responsibilities](#module-responsibilities)
- [Extension Points](#extension-points)

## Overview

SystemVision is a modern Linux system monitoring application built with Rust and egui. The architecture follows a modular, event-driven design with clear separation of concerns:

- **Collectors**: Gather system metrics from various sources
- **Storage**: Persist metrics and handle data export
- **UI**: Display metrics and handle user interactions
- **Alerts**: Monitor thresholds and notify users
- **Models**: Define data structures and application state

### Design Principles

1. **Modularity**: Each component has a single, well-defined responsibility
2. **Async-First**: Non-blocking operations for responsive UI
3. **Type Safety**: Leverage Rust's type system for correctness
4. **Performance**: Efficient metric collection with minimal overhead
5. **Extensibility**: Easy to add new collectors and UI views

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Application Layer                        │
│                           (src/app.rs)                           │
└───────────────┬─────────────────────────────────────┬───────────┘
                │                                     │
                ▼                                     ▼
┌───────────────────────────────┐   ┌─────────────────────────────┐
│       UI Layer (egui)         │   │    Background Services      │
│      (src/ui/*.rs)            │   │                             │
├───────────────────────────────┤   ├─────────────────────────────┤
│ • Dashboard                   │   │ • Metric Collection Loop    │
│ • Process View                │   │ • Alert Manager             │
│ • Network View                │   │ • Storage Writer            │
│ • GPU View                    │   │                             │
│ • Disk View                   │   │                             │
│ • Settings                    │   │                             │
│ • Alerts Panel                │   │                             │
└───────────┬───────────────────┘   └──────────┬──────────────────┘
            │                                  │
            │         ┌────────────────────────┘
            │         │
            ▼         ▼
┌─────────────────────────────────────────────────────────────────┐
│                      State Management                            │
│                    (src/models/state.rs)                         │
│                                                                   │
│  • SystemState (Arc<RwLock<T>>)                                  │
│  • Shared between UI and background tasks                        │
│  • Thread-safe access to current metrics                         │
└───────────────────────────┬─────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
┌─────────────────┐ ┌─────────────┐ ┌─────────────────┐
│   Collectors    │ │   Storage   │ │     Alerts      │
│ (src/collectors)│ │ (src/storage)│ │  (src/alerts)   │
├─────────────────┤ ├─────────────┤ ├─────────────────┤
│ • CPU           │ │ • SQLite DB │ │ • Threshold     │
│ • Memory        │ │ • CSV Export│ │   Monitoring    │
│ • Process       │ │ • JSON      │ │ • Notifications │
│ • Network       │ │   Export    │ │ • Alert History │
│ • Disk          │ │             │ │                 │
│ • GPU           │ │             │ │                 │
│ • Temperature   │ │             │ │                 │
│ • Power         │ │             │ │                 │
└────────┬────────┘ └─────────────┘ └─────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────┐
│                      System APIs / Kernel                        │
│                                                                   │
│  • /proc filesystem                                              │
│  • /sys filesystem                                               │
│  • sysinfo crate                                                 │
│  • NVML (GPU)                                                    │
│  • Network interfaces                                            │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Application Layer (`src/app.rs`)

The main application coordinator that:
- Initializes all subsystems
- Sets up the egui application
- Manages the application lifecycle
- Coordinates between UI and background services

**Key Responsibilities:**
- Application startup and shutdown
- Configuration loading
- Component initialization
- Main event loop setup

### 2. UI Layer (`src/ui/`)

egui-based user interface with multiple views:

#### Dashboard (`dashboard.rs`)
- Overview of all system metrics
- Real-time graphs and gauges
- Quick status indicators

#### Process View (`process_view.rs`)
- Process list with sorting and filtering
- CPU and memory usage per process
- Process management (kill, nice)

#### Network View (`network_view.rs`)
- Network interface statistics
- Bandwidth graphs
- Connection monitoring

#### GPU View (`gpu_view.rs`)
- GPU utilization and temperature
- VRAM usage
- Multi-GPU support

#### Disk View (`disk_view.rs`)
- Disk usage and I/O statistics
- Filesystem information
- Mount point details

#### Settings (`settings.rs`)
- Configuration management
- Theme selection
- Update intervals
- Alert thresholds

#### Theme (`theme.rs`)
- Dark/light mode support
- Custom color schemes
- Consistent styling

### 3. Collectors (`src/collectors/`)

Modular metric collection system:

Each collector implements a common trait:
```rust
pub trait MetricCollector {
    type Metric;
    
    fn collect(&self) -> Result<Self::Metric>;
    fn name(&self) -> &str;
}
```

**Collectors:**
- **CPU** (`cpu.rs`): Per-core usage, load average, frequency
- **Memory** (`memory.rs`): RAM, swap, cache usage
- **Process** (`process.rs`): Process enumeration and stats
- **Network** (`network.rs`): Interface stats, bandwidth
- **Disk** (`disk.rs`): Disk usage, I/O operations
- **GPU** (`gpu.rs`): GPU metrics via NVML/sysfs
- **Temperature** (`temperature.rs`): System temperatures
- **Power** (`power.rs`): Battery and power consumption

### 4. Storage Layer (`src/storage/`)

Persistent storage and data export:

#### Database (`database.rs`)
- SQLite for metric history
- Efficient time-series storage
- Automatic cleanup of old data
- Indexed queries for fast retrieval

#### Export (`export.rs`)
- CSV export for spreadsheet analysis
- JSON export for programmatic access
- Configurable time ranges
- Batch export operations

### 5. Alert System (`src/alerts/`)

Threshold monitoring and notifications:

#### Alert Manager (`manager.rs`)
- Monitors metrics against thresholds
- Generates notifications
- Alert history tracking
- Configurable alert rules

**Alert Types:**
- CPU usage threshold
- Memory usage threshold
- Disk space threshold
- Temperature threshold
- Custom metric alerts

### 6. Models (`src/models/`)

Data structures and types:

#### Config (`config.rs`)
- Application configuration
- User preferences
- Alert thresholds
- TOML serialization

#### Metrics (`metrics.rs`)
- Metric data structures
- Type-safe metric definitions
- Serialization support

#### State (`state.rs`)
- Application state management
- Thread-safe shared state
- State synchronization

## Data Flow

### Metric Collection Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. Collection Timer Triggers (every N seconds)                  │
└────────────────────────┬────────────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. Spawn Async Tasks for Each Collector                         │
│    • CPU Collector                                               │
│    • Memory Collector                                            │
│    • Process Collector                                           │
│    • Network Collector                                           │
│    • ... (parallel collection)                                   │
└────────────────────────┬────────────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. Collectors Read System Data                                  │
│    • /proc/stat, /proc/meminfo                                   │
│    • sysinfo crate APIs                                          │
│    • NVML for GPU                                                │
└────────────────────────┬────────────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. Aggregate Collected Metrics                                  │
│    • Combine results from all collectors                         │
│    • Calculate derived metrics                                   │
│    • Timestamp the data                                          │
└────────────────────────┬────────────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│ 5. Update Shared State (Arc<RwLock<SystemState>>)              │
│    • Acquire write lock                                          │
│    • Update current metrics                                      │
│    • Add to history buffer                                       │
│    • Release lock                                                │
└────────────────────────┬────────────────────────────────────────┘
                         ▼
         ┌───────────────┴───────────────┐
         ▼                               ▼
┌──────────────────────┐      ┌──────────────────────┐
│ 6a. Alert Manager    │      │ 6b. Storage Writer   │
│     Checks Thresholds│      │     Persists Metrics │
│     • Compare values │      │     • Write to DB    │
│     • Generate alerts│      │     • Batch writes   │
│     • Send notifs    │      │                      │
└──────────────────────┘      └──────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│ 7. UI Update (via GTK main thread)                              │
│    • Read from shared state                                      │
│    • Update widgets                                              │
│    • Redraw graphs                                               │
└─────────────────────────────────────────────────────────────────┘
```

### UI Event Flow

```
User Interaction (Click, Type, etc.)
         │
         ▼
GTK Event Handler
         │
         ▼
Application Logic
         │
         ├─→ Update Configuration
         │
         ├─→ Modify State
         │
         ├─→ Trigger Action (e.g., kill process)
         │
         └─→ Update UI
```

## Threading and Concurrency Model

### Thread Architecture

SystemVision uses a hybrid threading model:

1. **GTK Main Thread**
   - Handles all UI operations
   - Must not block
   - Receives updates via channels

2. **Tokio Runtime (Background)**
   - Async metric collection
   - Database operations
   - Alert processing
   - Network operations

3. **Thread Pool (CPU-bound tasks)**
   - Heavy computations
   - Data processing
   - Export operations

### Synchronization

```rust
// Shared state pattern
pub struct SystemState {
    metrics: Arc<RwLock<CurrentMetrics>>,
    config: Arc<RwLock<Config>>,
    alerts: Arc<RwLock<Vec<Alert>>>,
}

// Usage in collectors (async context)
async fn collect_metrics(state: Arc<RwLock<SystemState>>) {
    let metrics = collect().await?;
    
    let mut state = state.write().await;
    state.update_metrics(metrics);
}

// Usage in UI (GTK main thread)
fn update_ui(state: Arc<RwLock<SystemState>>) {
    let state = state.read().unwrap();
    let cpu_usage = state.metrics.cpu.usage;
    // Update UI widgets
}
```

### Communication Patterns

1. **State Sharing**: `Arc<RwLock<T>>` for shared mutable state
2. **Message Passing**: Channels for cross-thread communication
3. **GTK Signals**: For UI event propagation
4. **Async Streams**: For continuous data flows

## State Management

### State Structure

```rust
pub struct AppState {
    // Current system metrics
    current_metrics: SystemMetrics,
    
    // Historical data (ring buffer)
    history: VecDeque<SystemMetrics>,
    
    // Configuration
    config: Config,
    
    // Active alerts
    alerts: Vec<Alert>,
    
    // UI state
    selected_process: Option<ProcessId>,
    current_view: ViewType,
}
```

### State Updates

- **Atomic Updates**: Metrics updated atomically per collection cycle
- **History Management**: Fixed-size ring buffer for historical data
- **Persistence**: Configuration changes saved immediately
- **Consistency**: Read-write locks ensure consistency

### State Access Patterns

```rust
// Read-only access (multiple readers)
let state = app_state.read().unwrap();
let cpu_usage = state.current_metrics.cpu.usage;

// Write access (exclusive)
let mut state = app_state.write().unwrap();
state.current_metrics = new_metrics;
state.history.push_back(new_metrics.clone());
```

## Design Decisions

### 1. Why Rust?

- **Performance**: Native performance for system monitoring
- **Safety**: Memory safety without garbage collection
- **Concurrency**: Fearless concurrency with ownership system
- **Ecosystem**: Rich ecosystem for system programming

### 2. Why GTK4?

- **Native Look**: Integrates well with Linux desktop environments
- **Modern**: Latest GTK with improved performance
- **Libadwaita**: Beautiful, consistent UI components
- **Accessibility**: Built-in accessibility support

### 3. Why Async/Await?

- **Responsiveness**: Non-blocking UI operations
- **Efficiency**: Handle many concurrent operations
- **Scalability**: Easy to add more collectors
- **Tokio**: Mature, well-tested async runtime

### 4. Why SQLite?

- **Embedded**: No separate database server
- **Reliable**: ACID transactions
- **Efficient**: Fast time-series queries
- **Portable**: Single file database

### 5. Modular Collectors

- **Extensibility**: Easy to add new metrics
- **Testability**: Each collector can be tested independently
- **Maintainability**: Clear separation of concerns
- **Flexibility**: Enable/disable collectors as needed

### 6. Shared State with RwLock

- **Simplicity**: Easier to reason about than message passing
- **Performance**: Read-heavy workload benefits from RwLock
- **Consistency**: Single source of truth
- **Safety**: Rust's type system prevents data races

## Module Responsibilities

### `src/app.rs`
- Application initialization and lifecycle
- Component coordination
- Configuration management

### `src/collectors/`
- System metric collection
- Platform-specific implementations
- Error handling for unavailable metrics

### `src/ui/`
- User interface rendering
- Event handling
- Data visualization

### `src/storage/`
- Metric persistence
- Data export
- Database management

### `src/alerts/`
- Threshold monitoring
- Alert generation
- Notification delivery

### `src/models/`
- Data structure definitions
- Serialization/deserialization
- Type safety

### `src/utils/`
- Formatting utilities
- System helpers
- Common functions

## Extension Points

### Adding a New Collector

1. Create a new file in `src/collectors/`
2. Implement the `MetricCollector` trait
3. Add metric type to `src/models/metrics.rs`
4. Register collector in `src/app.rs`
5. Add UI view in `src/ui/`

### Adding a New UI View

1. Create a new file in `src/ui/`
2. Implement GTK4 widget
3. Add to main window navigation
4. Connect to state updates

### Adding a New Alert Type

1. Define alert type in `src/models/`
2. Add threshold configuration
3. Implement check logic in `src/alerts/manager.rs`
4. Add UI for configuration

## Performance Considerations

### Metric Collection

- **Parallel Collection**: Collectors run concurrently
- **Caching**: Avoid redundant system calls
- **Sampling**: Configurable collection intervals
- **Lazy Loading**: Load data only when needed

### UI Rendering

- **Incremental Updates**: Update only changed widgets
- **Throttling**: Limit update frequency
- **Efficient Graphs**: Use hardware acceleration
- **Lazy Rendering**: Render only visible views

### Memory Management

- **Ring Buffers**: Fixed-size history prevents unbounded growth
- **Database Cleanup**: Automatic old data removal
- **Resource Limits**: Configurable limits on stored data

## Security Considerations

- **Process Isolation**: Run with minimal privileges
- **Input Validation**: Validate all user inputs
- **Safe System Calls**: Use safe Rust wrappers
- **Configuration Security**: Validate configuration files

## Future Architecture Enhancements

- **Plugin System**: Dynamic loading of collectors
- **Remote Monitoring**: Network-based metric collection
- **Distributed Mode**: Monitor multiple systems
- **API Server**: REST API for external access
- **Custom Dashboards**: User-defined layouts

## Related Documentation

- [TECHNICAL_SPECIFICATION.md](../TECHNICAL_SPECIFICATION.md) - Detailed technical specifications
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development setup and workflow
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

---

For questions or clarifications about the architecture, please open a GitHub Discussion or contact the maintainers.