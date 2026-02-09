# Show HN Draft

## Suggested title

Show HN: patchwaste — catch Steam patch-size regressions in CI (Rust, Apache-2.0)

## Post body

I built `patchwaste` to catch a common release problem: tiny real content changes that produce surprisingly large Steam patches.

`patchwaste` parses SteamPipe preview BuildOutput and gives:
- patch waste metrics (`new_bytes`, `changed_content_bytes`, `waste_ratio`)
- baseline comparison
- CI-friendly pass/fail via exit codes

It is Apache-2.0 open core and currently focused on Unreal + Steam workflows.

Quick run:

```bash
cargo run -p patchwaste -- analyze \
  --input fixtures/automation_dummy/BuildOutput \
  --out /tmp/patchwaste-open-core-demo
```

In the bundled canonical example it prints:
- `new_bytes=4194304`
- `changed_content_bytes=1048576`
- `waste_ratio=0.750`

Repo: https://github.com/patchwaste/patchwaste
Tutorial: https://github.com/patchwaste/patchwaste/blob/main/docs/tutorial-canonical-example.md
FAQ: https://github.com/patchwaste/patchwaste/blob/main/docs/FAQ.md

Feedback I’m specifically looking for:
1. false positives/negatives on real logs
2. report clarity (what changed, what to fix)
3. CI integration friction

## First comment to pin

Thanks for checking it out. Scope note: open core is local CLI/parser/reporting only. Hosted team workflows (org policy/history/dashboard/enterprise controls) are planned separately.

If you test it, please share:
- input context (sanitized)
- command run
- expected vs actual

Issue templates here: https://github.com/patchwaste/patchwaste/issues/new/choose

## Fast reply snippets

### Why Apache instead of noncommercial?

I want maximum adoption and trust for the core analysis layer. Commercial value is planned around hosted team workflow and governance, not restricting local use.

### Why Unreal + Steam first?

Narrow scope gives better signal quality and faster iteration. I’d rather be excellent in one pipeline first than broad and noisy.

### How is this different from a script?

Main goals are stable schema, reproducible outputs, baseline gating, and actionable findings with CI-ready exit codes.

### Is this replacing profiling/build tooling?

No, it complements existing tooling by adding a patch-efficiency regression gate and report layer.
