## ELI5

Charge studios monthly to *prevent* “20GB patches from 20MB changes.” Take payment via a subscription checkout, then give them a **license token** for CI. The tool runs in GitHub Actions and fails builds if patch waste exceeds a budget.

## Economist/FT-style summary

You want recurring B2B revenue with minimal support. The cleanest structure is: **free CLI (or free tier) + paid “policy & workflow”** (baselines, PR comments, multi-branch history, org controls). For payments, decide early whether you want a **Merchant of Record (MoR)** (handles VAT/sales tax) vs being **seller-of-record** (you handle tax). MoR is usually lower cognitive load at low volume. Paddle positions itself as all-in-one MoR for SaaS/software; Lemon Squeezy also positions as MoR and is now tied into Stripe’s MoR direction. ([paddle.com][1])

**Deutsch:** Monetarisierung = Abo + CI-Lizenz (Token) + Baseline/Policy als Premium.
**Español vocab:** suscripción (subscription), facturación (billing), impuestos (tax), licencia (license), umbral (threshold).
**Italiano vocab:** abbonamento (subscription), fatturazione (billing), imposte (tax), licenza (license), soglia (threshold).

---

# Top 3 things to decide (drives monetisation + implementation)

1. **MoR vs Stripe direct**

   * **MoR (Paddle / Lemon Squeezy / Stripe MoR direction):** they handle VAT/sales tax + filings; simpler global selling. ([paddle.com][1])
   * **Stripe direct:** maximum flexibility, but you own tax compliance; Stripe provides the payments platform + Billing add-on pricing. ([Stripe][2])

2. **Local-first license vs hosted SaaS**

   * **Local-first (recommended early for devtools trust):** customers keep build artifacts local; you enforce with a signed license file/token.
   * **Hosted dashboard:** stronger lock-in, but increases data-handling, security, and procurement friction.

3. **What is the billable unit**

   * Per **app/title** (most intuitive), per **depot/branch** (aligns with complexity), or per **CI runs** (aligns with compute, but irritates teams).

---

# A monetisation model that fits this product (low support, high retention)

## Tiering (practical)

* **Free:** local report, no baselines, no PR comments, single-branch.
* **Pro (£19–£49/mo per title):** baselines, regression budgets, PR annotations, 90-day history.
* **Studio (£99–£299/mo org):** multi-title, multi-branch policies, SSO (later), audit log, seat controls.

## What people pay for (not the metric)

* “**Policy**”: budgets/thresholds per branch, enforced in CI.
* “**Workflow**”: PR comments + diff to baseline + trend regressions.
* “**Diagnosis**”: engine-aware “why” + “what to change”.

---

# Accepting payments (fastest paths)

## Option A — Merchant of Record (lowest tax friction)

### Paddle (MoR)

* Use if you want: global tax handled, subscription billing, dunning, invoices “in one box.” Paddle markets pricing as all-inclusive. ([paddle.com][1])
  **Implementation shape**

1. Create product + plans.
2. On webhook `subscription_created/updated/canceled` → issue/rotate license token.
3. Customer portal handles upgrades/cancellations.

### Lemon Squeezy (MoR)

* Similar MoR posture; their pricing page explicitly frames MoR + VAT/tax handling. ([lemonsqueezy.com][3])
* Note: Lemon Squeezy’s 2026 update indicates continued evolution under Stripe’s MoR direction. ([lemonsqueezy.com][4])
  **Implementation shape:** same as Paddle—webhooks → license issuance.

**Why MoR is often better for you early:** fewer tax/VAT edge cases and fewer “please send compliant invoices for X country” interruptions.

## Option B — Stripe direct (most control, more responsibilities)

* Stripe Payments pricing is published by Stripe; Billing is a separate add-on pricing page. ([Stripe][2])
  **Implementation shape**

1. Stripe Checkout for subscriptions.
2. Webhooks to manage entitlements.
3. Stripe Customer Portal for self-serve.

---

# How to enforce payment in a Rust CLI without pissing off engineers

## Recommended: signed offline license (no network required)

* After checkout, user gets a `license.jwt` (or `license.toml` containing a signed token).
* CLI validates with embedded public key.
* Token contains:

  * org_id
  * plan
  * expiry
  * limits (titles, branches, PR comments)
* In CI: set `PATCHWASTE_LICENSE` secret to the token (or mount file).

**Grace period:** allow 7–14 days after expiry to avoid “builds suddenly blocked” during billing hiccups.

## When to add a license server

Only when you need:

* seat counts
* usage-based billing (runs/month)
* revocation
* org management UI

---

# GitHub Marketplace as a channel (optional)

You *can* sell via GitHub Marketplace if you package as a GitHub App/action with paid plans; GitHub docs describe payout mechanics and plan types. ([GitHub Docs][5])
Tradeoff: added listing/compliance work, but discoverability and procurement simplicity for GitHub-native buyers.

---

# Minimal “payments → entitlement” flow (BDD-ready)

**Scenario: new subscription**

* Given customer completes checkout
* When webhook `subscription.active` arrives
* Then create `org` + issue license token
* And email link to docs + “copy this secret into GitHub Actions”

**Scenario: failed payment**

* Given subscription enters past_due
* Then keep license valid for grace window
* And warn in report output (non-failing) until grace ends
* After grace → disable premium features (don’t hard-break free reports)

---

# Imaginary advisory board (compressed)

* **Engineers:** Don’t build a big SaaS first; ship a deterministic CLI + offline license so teams trust it.
* **Lawyers/compliance voices:** Prefer MoR early unless you want to own VAT/invoicing obligations worldwide.
* **Competitors/strategists:** The “% waste” is copyable; sell governance (budgets, baselines, PR policy) and engine-specific diagnosis.
* **Operators:** Don’t block builds on day 1; start “warn-only,” then enforce after baseline stabilises.

**Synthesis:** MoR subscription + offline signed license + CI policy gating is the shortest path to revenue with the least operational drag.

---

## Spaced repetition (quick test)

1. What’s the single biggest difference between MoR and Stripe-direct for a UK seller? ([paddle.com][1])
2. What entitlement mechanism avoids any network call from CI?
3. What feature should remain free to keep adoption viral?
4. What do you make paid to avoid commoditisation?

If you pick **MoR vs Stripe-direct** and **offline license vs license server**, I can give a concrete checklist of webhooks/events and a minimal entitlement data model.

[1]: https://www.paddle.com/pricing?utm_source=chatgpt.com "Pricing"
[2]: https://stripe.com/pricing?utm_source=chatgpt.com "Pricing & Fees"
[3]: https://www.lemonsqueezy.com/pricing?utm_source=chatgpt.com "Pricing"
[4]: https://www.lemonsqueezy.com/blog/2026-update?utm_source=chatgpt.com "2026 Update: Lemon Squeezy + Stripe Managed Payments"
[5]: https://docs.github.com/en/apps/github-marketplace/selling-your-app-on-github-marketplace?utm_source=chatgpt.com "Selling your app on GitHub Marketplace"
