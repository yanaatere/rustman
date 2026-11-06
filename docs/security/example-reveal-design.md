# Example secret visibility — 2026-11-06

User confirmed a per-field eye button for debugging, with masked values by
default and a clear hover/focus affordance. Scope is the saved-example request
headers, response headers and parameter value table.

Reuse native icon font, tooltip atom and existing neutral palette. Reserve a
small eye column so showing the control does not shift values. Sensitive fields
have an eye / crossed-eye button; ordinary fields remain plain editable values.
The button changes only temporary UI state, never the stored string. Save,
panel changes, key/row changes and opening a different example clear reveal.
A revealed value is the original editable field, so local edits remain intact;
no stale DB reload overwrites them. Reveal does not mark an example dirty.

Database boundary: full example payload is already decrypted while loading.
This design is visibility control, not lazy per-field decryption. Achieving
true decrypt-on-click would require a separate storage/model redesign. Env
references are never replaced by resolved literals merely because of reveal.
This does not add a read-only environment-resolved preview for every field.

Validation: real pre-fix screenshot E09, default-masked native reopen E10;
reveal regression failed before implementation and example group14 passed after.
Native E11-E16 verified mask/reveal/hide/Save/panel behavior. Expanded security rerun:16 passed; see evidence/2026-11-06/test-cases.md.
No remote push or real credentials involved.
