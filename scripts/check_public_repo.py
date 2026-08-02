#!/usr/bin/env python3
"""Fail when the public repository loses required evidence or portability."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXPECTED_TRACE = "6272fa854de66bc42512f38d095d7ccf6f75bb85f581dc4acaabf9fbe8ede71d"
REQUIRED = [
    "LICENSE",
    "NOTICE.md",
    "CITATION.cff",
    "SECURITY.md",
    "rust/Cargo.lock",
    "lean/lean-toolchain",
    "docs/formalization_map.md",
    "presentation/html/index.html",
]
FORBIDDEN_PARTS = {"target", ".lake", "lo-profile"}
TEXT_SUFFIXES = {".md", ".toml", ".yml", ".yaml", ".json", ".ps1", ".py", ".rs", ".wl", ".wls", ".html", ".js"}


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


for relative in REQUIRED:
    if not (ROOT / relative).is_file():
        fail(f"missing required file: {relative}")

tracked_candidates = [p for p in ROOT.rglob("*") if p.is_file() and ".git" not in p.parts]
for path in tracked_candidates:
    relative = path.relative_to(ROOT)
    if relative == Path("scripts/check_public_repo.py"):
        continue
    if FORBIDDEN_PARTS.intersection(relative.parts):
        continue
    if path.suffix.lower() not in TEXT_SUFFIXES and path.name not in {"LICENSE"}:
        continue
    text = path.read_text(encoding="utf-8", errors="ignore")
    if re.search(r"C:\\(?:Users|Amp_demos)\\", text, re.IGNORECASE):
        fail(f"machine-specific Windows path in {relative}")
    if "/mnt/c/Amp_demos/" in text:
        fail(f"machine-specific WSL path in {relative}")

trace = ROOT / "exports/rust_demo_trace.json"
if hashlib.sha256(trace.read_bytes()).hexdigest() != EXPECTED_TRACE:
    fail("checked-in demonstration trace hash changed unexpectedly")

validation = json.loads((ROOT / "exports/validation_results.json").read_text(encoding="utf-8-sig"))
if validation.get("Passed") != 12 or validation.get("Failed") != 0 or validation.get("AllPassed") is not True:
    fail("Wolfram validation snapshot is not 12/12 passing")

html = (ROOT / "presentation/html/index.html").read_text(encoding="utf-8")
slides = re.findall(r'<section class="slide"', html)
references = sorted(set(re.findall(r"slides/(slide-[0-9]+\.jpg)", html)))
if len(slides) != 22 or len(references) != 22:
    fail(f"expected 22 HTML slides and images, got {len(slides)} and {len(references)}")
for image in references:
    if not (ROOT / "presentation/html/slides" / image).is_file():
        fail(f"missing HTML slide image: {image}")

repo_url = "https://github.com/zuwasi/mysticeti-proof-to-production-demo"
if repo_url not in html or repo_url not in (ROOT / "README.md").read_text(encoding="utf-8"):
    fail("public repository URL is not exposed in README and HTML presentation")

print("Public repository integrity checks passed.")
