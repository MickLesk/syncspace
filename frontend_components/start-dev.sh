#!/bin/bash
cd /home/mick/Dokumente/GitHub/syncspace/frontend_components
echo "📦 Installing dependencies..."
npm install --save-dev @sveltejs/vite-plugin-svelte@^4.0.0-next.6 --legacy-peer-deps
echo "🚀 Starting dev server..."
npm run dev
