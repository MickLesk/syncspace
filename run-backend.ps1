# SyncSpace Backend Auto-Restart Script
# Startet das Backend und startet es automatisch neu bei Absturz

Write-Host "🚀 Starting SyncSpace Backend with auto-restart..." -ForegroundColor Cyan
Write-Host "Press Ctrl+C to stop" -ForegroundColor Yellow
Write-Host ""

$backendPath = Join-Path $PSScriptRoot "backend"

while ($true) {
    try {
        Push-Location $backendPath
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] 🔄 Starting backend..." -ForegroundColor Green
        
        # Start backend and capture process
        $process = Start-Process -FilePath "cargo" -ArgumentList "run", "--release" `
            -PassThru -NoNewWindow -Wait
        
        if ($process.ExitCode -eq 0) {
            Write-Host "[$(Get-Date -Format 'HH:mm:ss')] ✅ Backend stopped cleanly" -ForegroundColor Green
            break
        }
        else {
            Write-Host "[$(Get-Date -Format 'HH:mm:ss')] ⚠️  Backend crashed with exit code: $($process.ExitCode)" -ForegroundColor Yellow
            Write-Host "[$(Get-Date -Format 'HH:mm:ss')] ⏳ Restarting in 2 seconds..." -ForegroundColor Yellow
            Start-Sleep -Seconds 2
        }
    }
    catch {
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] ❌ Error: $_" -ForegroundColor Red
        Write-Host "[$(Get-Date -Format 'HH:mm:ss')] ⏳ Restarting in 3 seconds..." -ForegroundColor Yellow
        Start-Sleep -Seconds 3
    }
    finally {
        Pop-Location
    }
}
