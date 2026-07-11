$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$visualSpec = Join-Path $repoRoot "docs\VISUAL_SPEC.md"
$visualReadme = Join-Path $repoRoot "tests\visual\README.md"
$goldenDir = Join-Path $repoRoot "tests\visual\golden"

$requiredSections = @(
    "## 1. Lab-Coat palette",
    "## 2. Empty states",
    "## 3. Loading & skeleton states",
    "## 4. Error & failure states",
    "## 5. Motion & reduced motion",
    "## 7. Keyboard map (viewer)",
    "## 8. Acceptance (L107)"
)

$missing = @()
foreach ($section in $requiredSections) {
    if (-not (Select-String -Path $visualSpec -SimpleMatch $section -Quiet)) {
        $missing += $section
    }
}

if (-not (Test-Path -Path $visualReadme -PathType Leaf)) {
    $missing += "tests/visual/README.md"
}
if (-not (Test-Path -Path $goldenDir -PathType Container)) {
    $missing += "tests/visual/golden/"
}

if ($missing.Count -gt 0) {
    Write-Error ("Visual contract validation failed. Missing: " + ($missing -join ", "))
}

Write-Host "Visual contract validation passed."
