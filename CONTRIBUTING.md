# Contributing to Rust Pi Zero Example

Thank you for your interest in contributing to this project! We welcome contributions from developers of all skill levels.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/rust_pi_zero_example.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes (both simulation and hardware if possible)
6. Submit a pull request

## 📋 Development Setup

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- For hardware testing: Raspberry Pi with GPIO access

### Local Development

```bash
# Clone the repository
git clone https://github.com/your-username/rust_pi_zero_example.git
cd rust_pi_zero_example

# Install dependencies and build
cargo build

# Run in simulation mode
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Cross-compilation Setup (Optional)

```bash
# Install ARM target for Pi deployment testing
rustup target add arm-unknown-linux-gnueabihf

# Install cross-compilation tools (Ubuntu/Debian)
sudo apt-get install gcc-arm-linux-gnueabihf
```

## 🎯 How to Contribute

### 🐛 Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/your-username/rust_pi_zero_example/issues) to avoid duplicates.

When filing a bug report, include:

- **Clear description** of the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, hardware)
- **Logs or error messages** if applicable
- **Screenshots** for display-related issues

### 💡 Suggesting Features

Feature requests are welcome! Please:

- Check existing [issues](https://github.com/your-username/rust_pi_zero_example/issues) and [discussions](https://github.com/your-username/rust_pi_zero_example/discussions)
- Provide a clear description of the feature
- Explain the use case and benefits
- Consider implementation complexity
- Be open to discussion and feedback

### 🔧 Code Contributions

#### Areas for Contribution

- **Hardware Support**: Additional sensors, displays, or Pi models
- **API Features**: New endpoints or enhanced functionality
- **Display Graphics**: Improved UI elements or animations
- **Performance**: Optimization and efficiency improvements
- **Documentation**: Examples, tutorials, or API docs
- **Testing**: Unit tests, integration tests, or hardware tests
- **Deployment**: CI/CD, packaging, or installation improvements

#### Code Style Guidelines

- Follow [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html)
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` and address warnings
- Write clear, self-documenting code
- Add comments for complex logic
- Include docstrings for public APIs

#### Testing Requirements

- Test both simulation and hardware modes when applicable
- Add unit tests for new functionality
- Ensure existing tests pass: `cargo test`
- Test API endpoints with curl or similar tools
- Verify cross-compilation builds: `cargo build --target arm-unknown-linux-gnueabihf`

#### Pull Request Process

1. **Create a feature branch** from `main`
2. **Make focused commits** with clear messages
3. **Update documentation** if needed
4. **Add tests** for new functionality
5. **Ensure CI passes** (formatting, linting, tests)
6. **Update CHANGELOG.md** if applicable
7. **Submit pull request** with clear description

#### Commit Message Format

Use clear, descriptive commit messages:

```
feat: add temperature threshold alerts
fix: resolve SPI communication timeout
docs: update API endpoint documentation
test: add unit tests for sensor simulation
refactor: improve display abstraction layer
```

Prefixes:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or modifications
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

## 🏗️ Project Structure

```
rust_pi_zero_example/
├── src/
│   └── main.rs              # Main application logic
├── docs/                    # Additional documentation
├── examples/                # Usage examples
├── tests/                   # Integration tests
├── Cargo.toml               # Dependencies and metadata
├── README.md                # Main documentation
├── CONTRIBUTING.md          # This file
├── CHANGELOG.md             # Version history
├── LICENSE                  # MIT License
└── .github/                 # GitHub workflows and templates
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in simulation mode
cargo test --features simulation

# Run tests in hardware mode (requires Pi)
cargo test --features hardware --no-default-features
```

### Test Categories

- **Unit Tests**: Individual function testing
- **Integration Tests**: API endpoint testing
- **Hardware Tests**: GPIO and SPI functionality
- **Simulation Tests**: Mock display and sensor testing

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_simulation() {
        let temp = simulate_temperature_reading();
        assert!(temp >= 18.0 && temp <= 35.0);
    }

    #[tokio::test]
    async fn test_api_status_endpoint() {
        // Test API functionality
    }
}
```

## 📚 Documentation

### Types of Documentation

- **Code Comments**: Explain complex logic
- **API Documentation**: Rust docs for public APIs
- **User Guides**: README and tutorial content
- **Developer Docs**: Architecture and contribution guides

### Writing Documentation

```rust
/// Simulates reading temperature from a sensor
/// 
/// Returns a temperature value between 18.0 and 35.0 degrees Celsius
/// with pseudo-random variation based on current time.
/// 
/// # Examples
/// 
/// ```
/// let temp = simulate_temperature_reading();
/// assert!(temp >= 18.0 && temp <= 35.0);
/// ```
fn simulate_temperature_reading() -> f32 {
    // Implementation
}
```

## 🔍 Code Review Process

### For Contributors

- Respond to feedback promptly and professionally
- Make requested changes in separate commits
- Ask questions if feedback is unclear
- Be open to suggestions and alternative approaches

### For Reviewers

- Be constructive and specific in feedback
- Focus on code quality, not personal preferences
- Suggest improvements with examples
- Approve when ready, request changes when needed

## 🌟 Recognition

Contributors will be:

- Listed in the project's contributors section
- Mentioned in release notes for significant contributions
- Invited to join the project's maintainer team for ongoing contributors

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Email**: [your-email@example.com] for private inquiries

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of background, experience level, or identity.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Personal attacks or trolling
- Public or private harassment
- Publishing others' private information without permission
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team. All complaints will be reviewed and investigated promptly and fairly.

## 🙏 Thank You

Your contributions help make this project better for everyone. Whether you're fixing a typo, adding a feature, or helping with documentation, every contribution is valued and appreciated!

---

**Happy coding! 🦀**
