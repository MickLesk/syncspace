# SyncSpace API Test Script
# Tests all authentication and file operation endpoints

$BASE_URL = "http://localhost:8080/api"
$TOKEN = ""

Write-Host "SyncSpace API Test Suite" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Register new user
Write-Host "1️⃣ Testing User Registration..." -ForegroundColor Yellow
try {
    $registerData = @{
        username = "testuser_$(Get-Random)"
        password = "TestPassword123!"
    } | ConvertTo-Json

    $response = Invoke-RestMethod -Uri "$BASE_URL/auth/register" `
        -Method Post `
        -ContentType "application/json" `
        -Body $registerData

    $TOKEN = $response.token
    Write-Host "   ✅ Registration successful!" -ForegroundColor Green
    Write-Host "   Username: $($response.user.username)" -ForegroundColor Gray
    Write-Host "   Token: $($TOKEN.Substring(0,20))..." -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Registration failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 2: Login with admin user
Write-Host "`n2️⃣ Testing Login..." -ForegroundColor Yellow
try {
    $loginData = @{
        username = "admin"
        password = "admin"
    } | ConvertTo-Json

    $response = Invoke-RestMethod -Uri "$BASE_URL/auth/login" `
        -Method Post `
        -ContentType "application/json" `
        -Body $loginData

    $TOKEN = $response.token
    Write-Host "   ✅ Login successful!" -ForegroundColor Green
    Write-Host "   Token: $($TOKEN.Substring(0,20))..." -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Login failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 3: Get current user info
Write-Host "`n3️⃣ Testing Get User Info..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/auth/me" `
        -Method Get `
        -Headers $headers

    Write-Host "   ✅ User info retrieved!" -ForegroundColor Green
    Write-Host "   Username: $($response.username)" -ForegroundColor Gray
    Write-Host "   2FA Enabled: $($response.totp_enabled)" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Get user info failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 4: List files
Write-Host "`n4️⃣ Testing List Files..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/files/" `
        -Method Get `
        -Headers $headers

    Write-Host "   ✅ Files listed!" -ForegroundColor Green
    Write-Host "   Files found: $($response.Count)" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ List files failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 5: Upload file
Write-Host "`n5️⃣ Testing File Upload..." -ForegroundColor Yellow
try {
    $testContent = "SyncSpace Test File - $(Get-Date)"
    $testFile = "test_$(Get-Random).txt"
    
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/upload/$testFile" `
        -Method Post `
        -Headers $headers `
        -Body ([System.Text.Encoding]::UTF8.GetBytes($testContent))

    Write-Host "   ✅ File uploaded!" -ForegroundColor Green
    Write-Host "   Filename: $testFile" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Upload failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 6: Get statistics
Write-Host "`n6️⃣ Testing Statistics..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/stats" `
        -Method Get `
        -Headers $headers

    Write-Host "   ✅ Statistics retrieved!" -ForegroundColor Green
    Write-Host "   File count: $($response.file_count)" -ForegroundColor Gray
    Write-Host "   Total size: $($response.total_size) bytes" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Statistics failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 7: Search files
Write-Host "`n7️⃣ Testing Search..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/search?q=test" `
        -Method Get `
        -Headers $headers

    Write-Host "   ✅ Search completed!" -ForegroundColor Green
    Write-Host "   Results: $($response.Count)" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Search failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 8: Create directory
Write-Host "`n8️⃣ Testing Create Directory..." -ForegroundColor Yellow
try {
    $testDir = "testdir_$(Get-Random)"
    
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/dirs/$testDir" `
        -Method Post `
        -Headers $headers

    Write-Host "   ✅ Directory created!" -ForegroundColor Green
    Write-Host "   Directory: $testDir" -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ Create directory failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 9: Setup 2FA
Write-Host "`n9️⃣ Testing 2FA Setup..." -ForegroundColor Yellow
try {
    $headers = @{
        "Authorization" = "Bearer $TOKEN"
    }

    $response = Invoke-RestMethod -Uri "$BASE_URL/auth/2fa/setup" `
        -Method Post `
        -Headers $headers `
        -ContentType "application/json"

    Write-Host "   ✅ 2FA setup initiated!" -ForegroundColor Green
    Write-Host "   Secret: $($response.secret)" -ForegroundColor Gray
    Write-Host "   QR URL: $($response.qr_url.Substring(0,40))..." -ForegroundColor Gray
}
catch {
    Write-Host "   ❌ 2FA setup failed: $_" -ForegroundColor Red
}

Start-Sleep -Seconds 1

# Test 10: Test without auth (should fail)
Write-Host "`n🔟 Testing Unauthorized Access..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$BASE_URL/files/" `
        -Method Get `
        -ErrorAction Stop

    Write-Host "   ❌ Security issue: Unauthorized access allowed!" -ForegroundColor Red
}
catch {
    Write-Host "   ✅ Access correctly denied!" -ForegroundColor Green
}

Write-Host ""
Write-Host "=============================" -ForegroundColor Cyan
Write-Host "Test Suite Complete!" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host ""
