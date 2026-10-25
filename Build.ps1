$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath $PSScriptRoot
$mingw = Join-Path (Split-Path $PSScriptRoot) '194x\.tools\mingw64\bin'
if (Test-Path -LiteralPath $mingw) { $env:PATH = "$mingw;$env:PATH" }
cargo build --release --bin starfighter
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo build --release --target wasm32-unknown-unknown --bin starfighter
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Copy-Item -LiteralPath 'target\release\starfighter.exe' -Destination 'Starfighter-Latest.exe' -Force
cargo build --release --bin starfighter-server
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Copy-Item -LiteralPath 'target\release\starfighter-server.exe' -Destination 'Starfighter-Server.exe' -Force
Copy-Item -LiteralPath 'target\wasm32-unknown-unknown\release\starfighter.wasm' -Destination 'web\starfighter.wasm' -Force
Write-Host 'Built Starfighter-Latest.exe and the mobile browser game in web/.'
