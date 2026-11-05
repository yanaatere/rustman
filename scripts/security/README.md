# Local security verification tools

- `native-keyring-smoke`: compiles the production native-keychain module; checks
  a unique synthetic 256-bit key and trusted migration-state lifecycle, then
  verifies both synthetic entries were deleted. It does not read a user database
  or production key IDs. Do not print keys or replace its `security-smoke-` prefix.
- `rustsec-offline`: official RustSec 0.33.0 lockfile report generator without
  network/Git transport. Supply a current, already-fetched advisory database.
  Exit1 means vulnerability matches; no ignores are configured.

See `docs/security/README.md` for commands, evidence boundaries and remaining gates.
