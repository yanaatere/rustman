# Security review checkpoint — 2026-11-05

This is an in-progress defensive audit of Rustman on
`codex/security-production-hardening`, based on development
`2525f0029b601ecfb115dd8d65396f4b21ce1aa6`. It is not production approval.

- [Per-case register](cases.md): 37 cases/review areas; includes confirmed issues,
  partial fixes, intentional edit/export boundaries and inconclusive checks.
- [Structured register](cases.json), [dependency inventory](dependency-inventory.json),
  [RustSec report](rustsec-report.json) and [report metadata](rustsec-report-metadata.json).
- [Design/plan](2026-11-05-hardening-plan.md).
- Root `progress-security.md` contains the latest handoff and historical results.

## Verification

Windows, 2026-11-05:

| Check | Result |
|---|---|
| Baseline before changes | 323 passed, 3 ignored |
| Final default suite | 361 passed, 3 ignored |
| Final `--no-default-features` suite | 337 passed, 3 ignored |
| Production `cargo check --offline` | Passed |
| Native key/state create-read-delete smoke | Passed; unique synthetic entries deleted |
| Independent source review | No critical/important introduced regressions found in reviewed changes |

The three ignored network tests were not run. Tests used synthetic credentials,
bounded local fixtures, and temporary databases/repositories. No real user database
was migrated; no real credential values were read; no RCE or real OOM tests ran.
Native UI screenshots and non-Windows runtime verification remain open.

## Storage and masking boundary

Sensitive SQLite payloads use RustCrypto AES-256-GCM with per-database random keys
in the native credential store, fresh nonces and row/column-bound associated data.
Environment/history/request/session/draft/example payloads are protected. This is
not SQLCipher/full-file encryption: names, identifiers and other metadata remain
readable. Trusted migration completion is recorded outside the SQLite file.

Unit fixtures substitute an ephemeral key provider. The separate native smoke
program compiles the actual production keychain module and verifies its lifecycle
with unique `security-smoke-...` credentials, never production database UUIDs.

Masking/redaction covers known credential names/values on documented display and
diagnostic paths. It does not universally detect arbitrary, encoded or split
secrets. Actual send values remain original. Explicit unmasked edit/copy/export
boundaries are labelled; they are separate from storage encryption.

## Important remaining release gates

- SEC-013: inline credentials can still enter recognized Git/export snapshots.
- Real pinned update signing key and signed release pipeline/configuration.
- Immutable CI action references, dependency advisory/maintenance remediation.
- SEC-029: browser preview writes plaintext temporary HTML without lifecycle cleanup.
- SEC-034: synchronous full snippet construction occurs before display clipping.
- Script/global memory budgets and same-tab WebSocket connection-generation races.
- Full native-app isolated migration/recovery, visual QA, Linux/macOS keystores.
- Secret classification/reveal/copy policy across environment editors, bulk editors,
  opaque/raw responses and other explicit exports.

Per-frame/preview/history limits are not certification of total process memory.
SQLite payload authentication is not whole-database replacement/rollback protection.
Old backups, exports, Git history and same-user/OS compromise are outside that
boundary. Layers 1–3 and production infrastructure were not certified by source
and loopback tests on this host.

## Reproduce boundary checks

From the repository root, using a disposable development checkout:

```powershell
cargo test --offline
cargo test --offline --no-default-features
cargo check --offline
cargo run --offline --manifest-path scripts/security/native-keyring-smoke/Cargo.toml
cargo run --offline --manifest-path scripts/security/rustsec-offline/Cargo.toml -- <local-rustsec-db> Cargo.lock <report.json>
```

The native smoke check creates and deletes only its own unique synthetic entries;
do not adapt it to production key IDs. The advisory helper uses official RustSec
0.33.0 without Git/network transport and requires an already-fetched current local
database. It exits nonzero when vulnerabilities are matched. Current report:
2 build-dependency vulnerability matches, 13 unmaintained entries, 1 unsound warning;
reachability and upstream references are detailed in the case register.
