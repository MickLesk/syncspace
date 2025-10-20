@echo off
echo ========================================
echo  SyncSpace Startup Script
echo ========================================
echo.
echo Starting Backend Server...
start "SyncSpace Backend" cmd /k "cd backend && cargo run"
timeout /t 3 /nobreak > nul

echo.
echo Starting Frontend Server...
start "SyncSpace Frontend" cmd /k "cd frontend && python -m http.server 8000"
timeout /t 2 /nobreak > nul

echo.
echo ========================================
echo  Servers sind gestartet!
echo ========================================
echo  Backend:  http://localhost:8080
echo  Frontend: http://localhost:8000
echo ========================================
echo.
echo Öffne Browser...
start http://localhost:8000

echo.
echo Drücke eine Taste um die Server zu beenden...
pause > nul

echo.
echo Beende Server...
taskkill /FI "WindowTitle eq SyncSpace*" /T /F > nul 2>&1
echo Fertig!
