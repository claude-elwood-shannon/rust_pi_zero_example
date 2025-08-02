# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup
- ST7789 LCD display integration
- Simulation mode for development without hardware
- RESTful web API with CORS support
- GPIO LED control with heartbeat functionality
- Automated deployment scripts
- Cross-compilation support for ARM
- Systemd service integration
- Comprehensive documentation

### Features
- **Display Support**: ST7789 240x240 IPS LCD with embedded-graphics
- **Dual Mode**: Hardware and simulation modes with feature flags
- **Web API**: Real-time monitoring and control endpoints
- **Sensor Simulation**: Temperature and humidity data generation
- **LED Control**: GPIO-based status indicator
- **Deployment**: Automated cross-compilation and deployment to Pi
- **Service Management**: Systemd integration for production deployment

## [0.1.0] - 2025-03-08

### Added
- Initial release
- Basic project structure
- Core functionality implementation
- Documentation and examples

### Technical Details
- Rust 2021 edition
- Tokio async runtime
- Warp web framework
- RPPAL GPIO library integration
- ST7789 display driver
- Embedded graphics support
- Feature-based compilation
- Cross-platform compatibility

### API Endpoints
- `GET /` - Welcome message
- `GET /status` - System status and metrics
- `GET /sensor` - Sensor data readings
- `GET /display` - LCD content (simulation mode)
- `POST /led` - LED control

### Hardware Support
- Raspberry Pi Zero/3B+/4B
- ST7789 240x240 IPS LCD
- GPIO LED control
- SPI communication
- I2C sensor support (framework)

### Development Features
- Desktop simulation mode
- Mock display implementation
- Console output visualization
- Hot-reload development
- Comprehensive logging
- Error handling and recovery

---

## Release Notes

### Version 0.1.0 - Initial Release

This is the first public release of the Rust Pi Zero Example project. It provides a complete foundation for Raspberry Pi development with LCD display integration and web API functionality.

**Key Highlights:**
- 🚀 Ready-to-deploy Raspberry Pi application
- 🖥️ Desktop simulation for development
- 📱 RESTful API for remote monitoring
- 🔧 Professional deployment tools
- 📚 Comprehensive documentation

**Getting Started:**
```bash
git clone https://github.com/your-username/rust_pi_zero_example.git
cd rust_pi_zero_example
cargo run  # Starts in simulation mode
```

**Hardware Deployment:**
```bash
./deploy.sh raspberrypi.local pi
```

For detailed instructions, see [README.md](README.md) and [README_SIMULATION.md](README_SIMULATION.md).
