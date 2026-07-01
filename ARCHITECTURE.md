# substack-cli — architecture

## Overview

`substack-cli` is a command-line tool for creating, publishing, and managing
Substack posts and publication settings. It drives Substack's **private web API**
and authenticates with the same session cookie the browser uses. Because the API
is private, its behavior can change without notice; the CLI keeps command inputs
and outputs explicit rather than assuming stable server behavior.

The crate builds one binary, `substack`, plus a `substack_cli` library.

## Components and boundaries

- **CLI surface** (`src/cli.rs`, `src/main.rs`) — the clap command tree. `main`
  is the only free function in the binary. The command contract is fixed (see
  below); options that have no behavioral effect are not accepted.
- **Application layer** (`src/application.rs`) — orchestrates a parsed command
  into client calls, Markdown conversion, manifest resolution, and image work.
- **API client** (`src/client.rs`) — the reqwest-based client against Substack's
  private web API, carrying the session cookie.
- **Domain types** (`src/types.rs`) — the domain structs. Domain values are
  newtypes (`PostId(u64)` is the pattern). `PostFull` pairs `Post` with lazy body
  payloads (`body_json` / `body_html`) the list endpoint never returns; it is the
  one accepted sibling type, not a template for more.
- **Errors** (`src/error.rs`) — a manual `thiserror` enum at the crate boundary.
  Never `anyhow`, `eyre`, or `Box<dyn Error>`.
- **Markdown to ProseMirror** (`src/prosemirror.rs`) — converts Markdown into the
  ProseMirror body Substack stores. `prosemirror::from_markdown` and friends are
  a deliberate current exception to the methods-on-types rule.
- **Local post manifest** (`src/local_post_manifest.rs`) — reads and updates the
  `.substack-posts.json` manifest and rewrites local Markdown article links to
  canonical Substack URLs.
- **Image handling** (`src/image_file.rs`, `src/table_image.rs`) — image upload
  and rendered table images; `table_image` uses the bundled font asset under
  `assets/`.

## CLI contract

```text
substack publication get
substack publication update
substack publication set-logo --file <path>
substack publication set-wide-logo --file <path>
substack publication set-cover-photo --file <path>
substack publication set-email-banner --file <path>
substack image upload --file <path>
substack post create (--body <markdown> | --file-path <path>)
substack post update <post-id> (--body <markdown> | --file-path <path>)
substack post list
substack post get <post-id> [--full] [--save-html <path>] [--save-json <path>]
substack post delete <post-id>
```

## Markdown behavior

Conversion from Markdown is intentionally small and explicit:

- Frontmatter delimited by `---` is removed before publishing; `title:` and
  `subtitle:` frontmatter set the post title and subtitle when supplied.
- With no explicit title, the first `# Heading` becomes the post title, and that
  heading is dropped from the body to avoid repeating the title.
- Supported body conversion is limited to `##`/`###` headings, paragraphs,
  blockquotes, hard line breaks (trailing backslash), inline links, and
  `*italic*` / `**bold**` / `***bold italic***`.

### Local article links

When a Markdown file links to another local `.md` file, the CLI can rewrite that
link to the canonical Substack post URL instead of publishing the raw path.

- Published local files are looked up in `.substack-posts.json` at the workspace
  root by default (or the nearest ancestor containing `.jj` or `.git`);
  `--link-manifest` overrides the location.
- A found local link whose target already exists in the manifest is rewritten to
  `https://<hostname>/p/<slug>`.
- A found local link missing from the manifest fails by default; pass
  `--publish-linked-files` to publish the missing dependencies first, record them,
  and then rewrite.
- When `--cover-image` is not supplied and the source file has a `banner_image`
  entry in the manifest, that banner is uploaded and used as the cover image.

## Authentication and secrets

The CLI reads two environment variables:

- `SUBSTACK_API_KEY` — the `substack.sid` session-cookie value.
- `SUBSTACK_HOSTNAME` — the publication hostname, such as `example.substack.com`.

The default flake app wraps `substack` and injects both from
`gopass show -o substack.com/api-key` (with `publication-url` for the hostname);
the unwrapped binary accepts the environment variables directly. Secret values
stay transient and out of durable surfaces.

## Constraints and invariants

- Keep command inputs and outputs explicit. Do not describe HTML as Markdown, and
  do not accept CLI options that have no behavioral effect.
- Rust object style follows lore's `rust/style.md`: methods on data-bearing types
  rather than free functions, domain newtypes, one object in and one object out at
  type boundaries, one type per concept, constructors as associated functions, and
  direction-encoded names (`as_*`, `to_*`, `from_*`, `into_*`) preferring trait
  domains. One concern per file, with impls beside their type; `src/types.rs` is
  the one place holding multiple small domain structs.
- `nix flake check` is the verification authority. `cargo test` is a local smoke
  check only.

## Nix

The flake uses crane with the fenix latest toolchain. `packages.default` is the
gopass-wrapping `substack`; `packages.unwrapped` is the bare binary. The source
filter keeps `assets/*.ttf` alongside the Cargo sources so the table-image font is
present in the build.
