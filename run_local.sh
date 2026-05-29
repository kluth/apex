#!/bin/bash
# APEX Local Launcher
# Führe dieses Skript aus, um die Live-Physik lokal zu sehen.

echo "🚀 APEX Local Environment Startup..."

# 1. Kill existing processes
pkill -f http.server
pkill -f simulate_human

# 2. Start HTTP Server for the 3D Viewer
python3 -m http.server 8000 &
SERVER_PID=$!

# 3. Start the Rust XPBD Simulation
echo "🏃 Starting Physics Engine (XPBD)..."
cargo run --example simulate_human &
SIM_PID=$!

echo "✅ System Ready!"
echo "👉 Open your browser at: http://localhost:8000/examples/viewer.html"
echo "Press Ctrl+C to stop everything."

# Wait for Ctrl+C
trap "kill $SERVER_PID $SIM_PID; exit" INT
wait
