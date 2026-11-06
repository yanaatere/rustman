# Rustman security case register

Updated: 2026-11-06. Cases/review areas, **not a count of proven vulnerabilities**.
Production readiness is NOT certified. RCE testing excluded. Synthetic fixtures only.

| Case | Finding | Status |
|---|---|---|
| [SEC-001](#sec-001) | Plaintext local secret payloads | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-002](#sec-002) | API-key disclosure across redirect origins | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-003](#sec-003) | Public DNS names default to plaintext HTTP | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-004](#sec-004) | Unbounded HTTP response buffering | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-005](#sec-005) | WebSocket task/socket survives disposal | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-006](#sec-006) | Invalid JWT JSON for escaped subjects | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-007](#sec-007) | Updater lacks enforced release authenticity | Code fixed; release configuration OPEN |
| [SEC-008](#sec-008) | Predictable updater staging path | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-009](#sec-009) | Updater archive/download resource abuse | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-010](#sec-010) | Collection ID used as filesystem path | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-011](#sec-011) | Cleanup deletes unrelated JSON files | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-012](#sec-012) | Broad Git staging includes unrelated secrets | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-013](#sec-013) | Inline request credentials serialized to Git/export | Known credential Git guard implemented; native rejection verified; final full suite pending |
| [SEC-014](#sec-014) | Mutable GitHub Action references | Source-confirmed exposure; no compromise demonstrated |
| [SEC-015](#sec-015) | Dependency identity / typosquatting review | Inventory only; INCONCLUSIVE |
| [SEC-016](#sec-016) | Dependency advisory audit tooling unavailable | Alternative RustSec scan completed; CLI install remains unavailable |
| [SEC-017](#sec-017) | Import scan queues entries before bounding traversal | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-018](#sec-018) | Secrets visible after decryption / missing output redaction | Known auth/cookie display and diagnostic paths hardened; broader output policy remains OPEN |
| [SEC-019](#sec-019) | quick-xml duplicate-attribute CPU exhaustion advisory | Lockfile match confirmed; OPEN |
| [SEC-020](#sec-020) | quick-xml namespace allocation exhaustion advisory | Dependency match retained; affected API not found in identified consumer |
| [SEC-021](#sec-021) | Transitive lru panic-safety unsoundness advisory | Dependency warning retained; documented trigger not supported by identified key type |
| [SEC-022](#sec-022) | Unmaintained dependency notices | 13 report entries captured; triage pending |
| [SEC-023](#sec-023) | Unavailable database can trigger empty Git snapshot writes | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-024](#sec-024) | Deleted migration marker can authorize plaintext reinjection | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-025](#sec-025) | Response-only and newly scripted auth secrets bypass log redaction | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-026](#sec-026) | Bounded managed-file reader rejects unrelated oversized/binary JSON | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-027](#sec-027) | Masked snippet preview and unmasked clipboard action differ | Label clarified; original-value copy remains explicit |
| [SEC-028](#sec-028) | Bulk header editor reveals secure-cell values | Explicit editing boundary; label clarified |
| [SEC-029](#sec-029) | Browser preview leaves plaintext response in temporary HTML | Native normal-close preview cleanup verified; crash/lock limits remain |
| [SEC-030](#sec-030) | Large auth input enters unbounded text shaping | Bounded/masked guard compiled and regression suite passed; native visual/timing check OPEN |
| [SEC-031](#sec-031) | History URL and hover tooltip expose query/userinfo secrets | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-032](#sec-032) | Automatic snippet preview exposes password form fields | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-033](#sec-033) | Environment editor displays unmasked variable values | Source observed; policy decision remains |
| [SEC-034](#sec-034) | Snippet display cap applies after full generation | Source-confirmed processing path; OPEN |
| [SEC-035](#sec-035) | WebSocket UI history and event buffers retain excessive data | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-036](#sec-036) | Script console renders unbounded accumulated output | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-037](#sec-037) | WebSocket events update active tab instead of source tab | Fix verified on Windows; see case limitations and remaining gates |
| [SEC-038](#sec-038) | Typed auth references are not substituted at actual send | Native bearer-reference send verified; remaining typed-auth fixture investigation OPEN |
| [SEC-039](#sec-039) | Git publication rejection lacks visible notification | Regression and native visual QA passed; local uncommitted fix |
| [QA-001](#qa-001) | History entry selection can reuse an already open request | Observed during native QA; dedicated reproduction pending |
| [SEC-040](#sec-040) | Saved example header editor exposes decrypted credentials | Native masking/reopen and temporary reveal verified; focused tests passed |
| [SEC-041](#sec-041) | Temporary secret reveal isolation and lifecycle | Focused unit and native visual checks passed |

## SEC-001

Plaintext local secret payloads

- Severity: High
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/storage.rs; src/services/storage/vault.rs; src/services/keychain.rs`

Pre-fix test exposed environment plaintext. Post-fix env/history/raw disk/reopen and migration/tamper tests passed. AES-GCM payload encryption is implemented, not full-database encryption. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes. Native production keychain-module smoke check also PASSED on Windows: create/read 256-bit synthetic key, trusted migration-state roundtrip, and deletion verified. No real credential values or user DB accessed. Native GUI QA 2026-11-06: actual Windows build sent synthetic POSTs, saved, closed normally and restored session/history. Requests/environment/session/history URL and request JSON all remained AES256GCM payloads; synthetic credential markers absent from SQLite/WAL/SHM. No real DB accessed.

Next: Native full-app isolated UI smoke, recovery/backup UX, cross-platform keystores/permissions, whole-file replacement/rollback limits, plaintext Git/export/temp artifacts remain outside this verified payload-encryption boundary.

## SEC-002

API-key disclosure across redirect origins

- Severity: High
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/http.rs`

Synthetic header was observed at a different loopback origin before fix. Same-origin-only redirect policy now passes this regression. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Positive same-origin redirect and downgrade/port matrix; full requester suite.

## SEC-003

Public DNS names default to plaintext HTTP

- Severity: High
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/http.rs::resolve_url`

String-prefix matching treated 127.attacker.example and public 172 addresses as private. Parsed IP classification regression now passes. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Expand Unicode/authority/IPv6/explicit-scheme cases; do not block intentional local APIs.

## SEC-004

Unbounded HTTP response buffering

- Severity: Medium
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/http.rs`

Pre-fix Content-Length limit test failed. 64 MiB cap now checks header and streamed chunks. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Chunked response regression; verify limits on decoded content and preview processing.

## SEC-005

WebSocket task/socket survives disposal

- Severity: Medium
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/websocket.rs`

Pre-fix disposal test failed; post-fix test passed with abort-on-handle-drop. Handshake/frame/message limits added. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Handshake stall, oversized messages and queued-event cancellation tests.

## SEC-006

Invalid JWT JSON for escaped subjects

- Severity: Low
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/http.rs::make_jwt`

Backslash/quote/multilingual subject produced invalid signed JSON. serde_json serialization passes regression. Claim injection was not demonstrated. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Full auth tests.

## SEC-007

Updater lacks enforced release authenticity

- Severity: High
- Status: Code fixed; release configuration OPEN
- Source: `src/services/update.rs; .github/workflows/release.yml`

24 focused updater tests passed. Install now refuses missing pinned public key and verifies signed checksums/hash/version binding. Existing workflow has no signing setup.

Next: Configure real release public key, offline/protected signing procedure and signed integration fixtures. Never commit private key.

## SEC-008

Predictable updater staging path

- Severity: Medium
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/update.rs`

Fixed staging filename replaced with randomized exclusive staging and cleanup. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Cross-platform filesystem verification.

## SEC-009

Updater archive/download resource abuse

- Severity: Medium
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/update.rs`

Archive, executable, metadata, entry-count and deadline limits added. Strict exact asset/version/URL checks. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Signed release integration and native replacement on all supported platforms.

## SEC-010

Collection ID used as filesystem path

- Severity: High
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/vcs.rs::write_working_tree`

Portable ASCII ID allowlist rejects traversal, separators, absolute/drive/UNC paths, dots, encoded traversal, reserved Windows names, NUL and Unicode/confusable IDs. Full batch validation rejects case-insensitive duplicates before writes. VCS containment/staging suite: 3 passed. Unicode display names remain supported. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Maintain regression matrix; final full suite after review fixes. Snapshot destinations reject links/non-regular files, writes use exclusive randomized staging and rename.

## SEC-011

Cleanup deletes unrelated JSON files

- Severity: Medium
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/vcs.rs::write_working_tree`

Writer deletes only recognized collection snapshots whose IDs match filenames, preserves unrelated JSON, validates destination collisions and refuses overwrites. Three focused VCS tests passed. Review regression SEC-026 remains to repair for oversized/binary unrelated JSON. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Resolve SEC-026 and verify again; do not claim all unrelated-file cases are closed yet.

## SEC-012

Broad Git staging includes unrelated secrets

- Severity: High
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/vcs.rs::commit`

After cleanup fix, isolated regression demonstrated private.txt entered a commit from preexisting staging. Commit now builds an independent index from HEAD plus recognized snapshots, preserving unrelated staging. Post-fix VCS suite: 3 passed. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Add retained HEAD/deletion/conflict edge tests and complete final full suite. SEC-013 inline credentials in recognized snapshots is separate and still OPEN.

## SEC-013

Inline request credentials serialized to Git/export

- Severity: High
- Status: Known credential Git guard implemented; native rejection verified; final full suite pending
- Source: `src/services/vcs.rs; src/services/import/mod.rs::native_export`

SavedRequest includes bearer/basic/API-key/cookie/JWT values; full clones are serialized without redaction. AES protection of SQLite does not protect these files. Continuation: synthetic inline bearer snapshot-write test and manually inserted cookie snapshot commit test both FAILED before fix. Local values must remain unchanged; known inline shared credentials will be rejected before write/index/commit. Explicit user exports remain separate unmasked actions. Synthetic red writer/commit/history tests proved inline credential publication and clean-HEAD historical leakage. Writer/commit now reject known inline credential fields, including inactive fields and nested protected JSON; bounded push preflight checks reachable managed snapshots before network. Native Git Refresh rejected dummy password body; no JSON snapshots created, no remote configured or push executed. Explicit user exports remain lossless/unmasked and labelled. Unknown/encoded/script literals are not universally detected.

Next: Separate explicit export semantics from shared Git publication; guard/redact with local data preservation and tests.

## SEC-014

Mutable GitHub Action references

- Severity: Medium hardening
- Status: Source-confirmed exposure; no compromise demonstrated
- Source: `.github/workflows/ci-iced.yml; .github/workflows/release.yml`

checkout@v4, rust-toolchain@stable, rust-cache@v2 and action-gh-release@v2 are mutable refs; release jobs have contents:write.

Next: Resolve reviewed upstream commits, pin full SHAs, restrict workflow permissions and use locked dependency builds.

## SEC-015

Dependency identity / typosquatting review

- Severity: Unrated
- Status: Inventory only; INCONCLUSIVE
- Source: `Cargo.toml; Cargo.lock`

766 locked packages, registry checksums present, no non-ASCII package names; two Git sources under rust-man-org locked to exact commit. These facts do not establish trusted ownership or absence of malicious code.

Next: Capture inventory; verify direct crate identities and Git commit provenance; review transitives/build scripts and registry policy.

## SEC-016

Dependency advisory audit tooling unavailable

- Severity: Release verification gate
- Status: Alternative RustSec scan completed; CLI install remains unavailable
- Source: `cargo-audit toolchain`

0.21.2 cannot parse CVSS 4.0 advisory. Installing 0.22.2 failed in aws-lc-sys native compilation. No successful current advisory report yet. Official RustSec 0.33.0 library with default features disabled successfully scanned the local advisory DB and Cargo.lock; report saved in rustsec-report.json: 2 vulnerability matches, 13 unmaintained warnings, 1 unsound warning.

Next: Preserve reproducible helper source/report and triage dependency reachability. Do not describe the report as a clean audit.

## SEC-017

Import scan queues entries before bounding traversal

- Severity: Medium availability candidate
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/import/resources.rs::collect_files`

visited > 2048 is checked on pop, but read_dir pushes arbitrary eligible children without pending-size check. File sizes are capped; directory enumeration/queue may exceed intended entry cap. Both folder readers can enumerate arbitrarily many ignored files without counting them toward entry limits. A bounded fixture with 2049 empty ignored files was added; result pending. Red test failed on 2049 empty ignored files. Both scan paths now count discovered entries including ignored children before enqueueing; test passed. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Complete final full/default-disabled/native checks and cross-platform/native UI review; retain case-specific limitations above.

## SEC-018

Secrets visible after decryption / missing output redaction

- Severity: High disclosure boundary
- Status: Known auth/cookie display and diagnostic paths hardened; broader output policy remains OPEN
- Source: `Cookie/auth/environment UI; snippet; clipboard; logs; export`

Pre-fix console test leaked synthetic cookie/bearer. Post-fix exact-known-value redaction passed while original request cookie remained unchanged. Auth inputs now secure=true; response cookie/sensitive-header views mask values; manually entered sensitive request headers use secure cells with no plaintext oversized preview. Regression run excluding then-open Git tests: 348 passed, 3 ignored. Review found response-only and newly scripted header values need additional handling (SEC-025). Clipboard/export/bulk/raw response paths are not declared closed.

Next: Resolve SEC-025, review explicit reveal/copy/export/bulk/raw body boundaries and add native visual verification. Redaction is known-value/name-based, not a guarantee against arbitrary encoded or split secrets.

## SEC-019

quick-xml duplicate-attribute CPU exhaustion advisory

- Severity: High upstream; Rustman reachability limited to build path identified
- Status: Lockfile match confirmed; OPEN
- Source: `Cargo.lock: quick-xml 0.39.4 <- wayland-scanner 0.31.10`

Official RustSec report matched RUSTSEC-2026-0194; fixed >=0.41.0. Current calamine uses quick-xml 0.42.0 and plist uses 0.41.0. Runtime response exploitation is not established. No exhaustion payload run.

Next: Review upstream wayland-scanner upgrade/backport and build-time XML trust boundaries; preserve full report.

## SEC-020

quick-xml namespace allocation exhaustion advisory

- Severity: High upstream; application exploitability not demonstrated
- Status: Dependency match retained; affected API not found in identified consumer
- Source: `Cargo.lock: quick-xml 0.39.4 <- wayland-scanner 0.31.10`

RustSec RUSTSEC-2026-0195 requires NsReader/NamespaceResolver paths. Dependency presence alone does not prove that wayland-scanner invokes that affected API. Fixed >=0.41.0. No OOM test run. Source inspection: wayland-scanner parse.rs imports plain Reader; no NsReader or NamespaceResolver use found. Thus the identified consumer does not establish the advisory-specific affected path.

Next: Inspect actual quick-xml APIs used by wayland-scanner; triage build-only/reachability separately from presence.

## SEC-021

Transitive lru panic-safety unsoundness advisory

- Severity: Informational unsound; prerequisites require review
- Status: Dependency warning retained; documented trigger not supported by identified key type
- Source: `Cargo.lock: lru 0.16.4 <- cryoglyph 0.1.0`

RUSTSEC-2026-0253 warning requires unwinding/catch_unwind and a key with potentially panicking Drop. Rustman direct lru is 0.18.5; release profile panic=abort. Do not claim RCE or demonstrated memory corruption in Rustman. cryoglyph uses LruCache<cosmic_text::CacheKey,...>; cosmic-text 0.15.0 CacheKey derives Copy. This key cannot implement the panicking Drop required by the advisory. No runtime exploitability established.

Next: Review cryoglyph key/drop usage and upstream replacement/upgrade. RCE tests remain excluded.

## SEC-022

Unmaintained dependency notices

- Severity: Maintenance risk, not proof of exploitable vulnerability
- Status: 13 report entries captured; triage pending
- Source: `docs/security/rustsec-report.json`

Notices include bincode, paste, pin-utils, rustybuzz, ttf-parser, unsafe-libyaml, yaml-rust, zbus_names and zvariant families. Some notices repeat across versions. Includes dependencies introduced by native-keyring integration.

Next: Map platform/runtime/build reachability and upstream migrations; do not count notices as 13 proven security exploits.

## SEC-023

Unavailable database can trigger empty Git snapshot writes

- Severity: High data loss
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/app/boot.rs; src/app/update/git.rs::refresh_task/load_view/Commit`

Boot failure (including missing/locked OS keychain) sets db=None and empty state; Git refresh still writes that empty collection list/global script pair. The managed writer can remove real snapshots. This was possible for baseline SQLite errors; keychain dependency adds a normal failure trigger. New test asserts no Git task is scheduled when db=None; result pending. Red test scheduled one task when db=None; handler now returns no tasks and preserves commit input. UI hides Git controls while storage unavailable; regression passed. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Complete final full/default-disabled/native checks and cross-platform/native UI review; retain case-specific limitations above.

## SEC-024

Deleted migration marker can authorize plaintext reinjection

- Severity: Medium integrity
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/storage/vault.rs::migrate`

Migration trusts absence of local_vault.format. Removing that row and inserting plaintext into an already encrypted DB causes it to be encrypted and accepted with legitimate key. New regression requires failure; result pending. Red downgrade test failed as predicted. Completion flag now resides in separate OS credential metadata; missing DB format after completion and mixed legacy states are rejected. Synthetic-key regression passed. This does not authenticate the entire database or prevent whole-file replacement/rollback; native OS metadata smoke test remains open. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Per-payload authentication and trusted migration marker are verified; this is not whole-database rollback/replacement protection. Native marker lifecycle passed on Windows; cross-platform integration remains.

## SEC-025

Response-only and newly scripted auth secrets bypass log redaction

- Severity: Medium disclosure
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/app/scripting.rs::run_and_apply/apply_effects`

Current redactor gathers tab/env secrets only. Response-only Set-Cookie values passed in HostInput and new SetHeader Authorization values are not included; logs can expose them. New synthetic regression added; result pending. Red test printed response-only cookie. Redactor now collects sensitive HostInput header/cookie values and new sensitive SetHeader effects. Regression passed while actual extra header remains original. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Complete final full/default-disabled/native checks and cross-platform/native UI review; retain case-specific limitations above.

## SEC-026

Bounded managed-file reader rejects unrelated oversized/binary JSON

- Severity: Medium availability regression
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/services/vcs.rs::managed_collection`

Independent review found eager reading before ownership recognition. Added bounded test with sparse 32 MiB+1 unrelated JSON and 2-byte non-UTF8 JSON; test failed on UTF-8 read. These files must be preserved and ignored rather than blocking valid collection operations. Managed discovery now reads bounded bytes and ignores oversized/invalid JSON. Explicit destination collision refusal remains. Regression passed. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Complete final full/default-disabled/native checks and cross-platform/native UI review; retain case-specific limitations above.

## SEC-027

Masked snippet preview and unmasked clipboard action differ

- Severity: Low UX disclosure boundary
- Status: Label clarified; original-value copy remains explicit
- Source: `src/ui/snippet_panel.rs; src/app/update/mod.rs; src/ui/request/header.rs`

Panel Copy resolves real environment/credential values while preview is masked. Tooltip/status now explicitly say unmasked; cURL export label likewise. Explicit copy is an intentional secret export, not an encryption bypass.

Next: Consider a separate copy-redacted action; review clipboard retention policy without silently breaking runnable exports.

## SEC-028

Bulk header editor reveals secure-cell values

- Severity: Low UX disclosure boundary
- Status: Explicit editing boundary; label clarified
- Source: `src/ui/request/headers.rs; src/app/update/request.rs::HeadersBulkToggle`

Header table masks known sensitive cells, but explicit Bulk Edit serializes originals to editor. Label now says Bulk Edit (unmasked). Request values remain intact.

Next: Native visual verification; decide whether a separate reveal control is required for this explicit edit surface.

## SEC-029

Browser preview leaves plaintext response in temporary HTML

- Severity: Medium confidentiality
- Status: Native normal-close preview cleanup verified; crash/lock limits remain
- Source: `src/app/update/response.rs::OpenBodyInBrowser`

Explicit browser preview writes full response to predictable PID/time-named temp HTML with no lifecycle cleanup. It can retain response secrets outside encrypted SQLite. No browser execution or execution payload test performed. Extracted existing preview writer without changing its behavior. Test proved plaintext file remains after final owner drops. No browser was opened. Planned fix uses exclusive random private files, bounded ownership retention and cleanup on normal lifecycle; crashes/OS file locks remain limitations. Native GUI 2026-11-06 opened 121-byte static HTML without scripts/resources via response menu. Owned private UUID preview file existed while app ran and was absent after normal close. Restart registry coordinates writes and explicit pre-exit cleanup; updater process restart not executed in GUI. Crash/power loss/browser caches and Windows locked-file cases remain outside normal-close proof.

Next: Use exclusive private temporary storage, track/clean lifecycle and make unmasked browser export explicit. Do not treat SQLite encryption as protecting these files.

## SEC-030

Large auth input enters unbounded text shaping

- Severity: Medium availability candidate
- Status: Bounded/masked guard compiled and regression suite passed; native visual/timing check OPEN
- Source: `src/ui/request/auth.rs::input`

All auth values entered text_input directly even when large/multiline. Input now uses existing INLINE_LIMIT policy; large secure values render only mask plus clear action; original state preserved. No real OOM or GUI stress test run.

Next: Native visual check and small-limit model verification; confirm no secret hover/preview path.

## SEC-031

History URL and hover tooltip expose query/userinfo secrets

- Severity: Medium confidentiality
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/ui/sidebar/history.rs; src/services/secrets.rs::display_url`

History label/tooltip formerly received raw entry.url. Synthetic regression failed. Presentation now masks userinfo, sensitive decoded query keys and fragments, bounds long URLs, and preserves stored/wire URL. Test passed including percent-encoded key and templated URL. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Full regression and native hover verification; sensitive-name heuristic is not a universal URL secret detector.

## SEC-032

Automatic snippet preview exposes password form fields

- Severity: Medium confidentiality
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/app/snippet.rs::resolved_input; src/services/secrets.rs::redact_json_fields`

Preview masked auth fields but not password URLencoded/multipart fields. Synthetic regression failed. Preview now masks credential-named form fields, URL secrets, and recognized JSON fields. Explicit unmasked copy and original request remain unchanged. Form regression passed. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Expand JSON/GraphQL/opaque-body tests. Unknown or encoded secrets cannot be universally inferred.

## SEC-033

Environment editor displays unmasked variable values

- Severity: Privacy/editing boundary
- Status: Source observed; policy decision remains
- Source: `src/ui/environment_workspace.rs; src/ui/components/atoms/table_input.rs`

Dedicated environment editing table accepts/shows original values, unlike sensitive-name URL popup preview. User clarified initial concern was storage, not blanket UI masking. Do not label intended direct editing as an exploit without a secret/reveal policy.

Next: Consider explicit secret metadata/reveal mode; do not erase values or replace stored originals with masks.

## SEC-034

Snippet display cap applies after full generation

- Severity: Medium availability candidate
- Status: Source-confirmed processing path; OPEN
- Source: `src/app/snippet.rs::refresh/curl_input; src/app/update/mod.rs`

Preview copies/normalizes full editor body and generates snippet synchronously before clipping display to 64 KiB. Rendering cap alone does not bound CPU/temporary allocations for large imported inputs. No destructive stress test run.

Next: Bound preview construction before parsing/generation or add cancellable single-flight background work with revision checks; test small thresholds.

## SEC-035

WebSocket UI history and event buffers retain excessive data

- Severity: Medium availability
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/state/tabs.rs::WsState; src/services/websocket.rs; src/app/subscription.rs`

1100 small synthetic messages exceeded intended retention; baseline had no history cap. Now latest 500 messages/1 MiB retained, per-message preview64 KiB with explicit truncation, service/subscription event channels4. Wire message cap remains8 MiB. No OOM workload run. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Retention and source-tab tests passed. Per-frame/per-connection preview bounds do not certify global runtime queue/process memory. Same-tab connection epochs remain separate.

## SEC-036

Script console renders unbounded accumulated output

- Severity: Medium availability
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/state/tabs.rs::set_script_logs/extend_script_logs; src/ui/layout/console.rs`

A 200 KB synthetic log entered editor without bound; regression failed. Visible console now retains64 KiB/max500 records with line cap and truncation marker, after secret redaction. No GUI freeze/OOM triggered. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Console retention tests passed. This caps retained/rendered output, not allocations inside script engine or complete process memory. Native visual/CPU benchmarking remains separate.

## SEC-037

WebSocket events update active tab instead of source tab

- Severity: Medium state isolation
- Status: Fix verified on Windows; see case limitations and remaining gates
- Source: `src/message.rs::WsMsg; src/app/subscription.rs; src/app/update/ws.rs`

Handshake carried tab ID, but text/disconnect/error messages lost identity and mutated active_tab_mut. Events now carry source tab ID and ignore closed tabs. New regression checks inactive source routing and closed source. Final checks: default suite 361 passed/3 ignored; feature-disabled suite 337 passed/3 ignored; production cargo check passed. Independent read-only review found no critical/important introduced regressions in reviewed changes.

Next: Tab routing/closed-tab tests passed. Same-tab reconnect generation race remains OPEN: tab ID alone cannot distinguish old/new connections.

## SEC-038

Typed auth references are not substituted at actual send

- Severity: Medium auth correctness / safe-sharing boundary
- Status: Native bearer-reference send verified; remaining typed-auth fixture investigation OPEN
- Source: `src/services/http.rs::do_send`

Bearer, Basic, API-key and cookie auth fields are used literally, unlike ordinary request headers. This can force users back to inline credentials when trying to share variable-backed requests. New loopback regression checks resolved wire values and unchanged template state; no real credentials. Deterministic HeaderMap red proved literal placeholder then passed after substitution fix. Native GUI sent literal token and {{TOKEN}} reference to owned loopback server; both received exact dummy bearer, JSON password, and Unicode text. Production Send/state/history flow worked. Latest combined fixture run remains 12 passed/1 loopback failed even with no_proxy; do not report all automated wire cases passed.

Next: Resolve all typed auth/query values with the active environment at send only; verify safe references work without mutating stored templates.

## SEC-039

Git publication rejection lacks visible notification

- Severity: Medium security feedback / UX
- Status: Regression and native visual QA passed; local uncommitted fix
- Source: `src/app/update/git.rs; src/app/notifications.rs; src/ui/notification.rs`

Native QA showed only tiny bottom status text for blocked credential publication; user could not see a notification. Regression failed with missing notification. Added persistent dismissible error toast with static credential-free guidance and existing semantic error palette; focused regression passed 1/1. Also uses error presentation for existing storage-failure toast. Native rebuilt app visibly showed red error icon, wrapped static guidance, persisted beyond eight seconds, dismissed correctly and reappeared on refresh. Screenshot E01/E02 and SHA manifest archived under docs/security/evidence/2026-11-06. Related Git tests 4 passed, 0 failed.

Next: Keep broader final audit checks separate; do not claim production-ready. Native build includes fix; no push.

## QA-001

History entry selection can reuse an already open request

- Severity: Functional correctness candidate
- Status: Observed during native QA; dedicated reproduction pending
- Source: `src/state/tabs.rs::TabManager::open_request; src/app/update/sidebar.rs`

After reopening a seeded request whose tab ID matched the saved request ID, clicking older /echo history left current /html request active. open_request deduplicates using request ID. Synthetic seed may make collision more likely; not labelled a proven general security exploit.

Next: Reproduce with ordinary newly created tabs and independent snapshot IDs before deciding whether history must always open an isolated tab.

## SEC-040

Saved example header editor exposes decrypted credentials

- Severity: Medium confidentiality / UI
- Status: Native masking/reopen and temporary reveal verified; focused tests passed
- Source: `src/ui/examples.rs::table`

Real native Save response -> example -> response Headers displayed full synthetic Set-Cookie and X-API-Key, unlike masked normal response viewer. Screenshot E09 records failing behavior with fake values only. Request/response/param value inputs now use sensitive-key secure presentation; original values remain in model and encrypted example payload. Raw body editor policy is unchanged. E10/E11 show masked saved-example credentials after reopen. E12/E14 reveal only selected field; E13 Save hides; E15 second click hides; E16 panel change hides. Example group16 passed: no outgoing task/client initialization, identical ciphertext during visibility changes, unchanged parent/templates, invalid indices, row-index reuse/reset. See evidence/2026-11-06/test-cases.md.

Next: Full audit gates remain open. Eye is presentation only; full payload already decrypted at load. No universal body masking/decrypt-on-click claim.

## SEC-041

Temporary secret reveal isolation and lifecycle

- Severity: Security control verification area, not a new proven vulnerability
- Status: Focused unit and native visual checks passed
- Source: `src/message/example.rs; src/state/examples.rs; src/app/update/examples.rs; src/ui/examples.rs`

User-requested per-field eye. EYE-001 through EYE-012 map real screenshots E11-E16 to executed tests. Transient visibility resets on Save/panel/key/row/open. Selected side/index only; no task or HTTP client; exact ciphertext/parent unchanged. Group16 passed, not16 proven vulnerabilities.

Next: Keyboard QA and wider surfaces remain separate. True lazy per-field decryption is not implemented.
