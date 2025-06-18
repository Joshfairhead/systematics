#!/bin/bash

# SysteMaster Full System Launcher
# Starts API server, frontend server, and optionally CLI

echo "🚀 Starting SysteMaster Full System"
echo "===================================="

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to open new terminal tab/window (macOS)
open_terminal_tab() {
    local cmd="$1"
    local title="$2"
    
    if command_exists osascript; then
        # macOS - open new Terminal tab
        osascript -e "
        tell application \"Terminal\"
            activate
            tell application \"System Events\" to keystroke \"t\" using command down
            delay 0.5
            do script \"cd '$PWD' && echo '🔧 $title' && $cmd\" in front window
        end tell
        "
    else
        echo "⚠️  Auto-opening terminals not supported on this OS"
        echo "   Please manually run: $cmd"
    fi
}

echo "📡 Starting API Server..."
open_terminal_tab "cd api && cargo run --bin server --features server" "API Server (Port 3001)"

echo "⏳ Waiting for API server to start..."
sleep 3

echo "🌐 Starting Frontend Server..."
open_terminal_tab "cd frontend && trunk serve --port 8081" "Frontend Server (Port 8081)"

echo "⏳ Waiting for frontend server to start..."
sleep 3

echo ""
echo "✅ SysteMaster System Starting!"
echo "================================"
echo "🔗 Frontend:  http://localhost:8081"
echo "🔗 API:       http://localhost:3001"
echo "📚 API Docs:  http://localhost:3001/health"
echo ""
echo "Optional: Start CLI in another terminal:"
echo "   cd cli && cargo run"
echo ""
echo "💡 To stop all servers:"
echo "   - Close the terminal tabs, or"
echo "   - Press Ctrl+C in each terminal"
echo ""

# Ask if user wants to start CLI too
read -p "🤔 Start CLI interface as well? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🖥️  Starting CLI..."
    open_terminal_tab "cd cli && cargo run" "CLI Interface"
fi

echo "🎉 All systems launched!" 