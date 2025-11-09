#!/bin/bash

# SyncSpace Backend Testing Script
# Runs immediately after binary compilation completes

set -e

BACKEND_DIR="/home/mick/Dokumente/GitHub/syncspace/backend"
BINARY="$BACKEND_DIR/target/release/syncbackend"
API_BASE="http://localhost:8080/api"

echo "🧪 ========================================="
echo "   SyncSpace Backend Test Suite"
echo "=========================================="

# 1. Check binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found at $BINARY"
    exit 1
fi

echo "✅ Binary exists: $(ls -lh $BINARY | awk '{print $5}')"

# 2. Start backend
echo ""
echo "🚀 Starting backend..."
cd "$BACKEND_DIR"
timeout 10 "$BINARY" >/tmp/backend.log 2>&1 &
BACKEND_PID=$!
sleep 2

# 3. Check if backend is running
if ! ps -p $BACKEND_PID >/dev/null 2>&1; then
    echo "❌ Backend failed to start"
    cat /tmp/backend.log | head -20
    exit 1
fi

echo "✅ Backend started (PID: $BACKEND_PID)"

# 4. Check database auto-migration
echo ""
echo "🗄️  Checking database..."
sleep 1
if [ -f "$BACKEND_DIR/data/syncspace.db" ]; then
    TABLES=$(sqlite3 "$BACKEND_DIR/data/syncspace.db" ".tables" 2>/dev/null | wc -w)
    echo "✅ Database initialized with $TABLES tables"
else
    echo "⚠️  Database file not created yet"
fi

# 5. Test public endpoint (no auth required)
echo ""
echo "🌐 Testing public endpoint..."
RESPONSE=$(curl -s -w "\n%{http_code}" "$API_BASE/sharing/public/test-token-12345" 2>/dev/null || echo -e "\n000")
HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "404" ]; then
    echo "✅ Public endpoint accessible (404 is expected for invalid token)"
else
    echo "⚠️  HTTP $HTTP_CODE returned"
fi

# 6. Test authentication endpoint
echo ""
echo "🔐 Testing auth endpoint..."
RESPONSE=$(curl -s -X POST "$API_BASE/auth/login" \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin"}' \
    -w "\n%{http_code}" 2>/dev/null || echo -e "\n000")
HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "401" ]; then
    echo "✅ Auth endpoint responding (HTTP $HTTP_CODE)"
else
    echo "⚠️  Auth endpoint returned HTTP $HTTP_CODE"
fi

# 7. Test protected endpoint (should require auth)
echo ""
echo "🔒 Testing protected endpoint..."
RESPONSE=$(curl -s -w "%{http_code}" "$API_BASE/files" 2>/dev/null || echo "000")
HTTP_CODE=$(echo "$RESPONSE" | tail -3)

if [ "$HTTP_CODE" = "401" ]; then
    echo "✅ Protected endpoint requires authentication (401 Unauthorized)"
elif [ "$HTTP_CODE" = "200" ]; then
    echo "⚠️  Protected endpoint returned 200 (check if this should need auth)"
else
    echo "⚠️  Protected endpoint returned HTTP $HTTP_CODE"
fi

# 8. Cleanup
echo ""
echo "🧹 Cleaning up..."
kill $BACKEND_PID 2>/dev/null || true

echo ""
echo "✅ ========================================="
echo "   Backend Tests Complete!"
echo "=========================================="
echo ""
echo "💡 Next steps:"
echo "   1. Start backend: cd backend && ./target/release/syncbackend"
echo "   2. Start frontend: cd frontend && npm run dev"
echo "   3. Open http://localhost:5173 in browser"
echo ""
