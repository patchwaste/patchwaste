# Proprietary Deployment and Payments Guidance

If this product is proprietary and monetized, use this baseline posture.

## Source and artifacts

- Keep source in a private GitHub/GitLab repo with branch protection.
- Store build artifacts/reports in a private object store bucket (S3/GCS/Azure Blob) with short-lived signed URLs.
- Encrypt at rest and in transit; rotate keys regularly.

## Runtime placement

- Run the CLI in your CI runner or a private worker network, not on customer machines.
- Keep SteamPipe logs and generated reports in tenant-scoped storage paths.
- Add retention policies (for example: 30-90 days for raw logs, longer for summarized billing records).

## Secrets and access

- Use cloud secret manager (AWS Secrets Manager / GCP Secret Manager / Vault).
- Do not store credentials in repo or workflow YAML.
- Enforce least-privilege IAM roles per environment.

## Payments and compliance

- Use Stripe (or equivalent) hosted checkout/customer portal to reduce PCI scope.
- Keep payment processing and card storage fully with the payment provider.
- Store only customer IDs, subscription status, and invoice metadata in your DB.
- Add audit logs for access, report generation, and subscription state changes.

## Recommended minimum stack

- App/API: private VPC service
- DB: managed Postgres with encryption
- Queue/Workers: managed queue + isolated workers
- Storage: private object storage bucket with lifecycle and KMS
- Billing: Stripe subscriptions + webhook processing

This setup gives a practical v0.1 commercial foundation while keeping security/compliance scope manageable.
