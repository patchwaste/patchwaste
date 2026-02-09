## patchwaste v0.1.0

Release date: 2026-02-05

### What this release is for

`patchwaste` v0.1.0 is the first public Apache-2.0 open-core release focused on Unreal + Steam teams. It helps detect patch-size regressions early by turning SteamPipe preview BuildOutput logs into clear metrics, findings, and CI-friendly pass/fail signals.

### Highlights

- Open-core CLI for local-first patch waste analysis
- Baseline comparison and budget-gate exit codes for CI automation
- Stable report outputs (`json`, `md`, `text`) for tooling and review
- Canonical tutorial and onboarding docs for multiple audience levels
- Open-core governance docs (license, trademark, security, support, contributing)

### Canonical example status

- `analyze` run: PASS
- baseline compare gate: PASS (exit `0`)
- forced fail gate: PASS (exit `2` expected)

### Breaking changes

- None

### Upgrade notes

- Initial public release; no migration required.

### Open core boundary reminder

Open core covers local CLI parsing, metrics, baseline comparison, and reporting.

Hosted/team workflow features are on the commercial track:
- managed history/dashboard
- org policy management
- PR annotation services
- enterprise controls (SSO/audit) and SLA support

See:
- `docs/open-core-scope.md`
- `docs/roadmap.md`
- `docs/FAQ.md`

### Documentation

- Quickstart: `README.md`
- Canonical tutorial: `docs/tutorial-canonical-example.md`
- Announcement: `docs/ANNOUNCEMENT.md`
- Release format reference: `docs/release-template.md`

### Full changelog

First public release.
