[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$exports = Join-Path $projectRoot "exports"
New-Item -ItemType Directory -Force -Path $exports | Out-Null
$env:Path = "$HOME\.cargo\bin;$env:Path"

Write-Host "[1/3] Building and exercising Rust digital twin..." -ForegroundColor Cyan
$rustRoot = Join-Path $projectRoot "rust"
$rustLog = [System.Collections.Generic.List[string]]::new()
$rustLog.Add("Mysticeti Rust Build Report")
$rustLog.Add("===========================")
$rustLog.Add((& rustc --version 2>&1 | Out-String).Trim())
$rustLog.Add((& cargo --version 2>&1 | Out-String).Trim())
$rustExit = 0
function Invoke-RustStep([string]$label, [string[]]$arguments) {
    $rustLog.Add("`n## $label`n> cargo $($arguments -join ' ')")
    $text = (& cargo @arguments 2>&1 | Out-String)
    $code = $LASTEXITCODE
    $rustLog.Add($text.TrimEnd())
    $rustLog.Add("Exit code: $code")
    if ($code -ne 0) { $script:rustExit = $code }
}
Push-Location $rustRoot
try {
    Invoke-RustStep "Format check" @("fmt", "--check")
    Invoke-RustStep "All-target tests" @("test", "--all-targets")
    Invoke-RustStep "Clippy warnings gate" @("clippy", "--all-targets", "--", "-D", "warnings")
    Invoke-RustStep "Release demo" @("run", "--release", "--", "demo", "--output", "..\exports\rust_demo_trace.json")
    Invoke-RustStep "Strict trace verification" @("run", "--release", "--", "verify", "..\exports\rust_demo_trace.json")
    $replayOutput = (& (Join-Path $rustRoot "target\release\mysticeti-twin.exe") replay "..\exports\rust_demo_trace.json" 2>&1 | Out-String)
    $replayExit = $LASTEXITCODE
    $rustLog.Add("`n## Replay capture`nExit code: $replayExit")
    if ($replayExit -eq 0) { Set-Content -Path (Join-Path $exports "rust_demo_replay.json") -Value $replayOutput -Encoding UTF8 } else { $rustExit = $replayExit; $rustLog.Add($replayOutput) }
    Invoke-RustStep "20-seed sequential event-driven sweep" @("run", "--release", "--", "sweep", "--seeds", "20", "--jobs", "1", "--output", "..\exports\rust_fault_sweep.csv")
    Invoke-RustStep "20-seed parallel event-driven sweep" @("run", "--release", "--", "sweep", "--seeds", "20", "--jobs", "8", "--output", "..\exports\rust_fault_sweep_parallel.csv")
} finally { Pop-Location }
$sequentialSweep = Join-Path $exports "rust_fault_sweep.csv"
$parallelSweep = Join-Path $exports "rust_fault_sweep_parallel.csv"
$sweepsEqual = (Test-Path $sequentialSweep) -and (Test-Path $parallelSweep) -and
    [System.Linq.Enumerable]::SequenceEqual(
        [System.IO.File]::ReadAllBytes($sequentialSweep),
        [System.IO.File]::ReadAllBytes($parallelSweep)
    )
$sequentialHash = if (Test-Path $sequentialSweep) { (Get-FileHash $sequentialSweep -Algorithm SHA256).Hash } else { "MISSING" }
$parallelHash = if (Test-Path $parallelSweep) { (Get-FileHash $parallelSweep -Algorithm SHA256).Hash } else { "MISSING" }
$rustLog.Add("`n## Deterministic parallel campaign evidence")
$rustLog.Add("jobs=1 SHA-256: $sequentialHash")
$rustLog.Add("jobs=8 SHA-256: $parallelHash")
$rustLog.Add("Byte-identical: $sweepsEqual")
if (-not $sweepsEqual) { $rustExit = 1 }
$rustLog.Add("`nCriterion benchmark is intentionally separate QA: cargo bench --bench simulation")
Set-Content -Path (Join-Path $exports "rust_build_report.txt") -Value ($rustLog -join "`n") -Encoding UTF8

Write-Host "[2/3] Building Wolfram model and cross-language evidence..." -ForegroundColor Cyan
Push-Location $projectRoot
try { $wolframOutput = (& wolframscript -file (Join-Path $projectRoot "build_project.wls") 2>&1 | Out-String); $wolframExit = $LASTEXITCODE } finally { Pop-Location }
Write-Host $wolframOutput

Write-Host "[3/3] Building Lean proof kernel..." -ForegroundColor Cyan
$leanRoot = Join-Path $projectRoot "lean"
Push-Location $leanRoot
try {
    $leanVersion = (& lean --version 2>&1 | Out-String).Trim()
    $lakeVersion = (& lake --version 2>&1 | Out-String).Trim()
    $mathlibRevision = (& git -C ".lake/packages/mathlib" rev-parse HEAD 2>&1 | Out-String).Trim()
    $leanOutput = (& lake build 2>&1 | Out-String)
    $leanExit = $LASTEXITCODE
} finally {
    Pop-Location
}

$proofSources = Get-ChildItem (Join-Path $leanRoot "MysticetiProofs") -Filter "*.lean" -File
$proofSources += Get-Item (Join-Path $leanRoot "MysticetiProofs.lean")
$placeholderMatches = $proofSources | Select-String -Pattern '\b(sorry|admit|axiom|unsafe|native_decide)\b' -CaseSensitive:$false
$placeholderStatus = if ($placeholderMatches) { "FAILED" } else { "PASSED" }

$leanReport = @"
Mysticeti Lean 4 Build Report
============================
Lean: $leanVersion
Lake: $lakeVersion
Mathlib revision: $mathlibRevision
Exit code: $leanExit
Forbidden-placeholder scan: $placeholderStatus

$leanOutput
"@
Set-Content -Path (Join-Path $exports "lean_build_report.txt") -Value $leanReport -Encoding UTF8

$requiredRust = @("rust_demo_trace.json", "rust_demo_replay.json", "rust_fault_sweep.csv", "rust_fault_sweep_parallel.csv", "rust_build_report.txt")
$rustArtifactsOK = -not ($requiredRust | Where-Object { -not (Test-Path (Join-Path $exports $_)) -or (Get-Item (Join-Path $exports $_)).Length -eq 0 })
$allPassed = ($rustExit -eq 0) -and $rustArtifactsOK -and ($wolframExit -eq 0) -and ($leanExit -eq 0) -and (-not $placeholderMatches)
$combined = @"
# Mysticeti Combined Validation Report

- Rust lane exit code: $rustExit
- Rust required artifacts nonempty: $rustArtifactsOK
- Wolfram lane exit code: $wolframExit
- Lean build exit code: $leanExit
- Lean placeholder scan: $placeholderStatus
- Combined release gate: $(if ($allPassed) { "PASSED" } else { "FAILED" })

## Evidence boundaries

- Rust executes a deterministic, stake-weighted, event-driven research twin with strict replay/tamper checks; it is not production Sui.
- Wolfram independently audits the recorded Rust schema, references, stake arithmetic, and evidence, alongside its paper-specific fixtures.
- Lean kernel-checks only the exact equal-authority quorum statements mapped in `docs/formalization_map.md`; Lean does not prove Rust or Wolfram.
- The project does not claim complete Mysticeti safety, liveness, cryptography, epoch-change, or production-performance verification.

## Wolfram output

$wolframOutput
"@
Set-Content -Path (Join-Path $exports "combined_validation_report.md") -Value $combined -Encoding UTF8

if (-not $allPassed) {
    throw "Combined release gate failed. See exports\lean_build_report.txt and exports\combined_validation_report.md."
}

Write-Host "COMBINED RELEASE GATE PASSED" -ForegroundColor Green
