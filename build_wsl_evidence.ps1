[CmdletBinding()]
param(
    [string]$Distro = "Ubuntu-24.04",
    [switch]$Benchmark
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$linuxRoot = (& wsl.exe -d $Distro -- wslpath -a ($projectRoot -replace '\\', '/') 2>&1 | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or -not $linuxRoot) {
    throw "Could not map the project into WSL distro '$Distro'."
}

$benchmarkCommand = if ($Benchmark) {
    'cargo bench --bench simulation -- --noplot --sample-size 10 2>&1 | tee ../exports/wsl_benchmark_report.txt;'
} else {
    'echo "Benchmark skipped; rerun with -Benchmark.";'
}

$script = @"
set -euo pipefail
. "`$HOME/.cargo/env"
cd '$linuxRoot/rust'
export CARGO_TARGET_DIR=/tmp/mysticeti-twin-target
{
  echo 'Mysticeti WSL/Linux Reproducibility Report - Parallel Campaign Edition'
  echo '================================================================'
  grep PRETTY_NAME /etc/os-release
  uname -srmo
  rustc --version
  cargo --version
  echo "cpus=`$(nproc)"
  cargo fmt --check
  cargo test --all-targets
  cargo clippy --all-targets -- -D warnings
  cargo run --release -- demo --output ../exports/rust_demo_trace_wsl.json
  cargo run --release -- verify ../exports/rust_demo_trace_wsl.json
  cargo run --release -- replay ../exports/rust_demo_trace_wsl.json > ../exports/rust_demo_replay_wsl.json
  cargo run --release -- sweep --seeds 20 --jobs 1 --output ../exports/rust_fault_sweep_wsl_jobs1.csv
  cargo run --release -- sweep --seeds 20 --jobs 8 --output ../exports/rust_fault_sweep_wsl_jobs8.csv
  sha256sum ../exports/rust_demo_trace.json ../exports/rust_demo_trace_wsl.json
  cmp -s ../exports/rust_demo_trace.json ../exports/rust_demo_trace_wsl.json
  echo 'BYTE_IDENTICAL_WINDOWS_WSL_TRACE=true'
  sha256sum ../exports/rust_fault_sweep.csv ../exports/rust_fault_sweep_wsl_jobs1.csv ../exports/rust_fault_sweep_wsl_jobs8.csv
  cmp -s ../exports/rust_fault_sweep.csv ../exports/rust_fault_sweep_wsl_jobs1.csv
  cmp -s ../exports/rust_fault_sweep_wsl_jobs1.csv ../exports/rust_fault_sweep_wsl_jobs8.csv
  echo 'BYTE_IDENTICAL_WINDOWS_WSL_JOBS1_JOBS8=true'
} 2>&1 | tee ../exports/wsl_rust_build_report.txt
$benchmarkCommand
"@

& wsl.exe -d $Distro -- bash -lc $script
if ($LASTEXITCODE -ne 0) {
    throw "WSL evidence build failed with exit code $LASTEXITCODE."
}

Write-Host "WSL CROSS-PLATFORM EVIDENCE PASSED" -ForegroundColor Green
