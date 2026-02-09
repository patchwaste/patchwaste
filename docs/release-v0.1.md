# v0.1 Release Checklist

Use this checklist before tagging and announcing v0.1.

## 1) Verify quality gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

## 2) Verify E2E flow

```bash
cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --out patchwaste-out
cp patchwaste-out/report.json baseline.json
cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --baseline baseline.json --budget-ratio 1.25
```

Expected:
- reports are generated
- compare run exits `0`

Optional failure-path check:

```bash
printf '{"metrics":{"new_bytes":1000}}\n' > baseline-small.json
cargo run -p patchwaste -- analyze --input fixtures/automation_dummy/BuildOutput --baseline baseline-small.json --budget-ratio 1.25
echo $?  # should be 2
```

## 3) Final docs pass

- `README.md` E2E example is current
- `docs/development.md` matches CI checks
- `docs/proprietary-deployment.md` reviewed for deployment posture

## 4) Tag and release

```bash
git tag -a v0.1.0 -m "patchwaste v0.1.0"
git push origin v0.1.0
```

Recommended release notes highlights:
- baseline + budget-gate flow
- exit-code contract (`0`, `2`, `1`)
- dummy automation fixture availability

## 5) Rollback plan

If release issue is found:
- mark release as superseded in notes
- cut `v0.1.1` with fix
- avoid force-moving tags
