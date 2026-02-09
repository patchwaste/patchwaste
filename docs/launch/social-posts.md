# Social Post Pack (v0.1.0)

## X / short form

Shipped `patchwaste` v0.1.0: Apache-2.0 open-core CLI for Unreal + Steam teams to catch patch-size regressions in CI.

- local-first analysis
- baseline compare + budget gate
- JSON/MD/text reports

Show HN soon. Feedback wanted on false positives + CI friction.

Repo: <REPO_URL>
Tutorial: <REPO_URL>/blob/main/docs/tutorial-canonical-example.md

## LinkedIn

We just released `patchwaste` v0.1.0 as Apache-2.0 open core.

It helps Unreal + Steam build/release teams detect patch-size regressions before shipping by converting SteamPipe BuildOutput into clear metrics and CI-friendly pass/fail gates.

What is in v0.1.0:
- CLI analysis and baseline comparison
- Budget-gate exit codes for CI
- JSON/Markdown/Text reports
- Canonical tutorial for first run

What we want feedback on:
1. false positives/false negatives
2. report clarity and actionability
3. integration friction in CI

Repo: <REPO_URL>
Tutorial: <REPO_URL>/blob/main/docs/tutorial-canonical-example.md

## Reddit (r/gamedev / r/devops)

Title: Open-source tool for Unreal/Steam patch regression gating (Rust CLI)

Body:
We released `patchwaste` v0.1.0 (Apache-2.0).

It parses SteamPipe preview BuildOutput and reports patch waste metrics so you can budget-gate in CI.

Current scope is intentionally local-first open core:
- analyze build output
- compare against baseline
- fail/pass with exit codes

If you try it, I’d love feedback on:
- accuracy on real logs
- where reports are unclear
- CI onboarding pain points

Repo: <REPO_URL>
Tutorial: <REPO_URL>/blob/main/docs/tutorial-canonical-example.md
