# substack-cli

Command-line tool for publishing and managing Substack posts.

## Features

- Create and publish posts from Markdown files or inline body text.
- Update an existing post and republish it.
- List, inspect, and delete posts.
- Upload images and set publication image assets.
- Read the post title from the first Markdown `#` heading and omit that heading from the published body.
- Read `title` and `subtitle` from YAML-style frontmatter when present.

## Requirements

- A Substack session cookie value for `substack.sid`.
- A publication hostname, such as `example.substack.com`.

The CLI reads these environment variables:

```sh
export SUBSTACK_API_KEY="your-substack.sid-value"
export SUBSTACK_HOSTNAME="example.substack.com"
```

## Nix

This repository includes a flake. The default package wraps the CLI and reads secrets from `gopass`:

- `substack.com/api-key` for `SUBSTACK_API_KEY`
- `substack.com/api-key publication-url` for `SUBSTACK_HOSTNAME`

Run the wrapped CLI:

```sh
nix run . -- post list
```

Enter a development shell:

```sh
nix develop
```

## Build

```sh
cargo build --locked
```

Run directly with Cargo:

```sh
cargo run -- post list
```

## Posts

Create and publish a Markdown post:

```sh
substack post create --file-path ./article.md
```

Create a draft instead of publishing:

```sh
substack post create --file-path ./article.md --draft
```

Update and republish an existing post:

```sh
substack post update 123456789 --file-path ./article.md
```

Publish a post while rewriting repo-local Markdown article links through the
manifest, auto-publishing missing linked files first:

```sh
substack post create \
  --file-path ./article.md \
  --publish-linked-files
```

List recent posts:

```sh
substack post list --limit 20
```

Get post metadata:

```sh
substack post get 123456789
```

Save the full post response:

```sh
substack post get 123456789 --full --save-html post.html --save-json post.json
```

Delete a post or draft:

```sh
substack post delete 123456789
```

## Markdown Behavior

When a post is created or updated from Markdown:

- Frontmatter delimited by `---` is removed before publishing.
- `title:` frontmatter is used as the post title when supplied.
- `subtitle:` frontmatter is used as the post subtitle when supplied.
- If no explicit title is supplied, the first `# Heading` becomes the post title.
- If the first line is the same `# Heading` used as the title, that heading is removed from the body to avoid repeating the title.

Supported body conversion is intentionally small:

- `##` and `###` headings
- Paragraphs
- Blockquotes
- Hard line breaks using a trailing backslash
- Inline links: `[label](url)`
- `*italic*`, `**bold**`, and `***bold italic***`

### Local article links

When a Markdown file contains a local link to another Markdown file, such as:

```md
[Plasma Recycling Manual](./Plasma_Recycling_Manual.md)
```

the CLI can rewrite that link to the canonical Substack post URL instead of
publishing the raw `.md` path.

Behavior:

- The CLI looks up published local files in `.substack-posts.json` at the
  workspace root by default (or the nearest ancestor containing `.jj` or
  `.git`).
- Override the manifest location with `--link-manifest /path/to/manifest.json`.
- If a local `.md` link is found and the target already exists in the manifest,
  the link is rewritten to `https://<hostname>/p/<slug>`.
- If a local `.md` link is found and the target is missing from the manifest,
  the command fails by default with a clear error.
- Pass `--publish-linked-files` to publish missing local `.md` dependencies
  first, record them in the manifest, and then rewrite the links to their
  canonical Substack URLs.

Manifest shape:

```json
{
  "posts": [
    {
      "source_path": "water/Keep_the_Plasma.md",
      "post_id": 195628661,
      "slug": "keep-the-plasma"
    }
  ]
}
```

## Images

Upload an image:

```sh
substack image upload --file ./image.png
```

Use an image as a post cover while creating or updating:

```sh
substack post create --file-path ./article.md --cover-image ./cover.jpg
substack post update 123456789 --file-path ./article.md --cover-image ./cover.jpg
```

Set publication images:

```sh
substack publication set-logo --file ./logo.png
substack publication set-wide-logo --file ./wide-logo.png
substack publication set-cover-photo --file ./cover.png
substack publication set-email-banner --file ./email-banner.png
```

## Publication

Read publication settings:

```sh
substack publication get
```

Update publication settings:

```sh
substack publication update --name "Publication Name" --hero-text "Short description"
```

Other supported update flags include:

- `--language`
- `--copyright`
- `--logo-url`
- `--logo-url-wide`
- `--cover-photo-url`
- `--email-banner-url`
- `--theme-var-background-pop`
- `--community-enabled`
- `--community-disabled`

## Notes

This tool uses Substack's private web API and authenticates with the same session cookie used by the browser. API behavior can change without notice.
