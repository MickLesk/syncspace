# SyncSpace - Start Both Backend and Frontend
# Startet Backend und Frontend in separaten Terminals

Write-Host "🚀 Starting SyncSpace..." -ForegroundColor Cyan
Write-Host ""

$rootPath = $PSScriptRoot

# Start Backend in new terminal
Write-Host "📦 Starting Backend..." -ForegroundColor Green
Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd '$rootPath\backend'; cargo run --release"

# Wait 3 seconds for backend to start
Start-Sleep -Seconds 3

# Start Frontend in new terminal
Write-Host "🌐 Starting Frontend..." -ForegroundColor Blue
Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd '$rootPath\frontend'; npm run dev"

Write-Host ""
Write-Host "✅ Both services starting in separate windows" -ForegroundColor Green
Write-Host "📝 Backend: http://localhost:8080" -ForegroundColor Cyan
Write-Host "📝 Frontend: http://localhost:5173 (check frontend terminal for exact port)" -ForegroundColor Cyan
Write-Host ""
Write-Host "💡 To stop: Close the terminal windows or press Ctrl+C in each" -ForegroundColor Yellow
