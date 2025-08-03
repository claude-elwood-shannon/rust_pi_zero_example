#!/bin/bash

echo "🚀 Starting Raspberry Pi Zero Rust Application in Simulation Mode"
echo "=================================================="

# Build and run in simulation mode (default)
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
