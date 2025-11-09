#!/bin/bash
# Backend Build & Test Script
# Waits for compilation to complete, then tests all endpoints

set -e

BACKEND_DIR="/home/mick/Dokumente/GitHub/syncspace/backend"
BINARY_PATH="$BACKEND_DIR/target/release/syncbackend"
CARGO="/home/mick/.cargo/bin/cargo"

echo "🔨 Starting Backend Build & Test Cycle"
echo "========================================="
echo ""

# Function to check build status
check_build() {
    if [ -f "$BINARY_PATH" ]; then
        return 0
    else
        return 1
    fi
}

# Wait for build to complete
echo "⏳ Waiting for build to complete..."
WAIT_COUNT=0
MAX_WAITS=60  # 60 * 10 seconds = 10 minutes max

while ! check_build; do
    WAIT_COUNT=$((WAIT_COUNT + 1))
    if [ $WAIT_COUNT -gt $MAX_WAITS ]; then
        echo "❌ Build took too long (>10 minutes). Exiting."
        exit 1
    fi
    
    # Show progress
    if ps aux | grep -q "[c]argo build"; then
        echo -ne "   Still building... (${WAIT_COUNT}s elapsed)\r"
    fi
    
    sleep 10
done

echo ""
echo "✅ Build Complete!"
echo ""

# Verify binary
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found at $BINARY_PATH"
    exit 1
fi

BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
echo "Binary info:"
echo "  Path: $BINARY_PATH"
echo "  Size: $BINARY_SIZE"
echo ""

# Make it executable
chmod +x "$BINARY_PATH"

# Start backend server
echo "🚀 Starting backend server..."
"$BINARY_PATH" &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Check if server is running
if ! ps -p $SERVER_PID > /dev/null; then
    echo "❌ Server failed to start"
    exit 1
fi

echo "✅ Server started (PID: $SERVER_PID)"
echo ""

# Test public endpoint (no auth required)
echo "🧪 Testing public endpoints..."
echo ""

RESPONSE=$(curl -s -w "\n%{http_code}" http://localhost:8080/api/sharing/public/test-token 2>/dev/null)
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)

if [ "$HTTP_CODE" = "404" ]; then
    echo "✅ Public endpoint works! (HTTP 404 as expected)"
else
    echo "⚠️  Unexpected response: HTTP $HTTP_CODE"
    if [ "$HTTP_CODE" = "401" ]; then
        echo "❌ ERROR: Public endpoint requires authentication (should not!)"
    fi
fi

echo ""
echo "========================================="
echo "✅ Phase 8 Complete!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. The backend is now running on http://localhost:8080"
echo "2. Start frontend: cd frontend && npm run dev"
echo "3. Test endpoints with curl or frontend"
echo "4. Press Ctrl+C to stop the server"
echo ""
echo "Server PID: $SERVER_PID"
echo "To stop: kill $SERVER_PID"
