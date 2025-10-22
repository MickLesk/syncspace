# SyncSpace Startup Script
# Starts Backend (Rust) and Frontend (Vite) concurrently

Write-Host "🚀 Starting SyncSpace..." -ForegroundColor Cyan
Write-Host ""

# Check if backend binary exists, build if not
if (-not (Test-Path ".\backend\target\release\syncbackend.exe")) {
    Write-Host "📦 Building backend (first time)..." -ForegroundColor Yellow
    Push-Location backend
    cargo build --release
    Pop-Location
}

# Start Backend in background
Write-Host "🔧 Starting Backend (http://localhost:8080)..." -ForegroundColor Green
$backend = Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd backend; cargo run --release" -PassThru -WindowStyle Normal

# Wait a moment for backend to start
Start-Sleep -Seconds 2

# Start Frontend in background  
Write-Host "🎨 Starting Frontend (http://localhost:5173)..." -ForegroundColor Green
$frontend = Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd frontend; npm run dev" -PassThru -WindowStyle Normal

Write-Host ""
Write-Host "✅ SyncSpace is starting!" -ForegroundColor Cyan
Write-Host "   Backend:  http://localhost:8080" -ForegroundColor White
Write-Host "   Frontend: http://localhost:5173" -ForegroundColor White
Write-Host ""
Write-Host "💡 Press Ctrl+C in each window to stop the servers" -ForegroundColor Yellow
Write-Host ""

# Keep script running to show status
Write-Host "📊 Monitoring processes... (Press Ctrl+C to exit this monitor)" -ForegroundColor Cyan
while ($true) {
    if ($backend.HasExited) {
        Write-Host "⚠️  Backend has stopped!" -ForegroundColor Red
        break
    }
    if ($frontend.HasExited) {
        Write-Host "⚠️  Frontend has stopped!" -ForegroundColor Red
        break
    }
    Start-Sleep -Seconds 5
}
