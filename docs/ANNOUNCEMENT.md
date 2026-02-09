# patchwaste Open Core Launch

We are launching `patchwaste` as an Apache-2.0 open core focused on one job:
help Unreal + Steam teams catch patch-size regressions early.

## What is available now

- Local-first CLI analysis from SteamPipe BuildOutput logs
- Baseline comparison and budget gate exit codes for CI
- JSON/Markdown/Text outputs for automation and review

## What is intentionally not in open core

- Hosted dashboard and managed long-term history
- Team/org policy management
- SSO, audit logs, and commercial SLA support

Those are planned for a commercial tier. See `docs/open-core-scope.md` and `docs/roadmap.md`.

## Why this split

Open core stays small, auditable, and useful immediately.
Commercial value will focus on team workflow, governance, and operational support.

## Who this first release is for

- Unreal build/release engineers using SteamPipe
- Teams wanting a simple CI gate for patch waste regression

## How to get started

- Quickstart in `README.md`
- End-to-end walkthrough in `docs/tutorial-canonical-example.md`

## Feedback we need most

1. False positives/false negatives from real logs
2. Report clarity: "what changed" and "what to fix"
3. CI integration friction

Please use GitHub Issues/Discussions (see `SUPPORT.md`).
