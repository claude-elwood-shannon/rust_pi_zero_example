#!/bin/bash

echo "🚀 Starting Raspberry Pi Zero Rust Application in Simulation Mode"
echo "=================================================="

# Check if port 3030 is already in use
if lsof -Pi :3030 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "⚠️  Port 3030 is already in use!"
    echo "🔧 Attempting to stop existing process..."
    
    # Try to kill the existing process
    pkill -f rust_pi_zero_example 2>/dev/null || true
    
    # Wait a moment for the process to terminate
    sleep 2
    
    # Check again if port is still in use
    if lsof -Pi :3030 -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo "❌ Could not free port 3030. Please run manually:"
        echo "   sudo lsof -ti:3030 | xargs kill -9"
        echo "   Then run this script again."
        exit 1
    else
        echo "✅ Port 3030 is now available!"
    fi
fi

# Build and run the application in simulation mode
echo "🔨 Building and starting application..."
cargo run --target x86_64-unknown-linux-gnu

echo ""
echo "📝 To test the API endpoints, try:"
echo "  curl http://localhost:3030/status"
echo "  curl http://localhost:3030/sensor"
echo "  curl http://localhost:3030/display"
echo "  curl -X POST -H 'Content-Type: application/json' -d '{\"state\": true}' http://localhost:3030/led"
echo ""
echo "🔧 To run in hardware mode instead:"
echo "  cargo run --features hardware --no-default-features"
