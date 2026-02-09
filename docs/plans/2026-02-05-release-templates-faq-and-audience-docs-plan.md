# Release Templates FAQ And Audience Docs Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add release collateral (GitHub release template + docs template), FAQ, and restructure README/tutorial for three audience tiers: ELI5, non-technical educated readers, and engineering/deep technical users.

**Architecture:** Documentation-only changes with no runtime behavior changes. Introduce `.github/release.yml` for release note generation, create `docs/release-template.md` and `docs/FAQ.md`, then reorganize README and canonical tutorial into progressive disclosure sections while preserving exact commands and expected outcomes.

**Tech Stack:** Markdown docs, GitHub release config YAML, existing Rust CLI command examples.

### Task 1: Add release template assets

**Files:**
- Create: `.github/release.yml`
- Create: `docs/release-template.md`

1. Add release note categories aligned to open-core and commercial messaging.
2. Add reusable human-written release template for GitHub tags/releases.

### Task 2: Add FAQ

**Files:**
- Create: `docs/FAQ.md`

1. Answer licensing, open-core boundary, commercial roadmap, and contribution questions.
2. Keep wording consistent with Apache-2.0 + trademark policy docs.

### Task 3: Restructure README for audience tiers

**Files:**
- Modify: `README.md`

1. Add badges and a "Start Here" quick navigation box near top.
2. Add ELI5 section for first-time readers.
3. Add section for educated non-technical audience (Guardian/FT/Economist style).
4. Keep engineering quickstart and deep-dive references for technical audiences.
5. Link FAQ, tutorial, release template docs.

### Task 4: Restructure canonical tutorial into progressive layers

**Files:**
- Modify: `docs/tutorial-canonical-example.md`

1. Add audience structure: ELI5 -> non-technical -> engineering -> hacker deep dive.
2. Preserve exact runnable commands and expected output/exit codes.
3. Add interpretation guidance for each audience level.

### Task 5: Verify and summarize

**Commands:**
- `cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --out /tmp/patchwaste-open-core-demo`
- `ls .github/release.yml docs/release-template.md docs/FAQ.md`
- `rg -n "ELI5|Guardian|Economist|Start Here|badge|deep dive|FAQ" README.md docs/tutorial-canonical-example.md docs/FAQ.md`

1. Confirm canonical command still runs.
2. Confirm new docs exist and are linked.
