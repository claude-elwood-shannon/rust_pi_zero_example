#!/bin/bash

echo "🛑 Stopping Raspberry Pi Zero Rust Application"
echo "=============================================="

# Method 1: Try to kill by process name
echo "🔍 Looking for rust_pi_zero_example processes..."
if pkill -f rust_pi_zero_example 2>/dev/null; then
    echo "✅ Terminated rust_pi_zero_example process"
    sleep 2
else
    echo "ℹ️  No rust_pi_zero_example process found by name"
fi

# Method 2: Check and kill by port
echo "🔍 Checking port 3030..."
if lsof -Pi :3030 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "🔧 Found process using port 3030, terminating..."
    sudo lsof -ti:3030 | xargs kill -9 2>/dev/null || true
    sleep 1
    
    # Verify port is free
    if lsof -Pi :3030 -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo "⚠️  Port 3030 still in use. Manual intervention may be required."
        echo "   Try: sudo lsof -ti:3030 | xargs kill -9"
    else
        echo "✅ Port 3030 is now free"
    fi
else
    echo "✅ Port 3030 is already free"
fi

# Method 3: Show any remaining Rust processes
echo "🔍 Checking for any remaining Rust processes..."
RUST_PROCS=$(ps aux | grep -E "(cargo|rust)" | grep -v grep | grep -v stop_simulation || true)
if [ -n "$RUST_PROCS" ]; then
    echo "ℹ️  Found other Rust processes:"
    echo "$RUST_PROCS"
    echo ""
    echo "💡 If these are related to your application, you can kill them with:"
    echo "   pkill -f cargo"
else
    echo "✅ No Rust processes found"
fi

echo ""
echo "🎉 Simulation stop procedure completed!"
echo "📝 You can now run ./run_simulation.sh to start again"
