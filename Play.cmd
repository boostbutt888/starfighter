@echo off
cd /d "%~dp0"
if exist "Starfighter-Latest.exe" (
 start "" "Starfighter-Latest.exe"
 exit /b
)
if exist "Starfighter.exe" (
 start "" "Starfighter.exe"
 exit /b
)
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0Build.ps1"
if errorlevel 1 (pause & exit /b 1)
start "" "Starfighter.exe"
