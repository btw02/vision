# Contributing to SystemVision

Thank you for your interest in contributing to SystemVision! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment Setup](#development-environment-setup)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Commit Message Conventions](#commit-message-conventions)
- [Issue Reporting](#issue-reporting)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Respect differing viewpoints and experiences
- Accept responsibility and apologize for mistakes

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/system_vision.git
   cd system_vision/vision
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/system_vision.git
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Environment Setup

### Prerequisites

- **Rust**: 1.70 or later (stable toolchain recommended)
- **System Libraries**:
  - GTK3 development files (`libgtk-3-dev`)
  - GLib2 development files (`libglib2.0-dev`)
  - Cairo development files (`libcairo2-dev`)
  - Pango development files (`libpango1.0-dev`)
  - SQLite development files (`libsqlite3-dev`)
  - Additional system monitoring libraries (see [DEVELOPMENT.md](DEVELOPMENT.md))

### Initial Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Ubuntu/Debian)
# Use the automated setup script
./setup-deps.sh

# Or install manually
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libglib2.0-dev libcairo2-dev \
    libpango1.0-dev libgdk-pixbuf2.0-dev libatk1.0-dev libsqlite3-dev \
    build-essential pkg-config

# Build the project
cd vision
cargo build

# Run tests
cargo test

# Run the application
cargo run
```

For detailed setup instructions, see [DEVELOPMENT.md](DEVELOPMENT.md).

## Code Style Guidelines

### Rust Conventions

We follow the official Rust style guidelines with some project-specific conventions:

#### Formatting

- Use `rustfmt` for automatic formatting:
  ```bash
  cargo fmt
  ```
- Maximum line length: 100 characters
- Use 4 spaces for indentation (no tabs)

#### Naming Conventions

- **Types**: `PascalCase` (e.g., `SystemMetrics`, `CpuCollector`)
- **Functions/Methods**: `snake_case` (e.g., `collect_metrics`, `update_ui`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_HISTORY_SIZE`, `DEFAULT_INTERVAL`)
- **Modules**: `snake_case` (e.g., `collectors`, `ui`)

#### Code Organization

- Keep functions focused and under 50 lines when possible
- Use meaningful variable names (avoid single-letter names except in closures)
- Add documentation comments (`///`) for public APIs
- Group related functionality into modules
- Prefer composition over inheritance

#### Error Handling

- Use `Result<T, E>` for operations that can fail
- Use `anyhow::Result` for application-level errors
- Use specific error types for library code
- Avoid `unwrap()` and `expect()` in production code
- Provide context with error messages

Example:
```rust
use anyhow::{Context, Result};

pub fn read_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read config file: {}", path.display()))?;

    toml::from_str(&content)
        .context("Failed to parse config file")
}
```

#### Documentation

- Add module-level documentation (`//!`) at the top of each file
- Document all public functions, structs, and enums
- Include examples in documentation when helpful
- Use `# Examples` sections in doc comments

Example:
```rust
/// Collects CPU usage metrics from the system.
///
/// This function queries the system for current CPU usage across all cores
/// and returns aggregated metrics.
///
/// # Examples
///
/// ```
/// use system_vision::collectors::CpuCollector;
///
/// let collector = CpuCollector::new();
/// let metrics = collector.collect()?;
/// println!("CPU usage: {}%", metrics.usage);
/// ```
///
/// # Errors
///
/// Returns an error if the system metrics cannot be read.
pub fn collect_cpu_metrics() -> Result<CpuMetrics> {
    // Implementation
}
```

#### Async Code

- Use `tokio` for async runtime
- Prefer `async/await` over manual futures
- Use `tokio::spawn` for concurrent tasks
- Handle cancellation gracefully

### Clippy Lints

Run Clippy before submitting:
```bash
cargo clippy -- -D warnings
```

We enforce the following lint levels:
- `clippy::all`: Deny
- `clippy::pedantic`: Warn
- `clippy::nursery`: Allow (but review suggestions)

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test collectors::

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Writing Tests

- Write unit tests in the same file as the code (in a `tests` module)
- Write integration tests in the `tests/` directory
- Aim for >80% code coverage for new features
- Test edge cases and error conditions

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_collector_initialization() {
        let collector = CpuCollector::new();
        assert!(collector.is_ok());
    }

    #[test]
    fn test_invalid_config_returns_error() {
        let result = Config::from_str("invalid toml");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_async_metric_collection() {
        let collector = CpuCollector::new().unwrap();
        let metrics = collector.collect().await;
        assert!(metrics.is_ok());
    }
}
```

## Submitting Changes

### Before Submitting

1. **Update your branch** with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run the full test suite**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Update documentation** if you changed APIs or added features

4. **Add tests** for new functionality

5. **Update CHANGELOG.md** under the "Unreleased" section

### Commit Message Conventions

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, dependency updates

#### Examples

```
feat(collectors): add GPU temperature monitoring

Implement GPU temperature collection using NVML for NVIDIA cards
and sysfs for AMD cards. Includes fallback for unsupported hardware.

Closes #123
```

```
fix(ui): correct memory usage percentage calculation

The memory usage was showing incorrect percentages due to
integer overflow in the calculation. Changed to use u64
for intermediate calculations.

Fixes #456
```

```
docs(readme): add installation instructions for Arch Linux
```

#### Scope

Common scopes in this project:
- `collectors`: Metric collection modules
- `ui`: User interface components
- `storage`: Database and export functionality
- `alerts`: Alert system
- `config`: Configuration handling
- `models`: Data models and types

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce the behavior
3. **Expected Behavior**: What you expected to happen
4. **Actual Behavior**: What actually happened
5. **Environment**:
   - OS and version
   - Rust version (`rustc --version`)
   - SystemVision version
6. **Logs**: Relevant log output or error messages
7. **Screenshots**: If applicable

Use the bug report template when creating issues.

### Feature Requests

When requesting features, please include:

1. **Problem Statement**: What problem does this solve?
2. **Proposed Solution**: How should it work?
3. **Alternatives**: Other solutions you've considered
4. **Additional Context**: Any other relevant information

## Pull Request Process

### PR Checklist

Before submitting a pull request, ensure:

- [ ] Code follows the style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes with no warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commit messages follow conventions
- [ ] Branch is up to date with main

### PR Description

Your PR description should include:

1. **Summary**: Brief description of changes
2. **Motivation**: Why is this change needed?
3. **Changes**: List of specific changes made
4. **Testing**: How was this tested?
5. **Screenshots**: If UI changes are involved
6. **Related Issues**: Link to related issues (e.g., "Closes #123")

### Review Process

1. **Automated Checks**: CI will run tests and linting
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, a maintainer will merge

### After Merge

- Delete your feature branch
- Pull the latest main branch
- Celebrate! 🎉

## Development Workflow

### Typical Workflow

1. Pick an issue or create one for your feature
2. Create a feature branch
3. Make your changes
4. Write/update tests
5. Update documentation
6. Commit with conventional commit messages
7. Push to your fork
8. Create a pull request
9. Address review feedback
10. Merge!

### Getting Help

- **Documentation**: Check [DEVELOPMENT.md](DEVELOPMENT.md) and [ARCHITECTURE.md](ARCHITECTURE.md)
- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Search existing issues before creating new ones
- **Chat**: Join our community chat (if available)

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- The CHANGELOG

Thank you for contributing to SystemVision! 🚀
