param(
    [string]$logLevel = "debug"
)

$env:RUST_LOG = $logLevel
cargo run

$env:RUST_LOG = $null
