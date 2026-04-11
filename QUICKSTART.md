# Quick Start Guide

This guide will help you get SystemVision up and running quickly.

## Prerequisites

SystemVision requires Rust 1.70+ and several system libraries. Follow the steps below for your distribution.

## Automated Setup (Recommended)

We provide a script that automatically installs all required dependencies:

```bash
# Make the script executable (if not already)
chmod +x setup-deps.sh

# Run the setup script
./setup-deps.sh
```

The script supports:
- Ubuntu/Debian/Pop!_OS/Linux Mint
- Fedora/RHEL/CentOS
- Arch Linux/Manjaro
- openSUSE/SLES

## Manual Setup

If the automated script doesn't work for your distribution, install these packages manually:

### Ubuntu/Debian/Pop!_OS/Linux Mint

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libglib2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libgdk-pixbuf2.0-dev \
    libatk1.0-dev \
    libsqlite3-dev \
    libssl-dev \
    cmake \
    git
```

### Fedora/RHEL/CentOS

```bash
sudo dnf install -y \
    gcc gcc-c++ \
    pkg-config \
    gtk3-devel \
    glib2-devel \
    cairo-devel \
    pango-devel \
    gdk-pixbuf2-devel \
    atk-devel \
    sqlite-devel \
    openssl-devel \
    cmake \
    git
```

### Arch Linux/Manjaro

```bash
sudo pacman -S --needed \
    base-devel \
    pkg-config \
    gtk3 \
    glib2 \
    cairo \
    pango \
    gdk-pixbuf2 \
    atk \
    sqlite \
    openssl \
    cmake \
    git
```

### openSUSE/SLES

```bash
sudo zypper install -y \
    gcc gcc-c++ \
    pkg-config \
    gtk3-devel \
    glib2-devel \
    cairo-devel \
    pango-devel \
    gdk-pixbuf-devel \
    atk-devel \
    sqlite3-devel \
    libopenssl-devel \
    cmake \
    git
```

## Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version  # Should be 1.70 or later
```

## Build and Run

Once dependencies are installed:

```bash
# Build the project
cargo build

# Run the application
cargo run

# Or build and run in release mode (optimized)
cargo run --release
```

## Troubleshooting

### Error: "Package X was not found"

This means a required system library is missing. The error message will tell you which `.pc` file is missing.

**Solution:** Install the corresponding development package for your distribution. Common mappings:

| Missing .pc file | Ubuntu/Debian | Fedora | Arch |
|-----------------|---------------|---------|------|
| `gtk+-3.0.pc` | `libgtk-3-dev` | `gtk3-devel` | `gtk3` |
| `glib-2.0.pc` | `libglib2.0-dev` | `glib2-devel` | `glib2` |
| `cairo.pc` | `libcairo2-dev` | `cairo-devel` | `cairo` |
| `pango.pc` | `libpango1.0-dev` | `pango-devel` | `pango` |
| `atk.pc` | `libatk1.0-dev` | `atk-devel` | `atk` |
| `gdk-pixbuf-2.0.pc` | `libgdk-pixbuf2.0-dev` | `gdk-pixbuf2-devel` | `gdk-pixbuf2` |

### Error: "PKG_CONFIG_PATH environment variable is not set"

**Solution:** Install `pkg-config`:

```bash
# Ubuntu/Debian
sudo apt-get install pkg-config

# Fedora
sudo dnf install pkg-config

# Arch
sudo pacman -S pkg-config
```

### Verify Dependencies

Check if all required libraries are installed:

```bash
pkg-config --modversion gtk+-3.0
pkg-config --modversion glib-2.0
pkg-config --modversion cairo
pkg-config --modversion pango
pkg-config --modversion atk
pkg-config --modversion gdk-pixbuf-2.0
pkg-config --modversion sqlite3
```

Each command should output a version number. If you get an error, that library is missing.

### Still Having Issues?

1. **Check the full error message** - It usually tells you exactly what's missing
2. **Run the setup script** - `./setup-deps.sh` (if you haven't already)
3. **Check DEVELOPMENT.md** - For more detailed troubleshooting
4. **Open an issue** - On GitHub with your error message and OS details

## Next Steps

Once you have SystemVision running:

1. **Explore the UI** - Check out the different monitoring views
2. **Configure settings** - Adjust update intervals and thresholds
3. **Read the docs** - See [README.md](README.md) for features and usage
4. **Contribute** - See [CONTRIBUTING.md](CONTRIBUTING.md) to get involved

## Quick Commands Reference

```bash
# Build (debug mode)
cargo build

# Build (release mode, optimized)
cargo build --release

# Run (debug mode)
cargo run

# Run (release mode)
cargo run --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Clean build artifacts
cargo clean
```

## System Requirements

- **OS**: Linux (kernel 4.0+)
- **Rust**: 1.70 or later
- **RAM**: 100MB minimum
- **Disk**: 50MB for binary + dependencies

## Optional: GPU Monitoring

For NVIDIA GPU monitoring, install NVIDIA drivers and utilities:

```bash
# Ubuntu/Debian
sudo apt-get install nvidia-utils

# Fedora
sudo dnf install xorg-x11-drv-nvidia-cuda

# Arch
sudo pacman -S nvidia-utils
```

## Getting Help

- **Documentation**: [DEVELOPMENT.md](DEVELOPMENT.md)
- **Architecture**: [ARCHITECTURE.md](ARCHITECTURE.md)
- **Issues**: [GitHub Issues](https://github.com/YOUR_USERNAME/system_vision/issues)
- **Discussions**: [GitHub Discussions](https://github.com/YOUR_USERNAME/system_vision/discussions)

---

**Ready to contribute?** Check out [CONTRIBUTING.md](CONTRIBUTING.md)!
