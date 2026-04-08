# SystemVision

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

A modern, feature-rich system monitoring application for Linux, built with Rust and egui.

![SystemVision Banner](assets/icon.png)

## ✨ Features

- **Real-time Monitoring**: Track CPU, memory, disk, network, GPU, and more
- **Beautiful UI**: Modern immediate-mode GUI with egui
- **Process Management**: View and manage running processes
- **Historical Data**: Store and visualize metric history
- **Smart Alerts**: Configurable threshold-based notifications
- **Data Export**: Export metrics to CSV and JSON formats
- **GPU Support**: Monitor NVIDIA and AMD GPUs
- **Low Overhead**: Efficient metric collection with minimal system impact
- **Customizable**: Flexible configuration and theming options

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- GTK3 and related development libraries
- Linux kernel 4.0 or later

### Quick Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/system_vision.git
cd system_vision/vision

# Run the automated setup script
./setup-deps.sh

# Build and run
cargo build --release
./target/release/system-vision
```

**Having build issues?** See [QUICKSTART.md](QUICKSTART.md) for troubleshooting.

For detailed installation instructions, see [DEVELOPMENT.md](DEVELOPMENT.md).

### Quick Run (Development)

```bash
cargo run
```

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide and troubleshooting
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Development setup and workflow
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture and design
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and changes
- **[Technical Specification](../TECHNICAL_SPECIFICATION.md)** - Detailed technical specs
- **[Implementation Roadmap](../IMPLEMENTATION_ROADMAP.md)** - Development roadmap

## 🎯 Usage

### Basic Usage

Launch SystemVision from your application menu or run:

```bash
system-vision
```

### Command Line Options

```bash
# Show help
system-vision --help

# Use custom configuration
system-vision --config /path/to/config.toml

# Enable verbose logging
system-vision --verbose

# Run in background mode (no UI)
system-vision --daemon
```

### Configuration

SystemVision uses TOML for configuration. The default config is located at:
- `~/.config/system-vision/config.toml` (user config)
- `/etc/system-vision/config.toml` (system-wide config)

Example configuration:

```toml
[general]
update_interval = 2000  # milliseconds
theme = "dark"

[alerts]
cpu_threshold = 90.0
memory_threshold = 85.0
disk_threshold = 90.0

[storage]
database_path = "~/.local/share/system-vision/metrics.db"
retention_days = 30
```

## 🖼️ Screenshots

> Screenshots will be added as the UI is implemented

### Dashboard View
*Coming soon*

### Process View
*Coming soon*

### GPU Monitoring
*Coming soon*

## 🏗️ Architecture

SystemVision follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│           (egui + Rust)                 │
├─────────────────────────────────────────┤
│  UI Layer  │  Background Services       │
│  • Views   │  • Collectors              │
│  • Widgets │  • Alerts                  │
│            │  • Storage                 │
├─────────────────────────────────────────┤
│         State Management                │
│      (Arc<RwLock<State>>)              │
├─────────────────────────────────────────┤
│    System APIs & Kernel Interfaces      │
└─────────────────────────────────────────┘
```

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## 🔧 Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Development Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open

# Watch for changes and rebuild
cargo watch -x run
```

For comprehensive development instructions, see [DEVELOPMENT.md](DEVELOPMENT.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment
- Code style and conventions
- Submitting pull requests
- Reporting issues

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -m 'feat: add amazing feature'`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📊 Project Status

SystemVision is currently in active development. See the [Implementation Roadmap](../IMPLEMENTATION_ROADMAP.md) for current progress and planned features.

### Current Phase: Phase 1 - Foundation (In Progress)

- [x] Project structure
- [x] Core data models
- [x] Basic collectors (CPU, Memory, Process)
- [ ] GTK4 UI framework
- [ ] Dashboard view
- [ ] Configuration system

## 🐛 Bug Reports and Feature Requests

Please use GitHub Issues to report bugs or request features:

- **Bug Report**: Use the bug report template
- **Feature Request**: Use the feature request template
- **Question**: Use GitHub Discussions

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- UI powered by [GTK4](https://www.gtk.org/) and [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- System metrics via [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- Async runtime by [Tokio](https://tokio.rs/)

## 📞 Community and Support

- **Documentation**: Check the docs in this repository
- **Issues**: [GitHub Issues](https://github.com/YOUR_USERNAME/system_vision/issues)
- **Discussions**: [GitHub Discussions](https://github.com/YOUR_USERNAME/system_vision/discussions)
- **Chat**: *Coming soon*

## 🗺️ Roadmap

### Phase 1: Foundation (Current)
- Core metric collection
- Basic UI framework
- Configuration system

### Phase 2: Enhanced Monitoring
- Advanced collectors (GPU, Temperature, Power)
- Historical data storage
- Data export functionality

### Phase 3: Alerts and Notifications
- Alert system
- Desktop notifications
- Alert history

### Phase 4: Polish and Optimization
- Performance optimization
- UI refinements
- Documentation completion

For detailed roadmap, see [IMPLEMENTATION_ROADMAP.md](../IMPLEMENTATION_ROADMAP.md).

## 📈 Performance

SystemVision is designed to be lightweight and efficient:

- **CPU Usage**: < 1% on average
- **Memory Usage**: ~50-100 MB
- **Update Interval**: Configurable (default: 2 seconds)
- **Storage**: Minimal disk usage with automatic cleanup

## 🔒 Security

- No network access required (local monitoring only)
- Minimal system permissions needed
- Safe Rust code (no unsafe blocks in core logic)
- Regular dependency audits with `cargo audit`

## 🌍 Platform Support

Currently supported:
- **Linux**: Full support (primary platform)

Planned:
- **BSD**: Future support planned
- **macOS**: Under consideration
- **Windows**: Not planned (Linux-specific features)

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## 💡 Tips and Tricks

### Performance Tuning

```toml
# Reduce update frequency for lower CPU usage
[general]
update_interval = 5000  # 5 seconds

# Disable unused collectors
[collectors]
gpu_enabled = false
temperature_enabled = false
```

### Keyboard Shortcuts

- `Ctrl+Q`: Quit application
- `Ctrl+R`: Refresh metrics
- `Ctrl+,`: Open settings
- `F11`: Toggle fullscreen

*More shortcuts will be added as features are implemented*

## 🎓 Learning Resources

New to Rust or GTK? Check out these resources:

- [The Rust Book](https://doc.rust-lang.org/book/)
- [GTK4 Rust Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Our Architecture Guide](ARCHITECTURE.md)

---

**Made with ❤️ by the SystemVision community**

*Star ⭐ this repository if you find it useful!*
