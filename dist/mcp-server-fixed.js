#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const index_js_1 = require("@modelcontextprotocol/sdk/server/index.js");
const stdio_js_1 = require("@modelcontextprotocol/sdk/server/stdio.js");
const types_js_1 = require("@modelcontextprotocol/sdk/types.js");
const substack_client_js_1 = require("./substack-client.js");
const TOOLS = [
    {
        name: "get_own_profile",
        description: "Get your own Substack profile information",
        inputSchema: { type: "object", properties: {}, required: [] }
    },
    {
        name: "get_profile_posts",
        description: "Get your recent Substack posts",
        inputSchema: {
            type: "object",
            properties: {
                limit: { type: "number", description: "Number of posts to retrieve (default: 10)" }
            },
            required: []
        }
    },
    {
        name: "create_note",
        description: "Create a new Substack note (short-form post)",
        inputSchema: {
            type: "object",
            properties: {
                text: { type: "string", description: "The text content of the note" }
            },
            required: ["text"]
        }
    },
    {
        name: "create_note_with_link",
        description: "Create a new Substack note with a link attachment",
        inputSchema: {
            type: "object",
            properties: {
                text: { type: "string", description: "The text content of the note" },
                link: { type: "string", description: "URL to attach to the note" }
            },
            required: ["text", "link"]
        }
    },
    {
        name: "get_post",
        description: "Get a specific Substack post by ID with full content",
        inputSchema: {
            type: "object",
            properties: {
                post_id: { type: "number", description: "The ID of the post to retrieve" }
            },
            required: ["post_id"]
        }
    },
    {
        name: "get_post_comments",
        description: "Get comments for a specific Substack post",
        inputSchema: {
            type: "object",
            properties: {
                post_id: { type: "number", description: "The ID of the post" },
                limit: { type: "number", description: "Number of comments to retrieve (default: 20)" }
            },
            required: ["post_id"]
        }
    },
    {
        name: "get_notes",
        description: "Get your recent Substack notes (short-form posts)",
        inputSchema: {
            type: "object",
            properties: {
                limit: { type: "number", description: "Number of notes to retrieve (default: 10)" }
            },
            required: []
        }
    },
    {
        name: "create_post",
        description: "Create and publish a full Substack blog post",
        inputSchema: {
            type: "object",
            properties: {
                title: { type: "string", description: "The title of the post" },
                subtitle: { type: "string", description: "The subtitle of the post (optional)" },
                body: { type: "string", description: "The body content of the post (HTML or markdown)" },
                draft: { type: "boolean", description: "Save as draft instead of publishing (default: true)" }
            },
            required: ["title", "body"]
        }
    }
];
const server = new index_js_1.Server({ name: "substack-mcp", version: "2.1.0" }, { capabilities: { tools: {} } });
server.setRequestHandler(types_js_1.ListToolsRequestSchema, async () => ({ tools: TOOLS }));
server.setRequestHandler(types_js_1.CallToolRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;
    try {
        const apiKey = process.env.SUBSTACK_API_KEY;
        if (!apiKey)
            throw new Error("SUBSTACK_API_KEY not configured");
        const client = new substack_client_js_1.SubstackClient({ apiKey, hostname: process.env.SUBSTACK_HOSTNAME || "substack.com" });
        // Existing tools
        if (name === "get_own_profile") {
            const profile = await client.ownProfile();
            return { content: [{ type: "text", text: JSON.stringify({
                            name: profile.name,
                            slug: profile.slug,
                            bio: profile.bio,
                            url: profile.url
                        }, null, 2) }] };
        }
        if (name === "get_profile_posts") {
            const { limit = 10 } = args;
            const profile = await client.ownProfile();
            const posts = [];
            let count = 0;
            for await (const post of profile.posts({ limit })) {
                posts.push({
                    id: post.id,
                    title: post.title,
                    subtitle: post.subtitle,
                    publishedAt: post.publishedAt
                });
                if (++count >= limit)
                    break;
            }
            return { content: [{ type: "text", text: JSON.stringify({ posts, count: posts.length }, null, 2) }] };
        }
        // New tools
        if (name === "create_note") {
            const { text } = args;
            const profile = await client.ownProfile();
            const result = await profile.newNote().paragraph().text(text).publish();
            return { content: [{ type: "text", text: JSON.stringify({
                            success: true,
                            note_id: result.id,
                            message: "Note created successfully"
                        }, null, 2) }] };
        }
        if (name === "create_note_with_link") {
            const { text, link } = args;
            const profile = await client.ownProfile();
            const result = await profile.newNoteWithLink(link).paragraph().text(text).publish();
            return { content: [{ type: "text", text: JSON.stringify({
                            success: true,
                            note_id: result.id,
                            link: link,
                            message: "Note with link created successfully"
                        }, null, 2) }] };
        }
        if (name === "get_post") {
            const { post_id } = args;
            const post = await client.postForId(post_id);
            return { content: [{ type: "text", text: JSON.stringify({
                            id: post.id,
                            title: post.title,
                            subtitle: post.subtitle,
                            body: post.htmlBody,
                            slug: post.slug,
                            publishedAt: post.publishedAt,
                            reactions: post.reactions,
                            restacks: post.restacks
                        }, null, 2) }] };
        }
        if (name === "get_post_comments") {
            const { post_id, limit = 20 } = args;
            const post = await client.postForId(post_id);
            const comments = [];
            let count = 0;
            for await (const comment of post.comments({ limit })) {
                comments.push({
                    id: comment.id,
                    body: comment.body,
                    author_name: comment.author.name,
                    created_at: comment.createdAt
                });
                if (++count >= limit)
                    break;
            }
            return { content: [{ type: "text", text: JSON.stringify({
                            post_id,
                            comments,
                            count: comments.length
                        }, null, 2) }] };
        }
        if (name === "get_notes") {
            const { limit = 10 } = args;
            const profile = await client.ownProfile();
            const notes = [];
            let count = 0;
            for await (const note of profile.notes({ limit })) {
                notes.push({
                    id: note.id,
                    body: note.body,
                    likesCount: note.likesCount,
                    author: note.author,
                    publishedAt: note.publishedAt
                });
                if (++count >= limit)
                    break;
            }
            return { content: [{ type: "text", text: JSON.stringify({ notes, count: notes.length }, null, 2) }] };
        }
        if (name === "create_post") {
            const { title, subtitle = "", body, draft = true } = args;
            // Convert markdown-style line breaks to HTML paragraphs if needed
            const htmlBody = body.includes('<p>') ? body : body.split('\n\n').map(p => `<p>${p.replace(/\n/g, '<br>')}</p>`).join('');
            // Create draft post
            const draftResponse = await fetch(`https://${process.env.SUBSTACK_HOSTNAME}/api/v1/drafts`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `connect.sid=${apiKey}`
                },
                body: JSON.stringify({})
            });
            if (!draftResponse.ok) {
                throw new Error(`Failed to create draft: ${draftResponse.statusText}`);
            }
            const draftData = await draftResponse.json();
            const postId = draftData.id;
            // Update draft with content
            const updateResponse = await fetch(`https://${process.env.SUBSTACK_HOSTNAME}/api/v1/drafts/${postId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `connect.sid=${apiKey}`
                },
                body: JSON.stringify({
                    title,
                    subtitle,
                    body_html: htmlBody,
                    draft: draft
                })
            });
            if (!updateResponse.ok) {
                throw new Error(`Failed to update draft: ${updateResponse.statusText}`);
            }
            return { content: [{ type: "text", text: JSON.stringify({
                            success: true,
                            post_id: postId,
                            title,
                            draft: draft,
                            message: draft ? "Draft post created successfully" : "Post published successfully",
                            url: `https://${process.env.SUBSTACK_HOSTNAME}/p/${postId}`
                        }, null, 2) }] };
        }
        throw new Error("Unknown tool: " + name);
    }
    catch (error) {
        return { content: [{ type: "text", text: JSON.stringify({ error: error.message }, null, 2) }], isError: true };
    }
});
async function main() {
    const transport = new stdio_js_1.StdioServerTransport();
    await server.connect(transport);
    console.error("Substack MCP Server v2.1.0 running with post creation support");
}
main().catch((error) => { console.error("Fatal error:", error); process.exit(1); });
