# Launch Collateral And Tutorial Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add public launch collateral, issue intake templates, and a canonical hands-on tutorial for the open-core rollout.

**Architecture:** Keep changes documentation-only. Add top-level launch/legal docs, add structured GitHub issue templates to separate open-core bugs/features from commercial demand signals, and include a deterministic tutorial that uses the existing automation fixture and verified command outputs.

**Tech Stack:** Markdown docs, GitHub issue forms (YAML), Rust CLI command examples.

### Task 1: Add announcement, tutorial, and notice docs

**Files:**
- Create: `docs/ANNOUNCEMENT.md`
- Create: `docs/tutorial-canonical-example.md`
- Create: `NOTICE`

1. Add launch announcement text with scope, who it's for, and feedback channels.
2. Add step-by-step canonical tutorial using existing fixture and expected outcomes.
3. Add NOTICE with attribution boilerplate for Apache-2.0 distribution.

### Task 2: Add issue templates for clean feedback routing

**Files:**
- Create: `.github/ISSUE_TEMPLATE/bug_report.yml`
- Create: `.github/ISSUE_TEMPLATE/open-core-feature.yml`
- Create: `.github/ISSUE_TEMPLATE/commercial-interest.yml`
- Create: `.github/ISSUE_TEMPLATE/config.yml`

1. Add bug template requiring reproducible command/input context.
2. Add open-core feature template scoped to local CLI/parser/reporting use cases.
3. Add commercial interest template for hosted/team workflow requests.
4. Add issue config linking discussions and support docs.

### Task 3: Link docs in README

**Files:**
- Modify: `README.md`

1. Add links to announcement and tutorial docs.
2. Mention NOTICE file under license section.

### Task 4: Run canonical example and capture output

**Commands:**
- `cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --out /tmp/patchwaste-open-core-demo`
- `cat /tmp/patchwaste-open-core-demo/report.md`
- `cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --baseline /tmp/patchwaste-open-core-demo/report.json --budget-ratio 1.25 --out /tmp/patchwaste-open-core-demo-compare ; echo $?`

1. Verify tutorial commands work as documented.
2. Record key output values for user-facing summary.

### Task 5: Final report

1. Summarize files added/updated.
2. Include canonical run result highlights and next optional launch steps.
