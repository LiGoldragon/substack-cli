# substack-cli

CLI for creating, publishing, and managing Substack posts and publication
settings.

## Workflow

- Use `jj` for VCS operations.
- Run `pwd` before `jj`, `cargo`, or `nix` commands.
- `nix flake check` is the verification authority. `cargo test` is only a
  local smoke check.
- Keep command inputs and outputs explicit. Do not describe HTML as Markdown,
  and do not accept CLI options that have no behavioral effect.

## CLI Contract

- `substack publication get`
- `substack publication update`
- `substack publication set-logo --file <path>`
- `substack publication set-wide-logo --file <path>`
- `substack publication set-cover-photo --file <path>`
- `substack publication set-email-banner --file <path>`
- `substack image upload --file <path>`
- `substack post create (--body <markdown> | --file-path <path>)`
- `substack post update <post-id> (--body <markdown> | --file-path <path>)`
- `substack post list`
- `substack post get <post-id> [--full] [--save-html <path>] [--save-json <path>]`
- `substack post delete <post-id>`

The default flake app wraps `substack` and injects `SUBSTACK_API_KEY` plus
`SUBSTACK_HOSTNAME` from `gopass show -o substack.com/api-key`. The unwrapped
binary still accepts those environment variables directly.
