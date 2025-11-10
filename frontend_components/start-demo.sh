#!/bin/bash

# SyncSpace Component Library Demo - Startup Script

cd /home/mick/Dokumente/GitHub/syncspace/frontend_components

echo "📦 Installing dependencies..."
npm install

echo ""
echo "🚀 Starting development server..."
echo "Demo will open at: http://localhost:5174"
echo ""

npm run dev
