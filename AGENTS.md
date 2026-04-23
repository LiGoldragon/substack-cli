# Agent instructions

See also [CLAUDE.md](./CLAUDE.md) for the CLI contract and workflow rules.

## Rust style

Follow [~/git/tools-documentation/rust/style.md](../tools-documentation/rust/style.md). Summary of what must be preserved in this crate:

- **Methods on types, not free functions.** `main` is the only free function in a binary crate. `prosemirror::from_markdown` and friends are current exceptions — any new behavior goes on a type (e.g. a `Markdown` or `ProseMirrorDoc` struct), not as a free `pub fn`.
- **Domain values are newtypes.** `PostId(u64)` is the pattern. A URL returned from the image upload is a candidate for a newtype, not a bare `String`.
- **One object in, one object out.** Methods take at most one explicit object argument and return exactly one. No anonymous tuples at type boundaries.
- **One type per concept.** Avoid `-Details` / `-Info` / `-Full` companions. `PostFull` pairs `Post` with body payloads — acceptable only because `body_json`/`body_html` are lazy projections the list endpoint never returns. Do not grow more sibling types.
- **Errors: manual enum in `src/error.rs` via `thiserror`.** Never `anyhow`, `eyre`, or `Box<dyn Error>`.
- **Constructors are associated functions** (`new`, `from_*`, `with_*`). Not module-level free functions.
- **Direction-encoded names** (`as_*`, `to_*`, `from_*`, `into_*`). Prefer trait domains (`FromStr`, `Display`, `TryFrom`) over inherent `parse` / `format` methods.
- **Module layout: one concern per file.** Impls live with their type. `src/types.rs` is the one place with multiple small domain structs.

## Tool docs

Tool usage references live at [~/git/tools-documentation/](../tools-documentation/):

- [rust/style.md](../tools-documentation/rust/style.md) — Rust object style (authoritative for this crate).
- [substack/basic-usage.md](../tools-documentation/substack/basic-usage.md) — our own CLI's user-facing surface.
- [jj/basic-usage.md](../tools-documentation/jj/basic-usage.md) — VCS loop (this repo uses `jj`, not `git`, per CLAUDE.md).
- [nix/basic-usage.md](../tools-documentation/nix/basic-usage.md) — `nix flake check` is the verification authority here.
