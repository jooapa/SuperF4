$shortcutPath = Join-Path $env:USERPROFILE "Start Menu\Programs\Startup\Superfetch.lnk"
$targetPath = Join-Path $PSScriptRoot "dist\Superfetch.exe"
$workingDirectory = $PSScriptRoot

$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut($shortcutPath)
$Shortcut.TargetPath = $targetPath
$Shortcut.WorkingDirectory = $workingDirectory  # Set the working directory for the shortcut
$Shortcut.Save()