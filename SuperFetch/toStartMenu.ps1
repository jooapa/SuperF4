$scriptPath = (Get-Item -Path ".\").FullName
$targetFile = Join-Path $scriptPath "Superfetch.dist/Superfetch.exe"
$startInFolder = Join-Path $scriptPath "Superfetch.dist"
$shortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\Superfetch.lnk"
$shortcutKey = "None"
$windowStyle = 1
$comment = ""

# Create a WScript Shell object
$wshShell = New-Object -ComObject WScript.Shell

# Create the shortcut object
$shortcut = $wshShell.CreateShortcut($shortcutPath)

# Set the properties of the shortcut
$shortcut.TargetPath = $targetFile
$shortcut.WorkingDirectory = $startInFolder
$shortcut.Hotkey = $shortcutKey
$shortcut.WindowStyle = $windowStyle
$shortcut.Description = $comment

# Save the shortcut
$shortcut.Save()

Write-Host "Shortcut created successfully at: $shortcutPath"
