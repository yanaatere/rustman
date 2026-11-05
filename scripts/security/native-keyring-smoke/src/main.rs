//! Explicit native-store smoke check; only unique synthetic entries are accessed.
#[path = "../../../../src/services/keychain.rs"]
mod keychain;

struct Cleanup(String);
impl Drop for Cleanup {
    fn drop(&mut self) {
        for service in ["org.rustman.database.v1", "org.rustman.database.migration.v1"] {
            if let Ok(entry) = keyring::Entry::new(service, &self.0) {
                match entry.delete_credential() {
                    Ok(()) | Err(keyring::Error::NoEntry) => {},
                    Err(_) => eprintln!("Could not clean synthetic credential: {}", self.0),
                }
            }
        }
    }
}
fn main() -> Result<(), String> {
    // This prefix cannot be a production database UUID, even in a collision.
    let id = format!("security-smoke-{}", uuid::Uuid::new_v4());
    let cleanup = Cleanup(id.clone());
    if keychain::key(&id, false).is_ok() { return Err("Synthetic entry unexpectedly existed".into()); }
    let first = keychain::key(&id, true)?;
    if first.len() != 32 { return Err("Key length mismatch".into()); }
    let second = keychain::key(&id, false)?;
    if first != second { return Err("Native key roundtrip failed".into()); }
    if keychain::migration_complete(&id)? { return Err("Unexpected migration state".into()); }
    keychain::mark_migrated(&id)?;
    if !keychain::migration_complete(&id)? { return Err("Migration state roundtrip failed".into()); }
    drop(cleanup);
    if keychain::key(&id, false).is_ok() { return Err("Synthetic key was not deleted".into()); }
    if keychain::migration_complete(&id)? { return Err("Synthetic state was not deleted".into()); }
    println!("PASS: native 256-bit key and migration-state lifecycle; synthetic entries deleted; no key values printed");
    Ok(())
}
