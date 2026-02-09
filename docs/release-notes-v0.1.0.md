# patchwaste v0.1.0 Release Notes (Draft)

## Highlights

- Initial PoC release of `patchwaste` for SteamPipe patch-efficiency regression detection.
- CLI analyze flow generates both machine-readable (`report.json`) and human-readable (`report.md`) output.
- Baseline comparison and budget gating are supported with stable exit codes:
  - `0`: pass
  - `2`: budget fail
  - `1`: tool/runtime error

## Included in v0.1.0

- Rust workspace structure (`crates/core`, `crates/cli`).
- Streaming log parser for SteamPipe preview output.
- Metrics:
  - `new_bytes`
  - `changed_content_bytes`
  - `delta_efficiency`
  - `waste_ratio`
  - `regression_ratio` (with baseline)
- Findings/rules for high waste and large top offender.
- Snapshot and behavior tests for core + CLI.
- CI checks (`fmt`, `clippy`, `test`) and local pre-commit hooks.
- Automation-safe dummy fixture:
  - `fixtures/automation_dummy/BuildOutput/steampipe_preview.log`

## E2E workflow

See:
- `README.md` for full E2E commands.
- `docs/development.md` for developer and CI workflow.
- `docs/release-v0.1.md` for release checklist.

## Operational notes

- Metrics are estimates unless confidence is high.
- Replace fixture input with real SteamPipe BuildOutput in production CI pipelines.
- For proprietary + paid rollout guidance, see `docs/proprietary-deployment.md`.
