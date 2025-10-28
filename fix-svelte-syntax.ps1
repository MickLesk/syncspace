# Fix Svelte 4 to Svelte 5 syntax across frontend
$files = Get-ChildItem -Path "frontend/src" -Filter "*.svelte" -Recurse -Exclude "*.old.svelte","*.backup.svelte" | Where-Object { $_.Name -notmatch "bak" }

foreach ($file in $files) {
    Write-Host "Processing: $($file.FullName)"
    $content = Get-Content $file.FullName -Raw
    
    # Replace on:click with onclick
    $content = $content -replace '\son:click([=|])', ' onclick$1'
    
    # Replace on:change with onchange  
    $content = $content -replace '\son:change([=|])', ' onchange$1'
    
    # Replace on:input with oninput
    $content = $content -replace '\son:input([=|])', ' oninput$1'
    
    # Replace on:submit with onsubmit
    $content = $content -replace '\son:submit([=|])', ' onsubmit$1'
    
    # Replace on:keydown with onkeydown
    $content = $content -replace '\son:keydown([=|])', ' onkeydown$1'
    
    # Replace on:keyup with onkeyup
    $content = $content -replace '\son:keyup([=|])', ' onkeyup$1'
    
    # Replace on:focus with onfocus
    $content = $content -replace '\son:focus([=|])', ' onfocus$1'
    
    # Replace on:blur with onblur
    $content = $content -replace '\son:blur([=|])', ' onblur$1'
    
    Set-Content $file.FullName $content -NoNewline
}

Write-Host "Done! Fixed all Svelte files."
