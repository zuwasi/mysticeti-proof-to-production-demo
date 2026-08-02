# Security policy

## Supported version

Security fixes are applied to the latest commit on `main` and the latest
published release.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability. Use GitHub's
private vulnerability reporting for this repository:

https://github.com/zuwasi/mysticeti-proof-to-production-demo/security/advisories/new

Include the affected file and revision, impact, reproduction steps, and any
suggested mitigation. Expect an acknowledgement within five business days.

## Security boundary

This repository is a bounded research digital twin, not production consensus
software. It does not implement production networking, signatures, storage,
epoch changes, validator integration, or Sui object execution. Do not deploy it
as a validator or use its output as a production safety guarantee.

See `docs/SECURITY_ASSESSMENT.md` for the public-release assessment and known
limitations.
