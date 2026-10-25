@echo off
cd /d "%~dp0"
if not exist "Starfighter-Server.exe" (
 powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0Build.ps1"
 if errorlevel 1 (pause & exit /b 1)
)
Starfighter-Server.exe
pause
