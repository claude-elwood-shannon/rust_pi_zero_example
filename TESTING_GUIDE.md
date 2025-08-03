# 🚀 Complete Testing Guide for Rust Pi Zero Application

## 📋 Prerequisites

### 1. Install Rust
```bash
# On Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# On Windows
# Download from: https://rustup.rs/
```

### 2. Verify Installation
```bash
rustc --version
cargo --version
```

## 📥 Download the Project

### Option A: Clone from GitHub
```bash
git clone https://github.com/claude-elwood-shannon/rust_pi_zero_example.git
cd rust_pi_zero_example
```

### Option B: Download ZIP
1. Go to: https://github.com/claude-elwood-shannon/rust_pi_zero_example
2. Click "Code" → "Download ZIP"
3. Extract and enter the directory

## 🏃‍♂️ Run the Application

### Method 1: Automated Script (Recommended)
```bash
# Grant execution permissions
chmod +x run_simulation.sh

# Execute
./run_simulation.sh
```

### Method 2: Direct Command
```bash
cargo run --target x86_64-unknown-linux-gnu
```

### Method 3: Build Only
```bash
cargo build --target x86_64-unknown-linux-gnu
```

## 🖥️ What You'll See

### Simulated LCD Display
```
╔════════════════════════════════════════════════╗
║ Hello World!                                   ║
║                                                ║
║ Pi Zero Monitor                                ║
║ Temp: 25.3C                                    ║
║                                                ║
║ Humidity: 65.2%                                ║
║                                                ║
║                                                ║
║ Uptime: 120s                                   ║
║ LED                                            ║
║                                                ║
║                                                ║
║                                                ║
╚════════════════════════════════════════════════╝
```

### Console Output
```
📊 Status: LED=OFF, Temp=25.3°C
```

## 🌐 Test the Web API

### 1. Verify Server is Running
```bash
curl http://localhost:3030/
# Response: "Raspberry Pi Zero Rust Server is running!"
```

### 2. Get Complete System Status
```bash
curl http://localhost:3030/status
```
**JSON Response:**
```json
{
  "uptime_seconds": 139,
  "led_status": true,
  "last_sensor_reading": {
    "temperature": 34.835,
    "humidity": 51.440002,
    "timestamp": 1754179702
  },
  "display_content": "Hello World!..."
}
```

### 3. Read Sensors
```bash
curl http://localhost:3030/sensor
```
**Response:**
```json
{
  "temperature": 23.36,
  "humidity": 71.56,
  "timestamp": 1754179727
}
```

### 4. View LCD Display Content
```bash
curl http://localhost:3030/display
```
**Response:**
```json
{
  "display_content": "Hello World!...",
  "mode": "simulation"
}
```

### 5. Control LED
```bash
# Turn LED ON
curl -X POST -H 'Content-Type: application/json' -d '{"state": true}' http://localhost:3030/led

# Turn LED OFF
curl -X POST -H 'Content-Type: application/json' -d '{"state": false}' http://localhost:3030/led
```
**Response:**
```json
{
  "led_state": true,
  "success": true
}
```

## 🔧 Useful Commands

### Stop the Application

**Method 1: Automated Stop Script (Recommended)**
```bash
# Use the dedicated stop script
./stop_simulation.sh
```

**Method 2: Graceful shutdown**
```bash
# In the terminal where the application is running:
Ctrl+C
```

**Method 3: From another terminal**
```bash
# Kill by process name
pkill -f rust_pi_zero_example

# Or kill by port (if you know it's using port 3030)
sudo lsof -ti:3030 | xargs kill -9
```

**Method 4: Find and kill specific process**
```bash
# Find the process ID
ps aux | grep rust_pi_zero_example

# Kill using the PID (replace XXXX with actual PID)
kill XXXX

# Force kill if needed
kill -9 XXXX
```

### View Detailed Logs
```bash
RUST_LOG=debug cargo run --target x86_64-unknown-linux-gnu
```

### Cross-compile for Raspberry Pi
```bash
# Install ARM target
rustup target add arm-unknown-linux-gnueabihf

# Build for Pi Zero
cargo build --target arm-unknown-linux-gnueabihf --features hardware --no-default-features
```

## 🎯 Features You Can Observe

### ✅ Virtual LCD Display
- **"Hello World!"** prominently displayed on first line
- Simulated temperature and humidity readings
- Runtime uptime counter
- High temperature alerts (>30°C)
- Updates every 2 seconds

### ✅ Simulated Sensors
- Temperature: 18°C - 35°C range
- Humidity: 30% - 80% range
- Realistic and variable values

### ✅ Virtual LED
- ON/OFF states
- API control
- Visual indicator on display

### ✅ Complete REST API
- 5 functional endpoints
- Structured JSON responses
- CORS enabled
- Port 3030

## 🐛 Troubleshooting

### Error: "Address already in use (os error 98)"
This means port 3030 is already in use by another instance of the application.

**Solution 1: Kill existing process**
```bash
# Find and kill the process using port 3030
sudo lsof -ti:3030 | xargs kill -9

# Or use pkill to find the specific process
pkill -f rust_pi_zero_example

# Then run the application again
./run_simulation.sh
```

**Solution 2: Use a different port**
```bash
# Edit src/main.rs and change the port number in the .serve() line
# For example, change from 3030 to 3031
```

### Error: "failed to select a version for st7789"
```bash
# Already fixed in the code, but if it appears:
cargo update
```

### Error: "linker not found"
```bash
# On Ubuntu/Debian:
sudo apt install build-essential

# On macOS:
xcode-select --install
```

### Permission Issues on Linux
```bash
chmod +x run_simulation.sh
```

### Unused Import Warnings
These warnings are harmless and don't affect functionality:
```bash
# To fix the warnings, run:
cargo fix --bin "rust_pi_zero_example"
```

## 📱 Test from Browser

Open your browser and visit:
- http://localhost:3030/ - Welcome message
- http://localhost:3030/status - Complete JSON status
- http://localhost:3030/sensor - Sensor data
- http://localhost:3030/display - Display content

## 🎉 Success!

If you see the **"Hello World!"** message on the ASCII display and can access the API endpoints, the application is working perfectly!

The application simulates a complete IoT monitor for Raspberry Pi Zero featuring:
- LCD display with "Hello World!" message
- Temperature and humidity sensors
- LED control
- Web API for remote monitoring
- Real-time logging

---
**Repository:** https://github.com/claude-elwood-shannon/rust_pi_zero_example
**Complete Documentation:** README.md and README_SIMULATION.md
