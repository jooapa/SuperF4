@echo off
:: BatchGotAdmin (Run as Admin code starts)
REM --> Check for permissions
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"

REM --> If error flag set, we do not have admin.
if '%errorlevel%' NEQ '0' (
    echo Requesting administrative privileges...
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    set params = %*:"=""
    echo UAC.ShellExecute "cmd.exe", "/c %~s0 %params%", "", "runas", 1 >> "%temp%\getadmin.vbs"
    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    pushd "%CD%"
    CD /D "%~dp0"

:: Run as Admin code ends

:: Set the target folder for the files
set target_folder=%ProgramFiles%\jooapa\SuperF4

:: Create the target folder if it does not exist
if not exist "%target_folder%" mkdir "%target_folder%"

:: Copy the files to the target folder
xcopy /y SuperF4.exe "%target_folder%"
xcopy /y blacklist.json "%target_folder%"

cd C:\Users\%USERNAME%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
echo start /B "" "%ProgramFiles%\jooapa\SuperF4\SuperF4.exe" > "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\SuperF4.bat"
