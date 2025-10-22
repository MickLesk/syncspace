# SyncSpace API Test Script
$BASE_URL = "http://localhost:8080/api"
$TOKEN = ""

Write-Host "================================" -ForegroundColor Cyan
Write-Host " SyncSpace API Test Suite" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Register
Write-Host "[1/10] Testing User Registration..." -ForegroundColor Yellow
try {
    $body = @{ username = "test$(Get-Random)"; password = "Test123!" } | ConvertTo-Json
    $r = Invoke-RestMethod -Uri "$BASE_URL/auth/register" -Method Post -ContentType "application/json" -Body $body
    $TOKEN = $r.token
    Write-Host "  OK - User: $($r.user.username)" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 2: Login
Write-Host "[2/10] Testing Login..." -ForegroundColor Yellow
try {
    $body = @{ username = "admin"; password = "admin" } | ConvertTo-Json
    $r = Invoke-RestMethod -Uri "$BASE_URL/auth/login" -Method Post -ContentType "application/json" -Body $body
    $TOKEN = $r.token
    Write-Host "  OK - Token received" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 3: Get User Info
Write-Host "[3/10] Testing Get User Info..." -ForegroundColor Yellow
try {
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $r = Invoke-RestMethod -Uri "$BASE_URL/auth/me" -Method Get -Headers $h
    Write-Host "  OK - User: $($r.username), 2FA: $($r.totp_enabled)" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 4: List Files
Write-Host "[4/10] Testing List Files..." -ForegroundColor Yellow
try {
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $r = Invoke-RestMethod -Uri "$BASE_URL/files/" -Method Get -Headers $h
    Write-Host "  OK - Files: $($r.Count)" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 5: Upload File
Write-Host "[5/10] Testing File Upload..." -ForegroundColor Yellow
try {
    $file = "test_$(Get-Random).txt"
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $body = [System.Text.Encoding]::UTF8.GetBytes("Test content")
    Invoke-RestMethod -Uri "$BASE_URL/upload/$file" -Method Post -Headers $h -Body $body | Out-Null
    Write-Host "  OK - File: $file" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 6: Statistics
Write-Host "[6/10] Testing Statistics..." -ForegroundColor Yellow
try {
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $r = Invoke-RestMethod -Uri "$BASE_URL/stats" -Method Get -Headers $h
    Write-Host "  OK - Files: $($r.file_count), Size: $($r.total_size) bytes" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 7: Search
Write-Host "[7/10] Testing Search..." -ForegroundColor Yellow
try {
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $r = Invoke-RestMethod -Uri "$BASE_URL/search?q=test" -Method Get -Headers $h
    Write-Host "  OK - Results: $($r.Count)" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 8: Create Directory
Write-Host "[8/10] Testing Create Directory..." -ForegroundColor Yellow
try {
    $dir = "testdir_$(Get-Random)"
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    Invoke-RestMethod -Uri "$BASE_URL/dirs/$dir" -Method Post -Headers $h | Out-Null
    Write-Host "  OK - Dir: $dir" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 9: 2FA Setup
Write-Host "[9/10] Testing 2FA Setup..." -ForegroundColor Yellow
try {
    $h = @{ "Authorization" = "Bearer $TOKEN" }
    $r = Invoke-RestMethod -Uri "$BASE_URL/auth/2fa/setup" -Method Post -Headers $h -ContentType "application/json"
    Write-Host "  OK - Secret: $($r.secret)" -ForegroundColor Green
}
catch {
    Write-Host "  FAIL - $_" -ForegroundColor Red
}

Start-Sleep -Milliseconds 500

# Test 10: Unauthorized Access
Write-Host "[10/10] Testing Unauthorized Access..." -ForegroundColor Yellow
try {
    Invoke-RestMethod -Uri "$BASE_URL/files/" -Method Get -ErrorAction Stop | Out-Null
    Write-Host "  FAIL - Unauthorized access allowed!" -ForegroundColor Red
}
catch {
    Write-Host "  OK - Access denied correctly" -ForegroundColor Green
}

Write-Host ""
Write-Host "================================" -ForegroundColor Cyan
Write-Host " Tests Complete!" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
