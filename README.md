# Substack MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/)

An MCP (Model Context Protocol) server that allows Claude Desktop and Claude Code to interact with your Substack publication. Create posts with cover images, publish notes, manage your content, and more - all through natural conversation with Claude.

## Features

- ✅ **Post Management**: Create full blog posts with cover images
- ✅ **Notes**: Create short-form Substack notes
- ✅ **Profile Access**: Get your own profile and other users' profiles
- ✅ **Content Retrieval**: Fetch posts, notes, and comments
- ✅ **Image Upload**: Native Substack image upload (v2.3.0) - no third-party dependencies
- ✅ **Draft Mode**: Create drafts for review before publishing

## Version 2.3.0 - What's New

This version replaces Imgur with Substack's native image upload:
- **Native Integration**: Direct upload to Substack's CDN (Amazon S3)
- **Data URI Format**: Images encoded as base64 data URIs
- **Automatic MIME Detection**: Supports PNG, JPG, JPEG, GIF, WEBP
- **Post Association**: Images properly linked to posts via `postId`
- **More Reliable**: No third-party API dependencies

## Prerequisites

- **Node.js 18+** installed on your system
- **Substack account** with publication access
- **Substack API Key** (connect.sid cookie value)

## Getting Your Substack API Key

The Substack API uses cookie-based authentication:

1. Login to Substack in your browser
2. Open Developer Tools (F12 or Right-click → Inspect)
3. Go to **Application/Storage** tab → **Cookies** → `https://substack.com`
4. Find the `connect.sid` cookie and copy its value
5. This value is your `SUBSTACK_API_KEY`

⚠️ **Important**: Keep this cookie value private. Do not commit it to version control.

### Cookie Extraction Tools

The `tools/` directory contains utility scripts to help extract your Substack cookie:
- `extract-cookie.js` - Manual cookie extraction tool
- `extract-cookie-auto.js` - Automated cookie extraction

To use these tools, save your cookie value to `tools/cookie.txt` (this file is git-ignored for security).

## Installation

### Building the MCP Server

```bash
# Clone or download this repository
cd substack-mcp

# Install dependencies
npm install

# Build the TypeScript code
npm run build
```

### For Claude Desktop

1. Open your Claude Desktop configuration file:
   - **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
   - **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - **Linux**: `~/.config/Claude/claude_desktop_config.json`

2. Add the Substack MCP server configuration:

```json
{
  "mcpServers": {
    "substack": {
      "command": "node",
      "args": ["C:\\mcp-servers\\substack-mcp\\dist\\mcp-server.js"],
      "env": {
        "SUBSTACK_API_KEY": "your-connect-sid-cookie-value-here",
        "SUBSTACK_HOSTNAME": "yoursite.substack.com"
      }
    }
  }
}
```

3. Replace `your-connect-sid-cookie-value-here` with your actual cookie value
4. Replace `yoursite.substack.com` with your Substack hostname
5. Restart Claude Desktop

### For Claude Code

Add to your Claude Code MCP configuration or use:

```bash
claude mcp add substack-mcp
```

## Available Tools

### `mcp__substack__create_post`
Create a full blog post with optional cover image

**Parameters:**
- `title` (required): Post title
- `body` (required): Post content (supports markdown)
- `subtitle` (optional): Post subtitle
- `cover_image` (optional): Path to cover image file
- `draft` (optional): Create as draft (default: true)

**Example:**
```typescript
{
  title: "My Amazing Post",
  body: "This is the content of my post...",
  cover_image: "c:/temp/cover-image.png",
  draft: true
}
```

### `mcp__substack__create_note`
Create a new Substack note (short-form post)

**Parameters:**
- `text` (required): Note content

### `mcp__substack__create_note_with_link`
Create a note with a link attachment

**Parameters:**
- `text` (required): Note content
- `link` (required): URL to attach

### `mcp__substack__get_own_profile`
Get your own Substack profile information

**Returns**: name, slug, handle, bio, follower count, photo URL

### `mcp__substack__get_profile_posts`
Get your recent posts

**Parameters:**
- `limit` (optional): Number of posts to retrieve (default: 10)

### `mcp__substack__get_post`
Get a specific post by ID with full content

**Parameters:**
- `post_id` (required): The post ID

### `mcp__substack__get_post_comments`
Get comments for a specific post

**Parameters:**
- `post_id` (required): The post ID
- `limit` (optional): Number of comments (default: 20)

### `mcp__substack__get_notes`
Get your recent notes

**Parameters:**
- `limit` (optional): Number of notes (default: 10)

## Usage Examples

### With Claude Desktop/Code

Once configured, you can have natural conversations with Claude:

```
"Create a new blog post titled 'Why I Love Programming' with this content..."

"Create a draft post with the article from article.md and use cover.png as the cover image"

"Get my recent posts from the last week"

"Create a note saying 'New post just published!'"
```

Claude will automatically use the appropriate MCP tools to fulfill your requests.

## Environment Variables

- `SUBSTACK_API_KEY` (required): Your connect.sid cookie value
- `SUBSTACK_HOSTNAME` (required): Your Substack hostname (e.g., "yourname.substack.com")

## Workflow Example

Here's a typical workflow for creating a post with cover image:

1. Write your article in markdown format
2. Create or generate a cover image
3. Tell Claude: "Create a draft post with article.md and cover.png"
4. Claude will:
   - Read the markdown file
   - Upload the cover image to Substack's CDN
   - Create the draft post with both
   - Return the draft URL for review

## Documentation

- [CLAUDE.md](CLAUDE.md) - Development guidelines for Claude Code

## Troubleshooting

### "SUBSTACK_API_KEY environment variable is required"
Make sure you've added the `SUBSTACK_API_KEY` to the `env` section of your MCP configuration.

### "Failed to connect"
1. Verify your connect.sid cookie value is correct and hasn't expired
2. Check that you're logged into Substack in your browser
3. Try getting a fresh cookie value by logging out and back in

### MCP server not appearing in Claude Desktop
1. Check that the path to `mcp-server.js` is correct
2. Ensure Node.js 18+ is installed: `node --version`
3. Run `npm run build` to compile the TypeScript
4. Restart Claude Desktop after configuration changes

### Image upload fails
1. Ensure the image file exists and is readable
2. Check that the file format is supported (PNG, JPG, JPEG, GIF, WEBP)
3. Verify your Substack API key is valid

## Development

To extend the MCP server:

1. Edit `src/mcp-server.ts` to add new tools
2. Run `npm run build` to compile TypeScript
3. Restart Claude Desktop/Code to load changes

### Testing

```bash
npm test              # Run all tests
npm run test:unit     # Unit tests only
npm run lint          # Check code style
npm run format        # Format code
```

## Security Notes

- Never commit your connect.sid cookie value to version control
- Store your API key securely using environment variables
- The cookie value gives full access to your Substack account
- Regularly refresh your cookie value for security

## Version History

- **v2.3.0** (Nov 2025): Native Substack image upload, replaces Imgur
- **v2.2.0**: Added Imgur-based cover image upload
- **v2.1.0**: Added create_post tool with ProseMirror support
- **v2.0.0**: Initial MCP server implementation

## License

MIT - See LICENSE file for details

## Credits

Built on top of the [substack-api](https://github.com/jakub-k-slys/substack-api) TypeScript client.

MCP server and image upload implementation by Daniel Simon Jr.
