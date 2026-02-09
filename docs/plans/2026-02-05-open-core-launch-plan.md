# Open Core Launch Docs Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Establish legal, governance, and messaging scaffolding for an Apache-2.0 open-core launch with a clear paid-tier boundary.

**Architecture:** Keep product code unchanged and focus on repository policy/docs. Add canonical legal and community docs at repo root, then add strategy docs under `docs/` and align README positioning to the new open-core narrative.

**Tech Stack:** Markdown documentation, Apache-2.0 license text, Rust workspace README.

### Task 1: Add legal and governance foundation

**Files:**
- Create: `LICENSE`
- Create: `TRADEMARK.md`
- Create: `SECURITY.md`
- Create: `SUPPORT.md`
- Create: `CONTRIBUTING.md`

1. Add full Apache-2.0 license text.
2. Define trademark use restrictions for name/logo while keeping code open.
3. Add vulnerability disclosure policy and supported versions section.
4. Add support expectations and channels.
5. Add contribution flow with DCO sign-off requirement.

### Task 2: Add open-core strategy docs

**Files:**
- Create: `docs/open-core-scope.md`
- Create: `docs/roadmap.md`

1. Document what is in open core now and what is reserved for paid tier.
2. Document near-term roadmap split into Open Core (public) and Commercial (private).

### Task 3: Align top-level README messaging

**Files:**
- Modify: `README.md`

1. Add explicit open-core positioning and licensing summary.
2. Link new docs (`CONTRIBUTING`, `SECURITY`, `SUPPORT`, `TRADEMARK`, roadmap/scope).
3. Keep current quickstart and CI gate examples intact.

### Task 4: Verify docs integrity

**Commands:**
- `ls LICENSE TRADEMARK.md SECURITY.md SUPPORT.md CONTRIBUTING.md docs/open-core-scope.md docs/roadmap.md`
- `rg -n "open core|Apache-2.0|paid" README.md docs/open-core-scope.md docs/roadmap.md`

### Task 5: Final report

1. Summarize created files and why each exists.
2. Call out any deferred decisions (e.g., CLA migration timing).
