@echo off
title SyncSpace Launcher
color 0B

echo ========================================
echo  SyncSpace Startup Script
echo ========================================
echo.
echo [1/2] Starting Backend Server (Rust)...
start "SyncSpace Backend" cmd /k "cd backend && cargo run --release"
timeout /t 3 /nobreak > nul

echo.
echo [2/2] Starting Frontend Server (Vite)...
start "SyncSpace Frontend" cmd /k "cd frontend && npm run dev"
timeout /t 2 /nobreak > nul

echo.
echo ========================================
echo  Servers sind gestartet!
echo ========================================
echo  Backend:  http://localhost:8080
echo  Frontend: http://localhost:5173
echo ========================================
echo.
echo Oeffne Browser...
start http://localhost:5173

echo.
echo Druecke eine Taste um die Server zu beenden...
pause > nul

echo.
echo Beende Server...
taskkill /FI "WindowTitle eq SyncSpace*" /T /F > nul 2>&1
echo Fertig!
