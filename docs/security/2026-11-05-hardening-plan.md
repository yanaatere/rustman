# Security hardening — 2026-11-05

Base: development 2525f0029b601ecfb115dd8d65396f4b21ce1aa6.
Branch: codex/security-production-hardening. Do not push.

## Scope and design

Use source review and isolated synthetic fixtures. RCE testing is excluded by
user request. Do not read real credentials or test third-party targets.

User confirmed the issue is database storage, not UI masking. Encrypt environment
variables, history URLs/request snapshots, request/session JSON, drafts and
examples with standard AES-256-GCM. Use random nonces, row/column-bound associated
data, and a random per-database key in the native OS credential store. The database
contains only a nonsecret key ID. Never fall back to plaintext on key failure.
Migrate legacy rows transactionally; checkpoint/vacuum to remove ordinary SQLite
remnants. Historical backups and filesystem snapshots need separate handling.
IDs, collection/environment names, timestamps and HTTP method/status stay metadata.
Exports and Git working trees are separate security boundaries.

## Work and evidence

1. Baseline: 323 passed, 3 ignored before security changes.
2. Red regressions: custom API key crosses redirect origins; body buffers before
   limiting; private-address lookalike DNS defaults to HTTP; malformed JWT JSON;
   dropped WebSocket retains reader. Five tests failed as expected.
3. Updater authenticity/staging/limits fixed; 24 focused tests passed.
4. Implement authenticated storage and legacy migration; verify tampering,
   ciphertext swapping, nonce freshness, raw database scans and reopen.
5. Fix transport findings and constrain Git filenames/staging.
6. Run dependency review and full regression checks, then record release gates.

OSI review covers application/session/transport configuration and loopback tests;
physical/link/network infrastructure and non-Windows platform behavior are not
certified by this audit. Unicode display text is valid; security identifiers and
trust boundaries need strict syntax. Do not claim production readiness prematurely.
