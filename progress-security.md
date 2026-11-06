# Latest verified eye-control checkpoint — 2026-11-06

- Security rerun completed:16 example tests passed/0 failed, covering no outgoing
  Task/client, byte-identical ciphertext, unchanged parent/template/dirty state,
  close/open reset, invalid index, row deletion/add/rename and reveal isolation.
- Native eye checks: default hidden, selected cookie/API only, second click hide,
  Save hide, Body/Headers switch hide. Actual evidence E11-E16.
- Full pack docs/security/evidence/2026-11-06/README.md:16 original JPEGs, SHA256,
  build mapping, red/green logs,12-case security matrix. Tool returned JPEG bytes;
  initial .png filenames were corrected to .jpg without changing image bytes.
- SEC-039/040/041 updated; E09 retained as pre-fix failure. Example is AES256GCM;
  after reveal/Save raw DB/WAL have no known dummy credential markers.
- Production eye build succeeded; git diff --check passed. No production source
  changed during final security rerun (only additional tests).
- Eye is UI visibility, not lazy DB decryption. No arbitrary-body masking,
  TLS/MITM/whole-process or keyboard navigation pass claimed.
- All continuation changes remain local/uncommitted in security worktree.
  No push/remote Git action. Main checkout still a766ace. Wider full audit OPEN,
  including previous combined loopback failure and QA-001 history observation.

---

# Saved-example follow-up and archived evidence — 2026-11-06

- Nine actual PNG screenshots exported (E01-E09), not mockups, with SHA256 and
  native window metadata in docs/security/evidence/2026-11-06/screenshots.json.
  Timestamp identifies export time, not signed capture time. No real secrets.
- E01 persistent Git rejection notification, E02 dismissal, E03 cookie masking,
  E04 actual 200/owned-server assertions, E05 header masking, E06 Save,
  E07 before restart history, E08 new-window restored session,
  E09 pre-fix FAILURE: saved example exposes dummy cookie/API key plaintext.
- SEC-039 native toast passed visible/error/persistent/dismiss/retrigger; related
  Git tests 4 passed. No push, no remote or successful Git commit in QA fixture.
- User added saved-example QA. Real UI Save response created one example;
  SQLite request_examples.data has AES256GCM prefix and DB/WAL contain no known
  dummy credential markers. Display alone exposed decrypted headers in editor.
- SEC-040: example header/parameter value inputs now secure for sensitive keys.
  Native E09 is the failing behavior. Native rebuild/reopen still pending in
  functions cell 111; do not claim masked example GUI passed yet.
- Example suite 13 passed/0 failed, including new save-response/save/reopen
  assertions: original bearer/cookie/API key survive encryption, parent unchanged.
- User asked whether resolved data can be viewed for debug. Recommended explicit
  per-field temporary Show/Hide, default hidden, reset on save/panel/reopen; never
  replace stored {{TOKEN}} template with resolved value. That reveal control is
  a design recommendation, NOT implemented/tested by the current one-line fix.
- Broader final full-suite and prior failed auth loopback fixture remain OPEN.
  History replay candidate QA-001 remains separate. Latest source is uncommitted
  in security worktree; main checkout still lacks these uncommitted changes.

---

# Native vision QA and Git notification fix — 2026-11-06

Vision now works. Native GUI checks were performed, not only source edits.
See docs/security/native-qa-2026-11-06.md for exact pass boundaries and open items.

- Actual Send to host loopback: literal bearer and {{TOKEN}} both correct; dummy
  JSON password/Unicode correct; response headers/cookies and snippet masked.
- Save + normal close + real reopen restored encrypted session/history/request.
  Raw DB/WAL/SHM contained no known synthetic credential markers. All protected
  columns encrypted, including nine history rows. Native keychain used.
- Git refresh guard denied dummy password body before snapshot; no JSON files or
  commits created, repo has no remote. No push; user reiterated local examples only.
- Preview static HTML opened; 121-byte owned file removed after normal close.
- User noticed invisible rejection feedback. Root cause status-bar-only update.
  SEC-039 regression RED (missing notification), then GREEN 1 passed. Error toast
  is persistent/dismissible and uses static guidance, never raw credentials.
  Native rebuild/retest is in progress: target/security-runtime-build.log.
- QA-001 seeded history dedup observation recorded separately, investigation OPEN.
- User asked about sniffing: explained HTTP plaintext versus validated HTTPS/TLS,
  and at-rest AES boundary. No TLS interception test was claimed or run.
- All current continuation edits remain in security worktree, uncommitted. Main
  checkout shares branch but does not contain these uncommitted changes.
- Full final suite still pending; prior combined fixture result 12 pass/1 failure
  remains unresolved. Never certify production readiness from these focused checks.

---

# Vision QA retry — 2026-11-06

User explicitly authorized vision QA. Read Windows Computer Use guidance and
confirmation policy, attempted node_repl initialization, reset kernel and retried;
both failed before sky could initialize. Reset CUA fallback and attempted inventory;
same failure: "failed to write kernel assets: The system cannot find the path
specified. (os error 3)". No screenshot acquired, no window input sent, no visual
PASS claimed. Previous synthetic smoke process was not found in current inventory
query; no app was launched without an available observation channel. Real database
and credentials untouched. Visual QA remains blocked on desktop tool initialization;
retry after desktop app/tool session recovery. Prior native startup/AES results
remain historical and do not substitute for visual QA.

---

# Checkout/manual handoff — 2026-11-05

- Main checkout D:/Dev/Rust/postman-clone/rustman switched as requested to
  codex/security-production-hardening, a766ace; used --ignore-other-worktrees
  because security worktree also owns this branch. No reset/clean/push.
- Main checkout pre-existing .gitignore edit and untracked .claude-flow directories
  and transport-audit-http.rs preserved. Latest continuation edits remain ONLY
  in security worktree and are not yet in main checkout.
- Both checkouts now reference the same branch: avoid concurrent branch/commit
  mutations until coordinating worktrees again.
- Latest native renamed build succeeded (38.35s); previously launched process
  34652 is the older runtime build and must close before manually reopening latest.
- Focused rerun: 12 passed / 1 failed (auth loopback transport), even with no_proxy.
  Do not claim all tests passed; full final suite and transport investigation pending.
- Manual instructions: docs/security/manual-validation.md. Generated workflow
  driver was not executed; native startup/migration only are runtime-verified.

---

# Runtime checkpoint — 2026-11-05, manual validation requested

Current continuation is uncommitted; latest committed baseline a766ace. No push.

- Actual native non-test build succeeded using an isolated renamed manifest and
  shared cache; original rustman.exe was not overwritten. Test process 34652
  launched with child-only RUSTMAN_DATA_DIR/TEMP/TMP under target/security-runtime-data-UUID.
- Windows reported Responding=True, a real Rustman window and ~188 MiB working set.
  This establishes startup, not visual correctness or absence of memory defects.
- Seeded synthetic legacy SQLite migrated using actual Windows native keychain:
  requests/data, environments/variables and session/data all have AES256GCM prefix.
  Synthetic token/password markers absent from DB, WAL and SHM after startup.
  No original user DB or credential values accessed.
- Native visual automation blocked: node_repl initialization failed, reset/retry
  failed, and CUA fallback failed, all with "failed to write kernel assets ...
  system cannot find the path specified (os error 3)". No visual pass claimed.
- User requested manual validation steps. End-to-end clicking Send/save/reopen,
  response masking and browser preview remains MANUAL/OPEN. Generated runtime
  workflow driver was not completed or executed; no Send runtime pass claimed.
- Three new red regressions confirmed (nested protected JSON, historical secret
  ancestor with clean HEAD, restart skipping preview destructors). Fixes implemented.
  Review identified schema/OID dedup bypass; scan now independently checks each
  recognized schema. Full final verification of this continuation still required.
- Focused suite initially 12 passed/1 failed due loopback fixture/network behavior.
  Deterministic auth HeaderMap passed; wire fixture now explicitly disables proxy
  to isolate local transport. Next result is target/security-publication-review-green.log.
- Runtime test DB owns two native credential entries. Retain while manual testing;
  cleanup of those exact UUID entries is pending after app closure. Do not sweep
  unrelated temp files or credential entries. No real data cleanup performed.

---

# Active continuation — publication and browser preview

Base checkpoint: a766ace. No push. New work is currently uncommitted.

- SEC-013: two synthetic red regressions FAILED as expected: writer leaks inline
  bearer into snapshot; commit accepts manually inserted inline cookie snapshot.
- SEC-029: extracted existing temp writer for an observable lifecycle seam;
  regression FAILED because plaintext file survives final owner drop. No browser
  opened or execution payload used.
- SEC-038 added: typed auth references do not expand at real send. Regression is
  queued/running in functions cell g6:297, log target/security-auth-reference-red.log.
- Plan: reject known inline credentials before shared Git writes/commit while
  preserving encrypted local values; allow variable references and resolve typed
  auth only at send. Label explicit user exports unmasked. Preview becomes a
  random/exclusive private file with shared RAII ownership, bounded retention and
  normal-lifecycle cleanup; crashes/browser-held locks cannot be guaranteed away.

Collect pending auth red result before modifying HTTP auth implementation. Prior
full-suite results below belong to a766ace, not these uncommitted changes.

---

# Latest verified checkpoint — 2026-11-05, UI/security continuation

Branch: `codex/security-production-hardening`; base development
`2525f0029b601ecfb115dd8d65396f4b21ce1aa6`. No push. Git history records local
checkpoint commits. Earlier pending/failed states below are historical.

**NOT production-ready. Real user database untouched.**

## Verified results

- Full default suite: **361 passed, 0 failed, 3 ignored**, exit 0.
- Feature-disabled suite: **337 passed, 0 failed, 3 ignored**, exit 0.
- Production `cargo check --offline`: exit 0, including actual native keychain
  and trusted migration-state implementations (not the unit-test substitute).
- Native credential-store smoke: **PASS** on Windows. Only unique
  `security-smoke-...` synthetic entries were created/read/deleted. Both key and
  trusted-state deletion were verified; no key values printed. Reproducer:
  `scripts/security/native-keyring-smoke`.
- Independent review: no critical/important introduced regressions in reviewed
  storage/Git/masking/WS/console changes. Existing reconnect-generation gap noted.
- `git diff --check` passed. Native visual QA and non-Windows runs remain OPEN.

## Fixed since the previous checkpoint

- Git actions are blocked before scheduling writes when local storage is
  unavailable; UI shows Git unavailable instead of actionable controls.
- Migration completion is recorded outside SQLite in native credential metadata;
  deleted format marker and mixed plaintext/ciphertext migration are rejected.
- Script diagnostic redaction covers response-only sensitive headers/cookies and
  newly scripted auth headers, preserving actual outgoing values.
- Unrelated oversized/binary JSON no longer blocks collection discovery/cleanup.
- Both folder scan paths count ignored entries before queue growth.
- History labels/tooltips mask URL userinfo, sensitive decoded query keys and
  fragments. Stored/wire URL remains original.
- Automatic snippet previews mask credential-named form/JSON fields and URL
  secrets. Explicit original-copy remains available, labelled unmasked.
- Large auth fields use bounded/masked previews rather than unbounded shaping.
- WS retained history: latest500 messages/1 MiB; per-message preview64 KiB with
  truncation indicator; service/subscription queues4. Incoming events retain
  source tab ID and ignore closed tabs.
- Console retained/rendered output:64 KiB/max500 records with line cap/truncation
  marker. These are retained-preview bounds, not whole-process/engine guarantees.

## Recorded exploration and remaining gates

Per-case record now contains SEC-001 through SEC-037 in
`docs/security/cases.md` and `cases.json`. Includes source-confirmed findings,
partial fixes, intentional edit/export boundaries and inconclusive review areas.
Do not call them 37 demonstrated vulnerabilities.

Important OPEN items: inline credentials in recognized Git/export snapshots
(SEC-013), signed-release pipeline/configuration, mutable CI action pins,
plain temporary browser-preview HTML (SEC-029), full snippet construction before
render cap (SEC-034), explicit environment/bulk/copy/raw-body secret policy,
script/global memory budgets and same-tab WS reconnect generation race.

SQLite uses AES-256-GCM for sensitive payloads, not SQLCipher/full-file encryption.
Metadata remains readable. Key recovery/backup, database replacement/rollback,
old backups/exports and same-user compromise are not solved by this boundary.
Actual Windows credential lifecycle passed; full isolated native-app migration
and visual QA, Linux/macOS credential integrations remain to verify.

RustSec report remains:2 vulnerability matches (quick-xml build dependency),
13 unmaintained entries and1 unsound warning. Reachability/limitations are in the
case register; no RCE or real OOM tests were performed.

Current full-test logs are under ignored target:
`security-full-verified.log`, `security-no-default-verified.log`,
`security-native-verified.log`, `security-native-keyring-smoke.log`.
Prior resource red log:18 passed/2 expected failures; UI red log:16 passed/2
expected failures. Redirect fixture reset was fixed with graceful half-close and
drain; redirect policy was unchanged and passes final suites.

Next audit pass: resolve OPEN publication/temp/preview-budget/reconnect issues,
complete native visual/cross-platform validation and release-signing workflow.
Do not merge/deploy or claim production readiness from this checkpoint.

---

## Historical checkpoints below (superseded by verified state above)

# Latest checkpoint — 2026-11-05, continued exploration

**NOT production-ready. No commit/push. Preserve uncommitted changes.**

User requested per-case tracking, masking/redaction even after decryption,
collection ID filesystem containment, and resource-exhaustion coverage.
Register is now `docs/security/cases.md` / `cases.json`: SEC-001 through SEC-026.
Each record has evidence, provisional severity, status and next verification step.

## Work completed and verified during continuation

- Dependency inventory captured: 766 locked packages, registry checksums present,
  two Git sources pinned to resolved commits. No confirmed typosquatting finding;
  this is NOT proof that all packages are trustworthy.
- Official RustSec 0.33.0 offline library scan succeeded after cargo-audit CLI
  compilation failed. Reproducer retained under `scripts/security/rustsec-offline`.
  Report: `docs/security/rustsec-report.json`, metadata/lock hash alongside it.
  Results: 2 vulnerability matches, 13 unmaintained warnings, 1 unsound warning.
- Both vulnerability matches are quick-xml 0.39.4 through wayland-scanner 0.31.10
  (build path), NOT calamine's runtime quick-xml (0.42.0). CPU-attribute path
  exists in scanner source; NsReader path for second advisory was NOT found.
  lru 0.16.4 is transitive through cryoglyph; its CacheKey is Copy, which does not
  support the advisory's panicking-Drop precondition. No RCE/OOM attack executed.
- Added log redaction for known tab/env cookies/auth values and test names/errors.
  Synthetic console disclosure test failed before fix and passed after fix;
  original request cookie is preserved for sending.
- Masked Bearer/API-key/JWT/cookie auth inputs and response cookie/sensitive
  headers. Added secure cells for manually entered sensitive request headers,
  including oversized values. Native visual verification still pending.
- Collection writer: strict portable ID allowlist; batch/duplicate/reserved-name
  validation before mutation; unrelated-file collision refusal; bounded regular
  reads; reject snapshot links; randomized exclusive staging/atomic rename;
  cleanup recognizes only owned snapshots.
- Git commits now build an isolated index from HEAD + recognized snapshots and
  preserve unrelated staging. First proved private.txt leaked into commit, then
  **3 VCS tests passed** after fixes, including traversal/duplicates/collisions.
- Regression run excluding then-open Git tests: **348 passed, 3 ignored**.
  A native non-test `cargo check --offline` completed successfully at that
  checkpoint. It does not exercise the actual Windows credential store.

## New review findings / current unresolved fixes

Independent read-only reviewer completed. These findings MUST remain visible:

- SEC-023 P1: db=None on storage/keychain failure still allows Git refresh to
  schedule empty snapshot writes/deletions. Guard all shared Git operations.
- SEC-024 P2: deleting unauthenticated format marker permits plaintext migration
  reinjection. Trusted migration-complete metadata needed; reject downgrades.
- SEC-025 P2: response-only Set-Cookie and newly scripted Authorization headers
  can bypass current redactor. Include HostInput/new effects without modifying
  actual request values.
- SEC-026 availability regression: unrelated oversized or non-UTF8 JSON blocks
  managed discovery. Failing fixture confirmed; preserve/ignore unrelated files.
- SEC-017 resource exhaustion: folder enumeration/queue bounds do not cover
  ignored entries. Added test with 2049 empty files, not a real OOM workload.
- SEC-013 inline credentials in recognized Git snapshots/export remains OPEN.
- Signing release configuration, CI immutable action pins, native keychain smoke,
  cross-platform checks, remaining secret display/copy/export surfaces remain
  release gates. Do not certify production readiness.

## Latest pending verification

functions exec cell `g6:247` runs all `security_` tests, log
`target/security-review-red.log`. It should demonstrate SEC-023/024/025/017 and
SEC-026 before fixes; collect its result before modifying those implementations.
Prior cell `g6:243` / log `target/security-unrelated-red.log` failed as expected
on unrelated invalid UTF-8 JSON. Prior containment command passed 3 VCS tests.
Some earlier async tool cells/sessions may need final polling; inspect logs.

Next: collect red regressions, fix review findings, run formatting + full tests +
non-test and feature-disabled checks, refresh register/progress, then commit only
verified changes. DO NOT PUSH. RCE testing remains excluded.

---

## Historical checkpoint follows (superseded by latest statuses above)

# Security audit checkpoint — 2026-11-05

## Status

**IN PROGRESS — NOT production-ready.** Checkpoint requested by user before the
5-hour usage limit. Do not claim fixes are complete from this checkpoint.
No security commit and no push have been made. Preserve all uncommitted changes.

Workspace: `C:\Users\YSHA-PC\.codex\worktrees\security-production-hardening\rustman`
Branch: `codex/security-production-hardening`
Base: freshly fetched `origin/development`, commit
`2525f0029b601ecfb115dd8d65396f4b21ce1aa6`.
Original checkout: `D:\Dev\Rust\postman-clone\rustman`; keep unrelated work intact.

## User decisions / authorization

- Audit and harden Rustman for a production security review; commit only, no push.
- Audit token disclosure, storage, HTTP/TLS, imports/filesystem, Git, updater,
  resource limits and relevant OSI boundaries. Use REA only if runtime/binary
  evidence is needed; it has not been needed/used yet.
- **Skip RCE-related testing.** No exploit execution or external attacks.
- Use synthetic credentials, temporary DB/repositories and loopback fixtures.
  Never inspect the real user database/keychain values or send real tokens.
- Unicode/multilingual inputs are relevant (Thai, Korean, Japanese `一`), but
  Unicode display text alone is not a vulnerability.
- Latest explicit clarification: plaintext concern is **database storage**, not
  masking the environment table UI. History must also be protected. User accepts
  AES-256-GCM. Do not invent a custom cipher or confuse encoding with encryption.
- User asked about idempotency keys: answered they are unnecessary for database
  encryption; useful for supported server APIs to avoid duplicate writes on retry.

## Confirmed baseline and red regressions

- Before security changes: full `cargo test --offline --quiet`: **323 passed,
  3 ignored**, exit 0 (326 total).
- Five transport regressions were executed before fixes: **0 passed, 5 failed**.
  1. Custom API-key header crossed origins during a redirect. Captured only a
     synthetic token between local loopback servers.
  2. Response body buffered without a hard streaming limit; a declared 1 GiB
     response did not trigger the intended limit before reading.
  3. DNS names such as `127.attacker.example` defaulted to HTTP due to prefix
     matching; all 172/8 incorrectly treated as private.
  4. JWT subject escaping mishandled backslashes and produced invalid JSON.
     This is NOT demonstrated claim injection; classify as auth correctness.
  5. Dropping WebSocket handle/receiver left a background reader/socket alive.
- Plaintext storage regression executed before wiring encryption: **failed as
  expected**, `plaintext leaked in environments.variables`.

## Changes in working tree

### Updater — focused tests passed

`src/services/update.rs` was hardened; actual worktree command
`cargo test --offline services::update::tests -- --test-threads=1`:
**24 passed, 0 failed**.

- Fail closed unless a build-time pinned `RUSTMAN_UPDATE_PUBLIC_KEY` exists.
- Verify minisign signature on SHA256SUMS and archive hash, with trusted comment
  exactly `rustman-release:{tag}` to bind the release version.
- Exact platform asset names, stable semver and repository/tag HTTPS validation;
  constrained redirects and downgrade refusal.
- Exclusive randomized staging; download/decompression/entry limits and deadlines.
- Synthetic regressions include tampering, Unicode/confusable archive paths,
  link/duplicate entries and native-header structure. No payload was executed.

**Release blocker:** the repository does not yet have a real pinned signing key
or signed release pipeline. Unsigned updater installation is intentionally
unavailable. Do not invent or commit private signing material. Signed release
integration and native replacement on Linux/macOS remain unverified.

### Transport — implementations written, verification pending

`src/services/http.rs`: standard serde JSON for JWT, parsed IP classification,
same-origin-only redirects (including port/scheme), connect deadline, 64 MiB
streaming response cap including Content-Length precheck. Invalid header errors
no longer interpolate the secret value.

`src/services/websocket.rs`: handshake deadline, message/frame limits, one
background task with abort-on-handle-drop and receiver-close handling.

Need verify positive same-origin redirects and chunked response limits, then all
existing request tests. User-selected private/local API destinations are expected
API-client behavior; do not label them SSRF merely because they are private.

### Database encryption — implementation written, NOT verified complete

Files: `src/services/keychain.rs`, new `src/services/storage/vault.rs`,
`src/services/storage.rs`, `storage/drafts.rs`, `storage/examples.rs`,
`src/app/boot.rs`, Cargo.toml/Cargo.lock.

Design implemented:
- RustCrypto `aes-gcm` 0.10.3, AES-256-GCM with random nonces and table/column/row
  ID associated data; versioned envelopes.
- `keyring` 3.6.3 native Windows/macOS/Secret Service features. Per-database random
  256-bit key in OS credential store, nonsecret UUID in `local_vault` metadata.
- Unit tests use ephemeral in-memory key provider, never user's credential store.
- Encrypt environment variables, saved request JSON, history URL + request JSON,
  session snapshot JSON, draft data/base, saved example JSON.
- IDs, names, timestamps, method/status and draft conflict/review metadata remain
  metadata; this is payload encryption, NOT SQLCipher/full SQLite encryption.
- Transactional legacy-row migration; `secure_delete`, WAL checkpoint and vacuum.
  Startup authenticates all protected rows before exposing writable connection.
- Key failure/corrupt ciphertext must fail closed; boot shows storage error and
  does not retain writable DB handle. No fallback to plaintext or replacement key.

Latest added tests: AEAD roundtrip/nonces/tampering/wrong key/row swapping;
legacy migration + corrupted ciphertext preservation; raw SQL/DB file scans;
reopen roundtrip for env/history/request. **These need execution and review.**
The `super::keychain` module-path compile error from the first attempt was fixed
by using `crate::services::keychain`.

Important follow-ups:
- Verify native keyring path in a NON-test build (`cargo check` at minimum): unit
  tests deliberately substitute key provider. Add an isolated native-store smoke
  test using a unique test credential and explicit cleanup if practical.
- Test unavailable/missing key preserving DB; do not silently recreate a key.
- Test migration rollback, session/draft/example roundtrips and all tests.
- Check resource bounds/startup migration scalability and secret error handling.
- Cross-platform credential stores/builds untested on this Windows host.
- Backups, old exports, filesystem snapshots, Git working trees/history are not
  retroactively protected. Same-user malware may access native credentials;
  at-rest encryption does not establish protection against a compromised account.

### Git/import — findings OPEN, only regression tests added

`src/services/vcs.rs` contains new `security_tests`; production logic unchanged.
- `write_working_tree` trusts collection.id as a filename => path traversal.
- Cleanup removes unrelated *.json files.
- `commit` stages `*`, including unrelated and previously staged secret files.
- Collection exports/Git serialize inline request credentials plaintext.
- Unbounded, link-following JSON reads remain to review/fix.

Tests use only bounded unique temporary roots, synthetic files and credentials.
Need run red tests and fix before claiming these closed. Suggested strategy:
strict portable filename IDs (support existing `c1`, not UUID-only), reject reserved
Windows names/global collision/duplicates before mutation, preserve unrelated
files, bounded regular-file reads/no links, and build commits from an explicit
allowlist without committing preexisting unrelated staged files. Add credential
publication guard/redaction plus local-preserving behavior; avoid silent data loss.

## Running / pending commands at checkpoint

Commands require escalation for shared D: target cache writes; normal root
worktree source writes work when explicit workdir is supplied. Relative
`apply_patch` stalled; it was terminated. Prefer explicit workdir/UTF-8 writes.
Python default Windows encoding can truncate a file before Unicode write failure;
`storage.rs` was restored from HEAD and its intended test re-added with UTF-8.
Always use explicit UTF-8 when editing.

- functions exec cell `g6:198`: runs
  `$env:CARGO_TARGET_DIR='D:\Dev\Rust\postman-clone\rustman\target'; cargo test --offline security -- --test-threads=1`
  Await result with functions.wait. It includes new VCS tests expected to remain
  red until Git fixes. Source may have changed while approval/build queued;
  rerun after final changes.
- exec session `84424`: `cargo install cargo-audit --version 0.22.2 --locked --root target/security-tools`
  still compiling at last poll. Use write_stdin to collect result.
- Old `cargo audit` 0.21.2 FAILED to parse current advisory database (CVSS 4.0,
  anchor-lang RUSTSEC-2026-0146). **No successful dependency audit yet**. Do not
  report zero vulnerabilities. After installation use the project-local
  `target/security-tools/bin/cargo-audit.exe audit --json --file Cargo.lock` and
  preserve/triage evidence. Advisory DB fetched under user's cargo cache.
- `cargo fetch` completed after adding AES/keyring; lockfile changed (39 packages,
  openssl-sys patch update); review and audit these additions.

## Next steps, in order

1. Collect pending test/build results; fix compilation/test failures.
2. Finish and verify encryption failure/migration boundaries, including native
   production keyring code and history/session/drafts/examples.
3. Finish Git/import fixes and credential publication protections.
4. Review fixes, run cargo fmt/check/full tests and no-default-features checks.
5. Complete dependency advisory audit with newer cargo-audit.
6. Write final evidence-based audit report, release gates and signing/key backup
   documentation. Do not certify OSI infrastructure or untested platforms.
7. Commit only verified changes on security branch; DO NOT PUSH.

## References consulted

- https://docs.rs/aes-gcm/0.10.3/aes_gcm/
- https://docs.rs/keyring/3.6.3/keyring/
- https://github.com/RustSec/advisory-db
- https://github.com/rustsec/rustsec/releases
- https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html

Durable design/plan: `docs/security/2026-11-05-hardening-plan.md`.
## Final checkpoint result collected

The pending `cargo test --offline security -- --test-threads=1` completed:
**7 passed, 2 failed** (343 filtered out), exit 1.

Passed: all five transport regressions (HTTP redirect, JWT JSON, response size,
URL defaults, WebSocket disposal), plus both storage regressions (encrypted
roundtrip/raw file scan/reopen and legacy migration/ciphertext preservation).

Failed as expected, still OPEN: VCS rejects unsafe IDs; VCS preserves unrelated
JSON files/staging. The first failure in the latter test happens at unrelated-file
preservation, so the later staging assertions have not yet executed.

The separate `storage::vault::tests::authenticated_roundtrip_nonce_and_context`
was NOT selected by the `security` name filter. Run it explicitly or in the full
suite. No full post-change regression suite or production/native keyring build
has been verified yet. Tests use synthetic key provider; native OS storage is
still a verification gate. Pending test cell g6:198/session57668 is now completed.

## Continued exploration

Per-case register: `docs/security/cases.md` and `cases.json` (SEC-001 through SEC-018). Dependency inventory saved. cargo-audit 0.22.2 installation FAILED in aws-lc-sys compilation; no success report yet.

User added explicit SEC-018 requirement: decoded cookie/token values must remain masked/redacted in ordinary UI and diagnostics. AES storage alone is insufficient; trace snippet/clipboard/export separately. Actual request dispatch still needs originals in memory. Do not claim memory is never plaintext.

Latest tool handoff: g6:247 yielded exec session `60241`; poll with write_stdin.
At last poll compilation had emitted normal warnings and test results were not yet
available. Do not assume that pending red regressions have passed. `git diff
--check` completed without whitespace errors at this checkpoint.
