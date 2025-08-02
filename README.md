# 🚀 Rust Pi Zero Example

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A comprehensive Rust application for Raspberry Pi Zero featuring ST7789 LCD display integration, sensor monitoring, and web API. Supports both hardware deployment and desktop simulation for development.

## ✨ Features

- 🖥️ **ST7789 LCD Display Support** (240x240 IPS)
- 📊 **Real-time Sensor Monitoring** (Temperature & Humidity)
- 🌐 **RESTful Web API** with CORS support
- 💡 **GPIO LED Control** with heartbeat functionality
- 🔄 **Dual Mode Operation** (Hardware/Simulation)
- 📱 **Cross-platform Development** (simulate without Pi hardware)
- 🚀 **Automated Deployment** scripts included
- 📋 **Comprehensive Logging** with configurable levels

## 🎯 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- For hardware mode: Raspberry Pi Zero/3B+/4B with GPIO access

### 🖥️ Desktop Simulation (No Hardware Required)

```bash
# Clone the repository
git clone https://github.com/your-username/rust_pi_zero_example.git
cd rust_pi_zero_example

# Run in simulation mode (default)
cargo run

# Or use the convenience script
chmod +x run_simulation.sh
./run_simulation.sh
```

### 🔧 Hardware Deployment

```bash
# Build for Raspberry Pi (cross-compilation)
cargo build --release --target arm-unknown-linux-gnueabihf --features hardware --no-default-features

# Deploy to Pi (requires SSH access)
./deploy.sh [pi_hostname] [username]
```

## 📱 API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/` | GET | Welcome message | Plain text |
| `/status` | GET | System status & metrics | JSON |
| `/sensor` | GET | Latest sensor readings | JSON |
| `/display` | GET | LCD content (simulation only) | JSON |
| `/led` | POST | Control LED state | JSON |

### Example API Usage

```bash
# Get system status
curl http://localhost:3030/status

# Read sensor data
curl http://localhost:3030/sensor

# Control LED
curl -X POST -H 'Content-Type: application/json' \
     -d '{"state": true}' \
     http://localhost:3030/led

# View display content (simulation mode)
curl http://localhost:3030/display
```

## 🏗️ Architecture

### Project Structure

```
rust_pi_zero_example/
├── src/
│   └── main.rs              # Main application logic
├── Cargo.toml               # Dependencies and features
├── deploy.sh                # Deployment script
├── run_simulation.sh        # Simulation runner
├── rust-pi-app.service      # Systemd service file
├── README.md                # This file
├── README_SIMULATION.md     # Simulation guide
├── LICENSE                  # MIT License
└── .gitignore              # Git ignore rules
```

### Feature Flags

- `simulation` (default): Desktop simulation mode
- `hardware`: Raspberry Pi hardware mode with GPIO/SPI

### Dependencies

- **Core**: `tokio`, `warp`, `serde`, `anyhow`, `log`
- **Hardware**: `rppal`, `st7789`, `embedded-graphics`
- **Simulation**: Built-in mock implementations

## 🔌 Hardware Connections

For Raspberry Pi deployment:

| Component | GPIO Pin | Function |
|-----------|----------|----------|
| Status LED | 18 | Visual heartbeat indicator |
| LCD DC | 24 | Display Data/Command control |
| LCD Reset | 25 | Display reset line |
| LCD SPI | SPI0 | Data communication (MOSI, SCLK, CE0) |

### Wiring Diagram

```
Raspberry Pi Zero    ST7789 LCD (240x240)
┌─────────────────┐  ┌─────────────────┐
│ 3.3V      (Pin1)├──┤ VCC             │
│ GND       (Pin6)├──┤ GND             │
│ GPIO24   (Pin18)├──┤ DC              │
│ GPIO25   (Pin22)├──┤ RST             │
│ SPI0_MOSI(Pin19)├──┤ SDA             │
│ SPI0_SCLK(Pin23)├──┤ SCL             │
│ SPI0_CE0 (Pin24)├──┤ CS              │
└─────────────────┘  └─────────────────┘

Status LED
┌─────────────────┐
│ GPIO18   (Pin12)├──[220Ω]──[LED]──[GND]
└─────────────────┘
```

## 🛠️ Development

### Building

```bash
# Development build (simulation)
cargo build

# Release build for Pi hardware
cargo build --release --target arm-unknown-linux-gnueabihf --features hardware --no-default-features

# Check code without building
cargo check
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Cross-compilation Setup

```bash
# Install ARM target
rustup target add arm-unknown-linux-gnueabihf

# Install cross-compilation tools (Ubuntu/Debian)
sudo apt-get install gcc-arm-linux-gnueabihf
```

## 🚀 Deployment

### Automated Deployment

The included `deploy.sh` script handles:
- Cross-compilation for ARM
- Binary transfer via SCP
- Service installation and management
- Connectivity verification

```bash
# Deploy to default Pi
./deploy.sh

# Deploy to specific Pi
./deploy.sh raspberrypi.local pi

# Deploy to custom hostname/user
./deploy.sh 192.168.1.100 myuser
```

### Manual Deployment

```bash
# 1. Build for ARM
cargo build --release --target arm-unknown-linux-gnueabihf --features hardware --no-default-features

# 2. Copy binary to Pi
scp target/arm-unknown-linux-gnueabihf/release/rust_pi_zero_example pi@raspberrypi.local:~/

# 3. SSH to Pi and run
ssh pi@raspberrypi.local
chmod +x rust_pi_zero_example
sudo ./rust_pi_zero_example
```

### Service Installation

```bash
# Copy service file to Pi
scp rust-pi-app.service pi@raspberrypi.local:~/

# On Pi: Install and enable service
sudo cp rust-pi-app.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable rust-pi-app.service
sudo systemctl start rust-pi-app.service

# Check service status
sudo systemctl status rust-pi-app.service
```

## 📊 Monitoring

### Logs

```bash
# View application logs
sudo journalctl -u rust-pi-app.service -f

# View with timestamps
sudo journalctl -u rust-pi-app.service -f --since "1 hour ago"
```

### Web Interface

Access the web API at `http://[pi-ip]:3030/` for real-time monitoring.

## 🎨 Customization

### Display Content

Modify the `update_display_content()` function in `src/main.rs`:

```rust
// Add custom text
display.draw_text("Your Message", 10, 30, Rgb565::WHITE)?;

// Add graphics
Rectangle::new(Point::new(10, 50), Size::new(100, 20))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
    .draw(display)?;
```

### Sensor Integration

Replace simulation functions with real sensor drivers:

```rust
// Replace simulate_temperature_reading() with:
fn read_real_temperature() -> f32 {
    // Your sensor reading code here
}
```

### API Extensions

Add new endpoints in `setup_routes()`:

```rust
let custom_route = warp::path("custom")
    .and(warp::get())
    .and_then(custom_handler);
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation for API changes
- Use `cargo fmt` and `cargo clippy`
- Test both simulation and hardware modes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [RPPAL](https://github.com/golemparts/rppal) - Raspberry Pi GPIO library
- [ST7789](https://github.com/almindor/st7789) - Display driver
- [Embedded Graphics](https://github.com/embedded-graphics/embedded-graphics) - 2D graphics library
- [Warp](https://github.com/seanmonstar/warp) - Web framework

## 📞 Support

- 🐛 [Report Issues](https://github.com/your-username/rust_pi_zero_example/issues)
- 💬 [Discussions](https://github.com/your-username/rust_pi_zero_example/discussions)
- 📧 [Contact](mailto:your-email@example.com)

---

⭐ **Star this repository if you find it helpful!**
