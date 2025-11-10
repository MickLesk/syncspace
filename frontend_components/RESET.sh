#!/bin/bash
# Complete cleanup and reinstall for frontend_components

cd /home/mick/Dokumente/GitHub/syncspace/frontend_components

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║  SyncSpace Component Library - Full Reset & Reinstall            ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Step 1: Kill running processes
echo "1️⃣  Stopping running dev servers..."
pkill -f "vite" 2>/dev/null || true
pkill -f "npm" 2>/dev/null || true
sleep 2

# Step 2: Deep clean
echo "2️⃣  Cleaning node_modules and caches..."
rm -rf node_modules package-lock.json
rm -rf dist .vite

# Step 3: Install with correct settings
echo "3️⃣  Installing dependencies with Tailwind v4..."
npm install --legacy-peer-deps

echo ""
echo "✅ Reset complete!"
echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║  Next: Start dev server                                          ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""
echo "Run in fish shell:"
echo "  cd /home/mick/Dokumente/GitHub/syncspace/frontend_components"
echo "  npm run dev"
echo ""
echo "Dev server will open on http://localhost:5174"
echo ""
