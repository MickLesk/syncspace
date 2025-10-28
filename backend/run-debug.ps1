Set-Location "c:\Users\LeskowitzMickey\Documents\GitHub\syncspace\backend"
$env:RUST_LOG = "debug"
$env:RUST_BACKTRACE = "1"
cargo run --release
