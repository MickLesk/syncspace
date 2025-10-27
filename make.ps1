# SyncSpace Development Commands (PowerShell)
# Windows-compatible build automation

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

function Show-Help {
    Write-Host "SyncSpace Development Commands:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  .\make.ps1 dev          - Start backend + frontend in parallel" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 backend      - Start Rust backend (release mode)" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 frontend     - Start Svelte frontend (dev mode)" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 test         - Run all tests" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 lint         - Run linters" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 format       - Format code" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 clean        - Clean build artifacts" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 build        - Build production binaries" -ForegroundColor Yellow
    Write-Host "  .\make.ps1 install      - Install dependencies" -ForegroundColor Yellow
    Write-Host ""
}

function Start-Dev {
    Write-Host "🚀 Starting SyncSpace in development mode..." -ForegroundColor Green
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd backend; cargo run --release"
    Start-Sleep -Seconds 2
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd frontend; npm run dev"
}

function Start-Backend {
    Write-Host "🦀 Starting Rust backend..." -ForegroundColor Green
    Set-Location backend
    cargo run --release
}

function Start-Frontend {
    Write-Host "⚡ Starting Svelte frontend..." -ForegroundColor Green
    Set-Location frontend
    npm run dev
}

function Run-Tests {
    Write-Host "🧪 Running tests..." -ForegroundColor Green
    Write-Host "Backend tests..." -ForegroundColor Yellow
    Set-Location backend
    cargo test
    Set-Location ..
    Write-Host "Frontend tests..." -ForegroundColor Yellow
    Set-Location frontend
    npm run test
    Set-Location ..
}

function Run-Lint {
    Write-Host "🔍 Running linters..." -ForegroundColor Green
    Set-Location backend
    cargo clippy -- -D warnings
    Set-Location ..
    Set-Location frontend
    npm run lint
    Set-Location ..
}

function Run-Format {
    Write-Host "🎨 Formatting code..." -ForegroundColor Green
    Set-Location backend
    cargo fmt
    Set-Location ..
    Set-Location frontend
    npm run format
    Set-Location ..
}

function Clean-Build {
    Write-Host "🧹 Cleaning build artifacts..." -ForegroundColor Green
    Set-Location backend
    cargo clean
    Set-Location ..
    Set-Location frontend
    Remove-Item -Recurse -Force dist, node_modules\.vite -ErrorAction SilentlyContinue
    Set-Location ..
}

function Build-Production {
    Write-Host "📦 Building production binaries..." -ForegroundColor Green
    Set-Location backend
    cargo build --release
    Set-Location ..
    Set-Location frontend
    npm run build
    Set-Location ..
    Write-Host "✅ Build complete!" -ForegroundColor Green
    Write-Host "Backend binary: backend\target\release\syncbackend.exe" -ForegroundColor Cyan
    Write-Host "Frontend dist: frontend\dist\" -ForegroundColor Cyan
}

function Install-Dependencies {
    Write-Host "📥 Installing dependencies..." -ForegroundColor Green
    Set-Location backend
    cargo build
    Set-Location ..
    Set-Location frontend
    npm install
    Set-Location ..
    Write-Host "✅ Dependencies installed!" -ForegroundColor Green
}

# Command dispatcher
switch ($Command) {
    "help" { Show-Help }
    "dev" { Start-Dev }
    "backend" { Start-Backend }
    "frontend" { Start-Frontend }
    "test" { Run-Tests }
    "lint" { Run-Lint }
    "format" { Run-Format }
    "clean" { Clean-Build }
    "build" { Build-Production }
    "install" { Install-Dependencies }
    default {
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Show-Help
    }
}
