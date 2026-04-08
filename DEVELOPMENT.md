# Development Guide

This guide covers everything you need to know to set up your development environment and start contributing to SystemVision.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Building the Project](#building-the-project)
- [Running the Application](#running-the-application)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Debugging](#debugging)
- [IDE Setup](#ide-setup)
- [Useful Commands](#useful-commands)
- [Common Issues](#common-issues)
- [Performance Profiling](#performance-profiling)

## Prerequisites

### Required Software

#### Rust Toolchain

SystemVision requires Rust 1.70 or later. Install using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version  # Should be 1.70 or later
cargo --version
```

#### System Libraries

The required system libraries vary by distribution:

**Ubuntu/Debian:**
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

**Fedora:**
```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
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

**Arch Linux:**
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

**openSUSE:**
```bash
sudo zypper install -y \
    gcc \
    gcc-c++ \
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

### Optional Dependencies

#### GPU Monitoring (NVIDIA)

For NVIDIA GPU support:

```bash
# Ubuntu/Debian
sudo apt-get install -y nvidia-utils-535  # or latest version

# Fedora
sudo dnf install -y xorg-x11-drv-nvidia-cuda

# Arch Linux
sudo pacman -S nvidia-utils
```

#### Development Tools

Recommended tools for development:

```bash
# Install cargo tools
cargo install cargo-watch      # Auto-rebuild on file changes
cargo install cargo-edit        # Manage dependencies
cargo install cargo-outdated    # Check for outdated dependencies
cargo install cargo-audit       # Security vulnerability scanning
cargo install cargo-tarpaulin   # Code coverage
```

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/YOUR_USERNAME/system_vision.git
cd system_vision/vision
```

### 2. Verify Dependencies

Check that all system dependencies are installed:

```bash
pkg-config --modversion gtk4
pkg-config --modversion libadwaita-1
pkg-config --modversion sqlite3
```

### 3. Build Dependencies

First build will download and compile all Rust dependencies:

```bash
cargo build
```

This may take 5-10 minutes on the first run.

### 4. Verify Installation

Run the test suite to ensure everything is working:

```bash
cargo test
```

## Building the Project

### Debug Build

For development, use debug builds (faster compilation, includes debug symbols):

```bash
cargo build
```

The binary will be at: `target/debug/system-vision`

### Release Build

For production or performance testing:

```bash
cargo build --release
```

The optimized binary will be at: `target/release/system-vision`

### Build Options

```bash
# Build with all features
cargo build --all-features

# Build without default features
cargo build --no-default-features

# Build specific features
cargo build --features "gpu-support,advanced-metrics"

# Clean build (remove all build artifacts)
cargo clean && cargo build
```

### Incremental Compilation

Rust uses incremental compilation by default. To disable:

```bash
CARGO_INCREMENTAL=0 cargo build
```

## Running the Application

### Development Mode

Run directly with cargo:

```bash
cargo run
```

With arguments:
```bash
cargo run -- --config custom_config.toml
cargo run -- --verbose
cargo run -- --help
```

### Running the Binary

After building:

```bash
./target/debug/system-vision
```

### Environment Variables

Useful environment variables for development:

```bash
# Enable Rust backtrace on panic
RUST_BACKTRACE=1 cargo run

# Full backtrace
RUST_BACKTRACE=full cargo run

# Enable logging
RUST_LOG=debug cargo run
RUST_LOG=system_vision=trace cargo run

# GTK debugging
GTK_DEBUG=interactive cargo run  # Opens GTK Inspector
```

## Development Workflow

### Typical Development Cycle

1. **Make Changes**: Edit source files
2. **Check Compilation**: `cargo check` (fast syntax check)
3. **Run Tests**: `cargo test`
4. **Run Application**: `cargo run`
5. **Format Code**: `cargo fmt`
6. **Lint Code**: `cargo clippy`
7. **Commit Changes**: Follow commit conventions

### Auto-Rebuild on Changes

Use `cargo-watch` for automatic rebuilding:

```bash
# Auto-rebuild and run
cargo watch -x run

# Auto-rebuild and test
cargo watch -x test

# Auto-rebuild, test, and run
cargo watch -x test -x run

# Clear screen between runs
cargo watch -c -x run
```

### Quick Iteration

For fast iteration during UI development:

```bash
# Check only (no codegen, very fast)
cargo check

# Check with clippy
cargo clippy --all-targets

# Build and run in one command
cargo run --release
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_cpu_collector

# Run tests in a module
cargo test collectors::

# Run tests matching a pattern
cargo test cpu

# Run ignored tests
cargo test -- --ignored

# Run tests in parallel (default)
cargo test

# Run tests sequentially
cargo test -- --test-threads=1
```

### Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Doc tests only
cargo test --doc

# Benchmark tests
cargo test --benches
```

### Code Coverage

Using `cargo-tarpaulin`:

```bash
# Generate coverage report
cargo tarpaulin --out Html

# Generate and open report
cargo tarpaulin --out Html && xdg-open tarpaulin-report.html

# Coverage for specific package
cargo tarpaulin --packages system-vision

# Exclude tests from coverage
cargo tarpaulin --exclude-files 'tests/*'
```

### Writing Tests

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = some_function();
        assert_eq!(result, expected_value);
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_case() {
        panic_function();
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]  // Run with: cargo test -- --ignored
    fn expensive_test() {
        // Long-running test
    }
}
```

## Debugging

### Using rust-gdb

```bash
# Build with debug symbols
cargo build

# Run with gdb
rust-gdb target/debug/system-vision

# Common gdb commands:
# (gdb) run                    # Start program
# (gdb) break main             # Set breakpoint
# (gdb) continue               # Continue execution
# (gdb) backtrace              # Show stack trace
# (gdb) print variable         # Print variable value
# (gdb) quit                   # Exit gdb
```

### Using lldb

```bash
rust-lldb target/debug/system-vision
```

### Print Debugging

```rust
// Simple debug print
println!("Value: {:?}", value);

// Pretty print
println!("{:#?}", complex_struct);

// Debug macro (only in debug builds)
dbg!(variable);

// Conditional debug
#[cfg(debug_assertions)]
println!("Debug info: {:?}", data);
```

### Logging

SystemVision uses the `log` crate:

```rust
use log::{debug, info, warn, error};

debug!("Detailed debug information");
info!("General information");
warn!("Warning message");
error!("Error occurred: {}", error);
```

Run with logging:
```bash
RUST_LOG=debug cargo run
RUST_LOG=system_vision=trace cargo run
```

### GTK Inspector

Debug GTK UI issues:

```bash
GTK_DEBUG=interactive cargo run
```

This opens the GTK Inspector where you can:
- Inspect widget hierarchy
- View CSS styles
- Monitor signals
- Check properties

### Memory Debugging

Using Valgrind:

```bash
# Build with debug symbols
cargo build

# Run with valgrind
valgrind --leak-check=full ./target/debug/system-vision

# With suppressions for GTK
valgrind --leak-check=full --suppressions=gtk.supp ./target/debug/system-vision
```

## IDE Setup

### Visual Studio Code

#### Recommended Extensions

Install these extensions:
- `rust-analyzer`: Rust language server
- `CodeLLDB`: Debugger
- `crates`: Manage dependencies
- `Better TOML`: TOML syntax highlighting
- `Error Lens`: Inline error messages

#### Configuration

Create `.vscode/settings.json`:

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.inlayHints.enable": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

Create `.vscode/launch.json` for debugging:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug SystemVision",
            "cargo": {
                "args": ["build", "--bin=system-vision"],
                "filter": {
                    "name": "system-vision",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}/vision",
            "env": {
                "RUST_BACKTRACE": "1",
                "RUST_LOG": "debug"
            }
        }
    ]
}
```

#### Tasks

Create `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo build",
            "type": "shell",
            "command": "cargo build",
            "group": "build"
        },
        {
            "label": "cargo test",
            "type": "shell",
            "command": "cargo test",
            "group": "test"
        },
        {
            "label": "cargo run",
            "type": "shell",
            "command": "cargo run",
            "group": "none"
        }
    ]
}
```

### RustRover / IntelliJ IDEA

1. Install Rust plugin
2. Open project directory
3. RustRover will auto-detect Cargo.toml
4. Configure run configurations in Run → Edit Configurations

### Vim/Neovim

Use `rust-analyzer` with your LSP client:

```vim
" Using coc.nvim
:CocInstall coc-rust-analyzer

" Using native LSP (Neovim 0.5+)
lua << EOF
require'lspconfig'.rust_analyzer.setup{}
EOF
```

## Useful Commands

### Cargo Commands

```bash
# Check code without building
cargo check

# Check all targets
cargo check --all-targets

# Format code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check

# Lint with clippy
cargo clippy

# Clippy with all targets
cargo clippy --all-targets -- -D warnings

# Update dependencies
cargo update

# Show dependency tree
cargo tree

# Show outdated dependencies
cargo outdated

# Audit dependencies for security issues
cargo audit

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean

# Show build timings
cargo build --timings

# Expand macros
cargo expand

# Show assembly output
cargo asm
```

### Project-Specific Commands

```bash
# Run with custom config
cargo run -- --config config/custom.toml

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture

# Build for different targets
cargo build --target x86_64-unknown-linux-gnu

# Check binary size
cargo bloat --release

# Profile compilation time
cargo build -Z timings
```

## Common Issues

### Issue: GTK4 not found

**Error:**
```
Package gtk4 was not found in the pkg-config search path
```

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install libgtk-4-dev

# Verify installation
pkg-config --modversion gtk4
```

### Issue: Linker errors

**Error:**
```
error: linking with `cc` failed
```

**Solution:**
```bash
# Install build essentials
sudo apt-get install build-essential

# Or try using lld linker
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### Issue: Permission denied for system metrics

**Error:**
```
Permission denied when reading /proc or /sys
```

**Solution:**
Run with appropriate permissions or add user to required groups:
```bash
# Add user to required groups
sudo usermod -a -G video,input $USER

# Log out and back in for changes to take effect
```

### Issue: Slow compilation

**Solutions:**

1. Use `cargo check` instead of `cargo build` during development
2. Enable parallel compilation:
   ```bash
   # In ~/.cargo/config.toml
   [build]
   jobs = 8  # Number of CPU cores
   ```
3. Use `sccache` for caching:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```
4. Use `mold` linker (faster than ld):
   ```bash
   # Install mold
   sudo apt-get install mold
   
   # In ~/.cargo/config.toml
   [target.x86_64-unknown-linux-gnu]
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=mold"]
   ```

### Issue: Out of memory during compilation

**Solution:**
```bash
# Reduce parallel jobs
cargo build -j 2

# Or set in config
[build]
jobs = 2
```

## Performance Profiling

### CPU Profiling

Using `perf`:

```bash
# Build with debug symbols
cargo build --release

# Record profile
perf record --call-graph=dwarf ./target/release/system-vision

# View report
perf report

# Generate flamegraph
cargo install flamegraph
cargo flamegraph
```

### Memory Profiling

Using `heaptrack`:

```bash
# Install heaptrack
sudo apt-get install heaptrack

# Profile application
heaptrack ./target/release/system-vision

# Analyze results
heaptrack_gui heaptrack.system-vision.*.gz
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Benchmark specific test
cargo bench --bench benchmark_name
```

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [GTK4 Rust Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

## Getting Help

- **Documentation**: Check the docs in this repository
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions
- **Community**: Join our community chat (if available)

---

Happy coding! If you encounter any issues not covered here, please open an issue or contribute to this documentation.