# Design (MVP)

## Goal

Detect patch-size regressions *before release* by analyzing SteamPipe preview build outputs and producing:

* reproducible metrics (delta efficiency / waste)
* CI gate (budget thresholds vs baseline)
* actionable diagnostics (likely causes + mitigations)

## Inputs

* A directory containing preview build artifacts (logs at minimum).
* Optional baseline report JSON.

## Outputs

* report.json (versioned schema)
* report.md (human summary)
* exit code: 0 pass, 2 budget fail, 1 error

## Core metric definitions

* new_bytes: estimated bytes newly downloaded (from log counters if present; otherwise proxy)
* changed_content_bytes: estimated changed content bytes (counter or proxy)
* delta_efficiency = changed_content_bytes / max(new_bytes, 1)
* waste_ratio = 1 - delta_efficiency
* regression_ratio = new_bytes / max(baseline_new_bytes, 1)

## Resilience

* streaming parsing, input size caps, explicit error categories, confidence levels
* regression tests via fixtures + stable JSON snapshots

## Repo layout

* crates/core: parsing, metrics, rules, reporters
* crates/cli: clap CLI wrapper
* fixtures: golden input samples
* .github/workflows: CI
