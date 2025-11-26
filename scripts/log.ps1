param(
    [string]$logLevel = "debug"
)

$env:RUST_LOG = $logLevel
cargo run > output.log 2>&1

$env:RUST_LOG = $null

Write-Host "Done logging. File saved at 'output.log'."