#!/usr/bin/env node
"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const index_js_1 = require("@modelcontextprotocol/sdk/server/index.js");
const stdio_js_1 = require("@modelcontextprotocol/sdk/server/stdio.js");
const types_js_1 = require("@modelcontextprotocol/sdk/types.js");
const substack_client_js_1 = require("./substack-client.js");
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
// Helper function to upload image to Substack and get URL
async function uploadImageToSubstack(filePath, postId, hostname, apiKey) {
    const fileBuffer = fs.readFileSync(filePath);
    const base64Image = fileBuffer.toString('base64');
    // Determine MIME type from file extension
    const ext = path.extname(filePath).toLowerCase();
    const mimeTypes = {
        '.png': 'image/png',
        '.jpg': 'image/jpeg',
        '.jpeg': 'image/jpeg',
        '.gif': 'image/gif',
        '.webp': 'image/webp'
    };
    const mimeType = mimeTypes[ext] || 'image/png';
    // Create data URI
    const dataUri = `data:${mimeType};base64,${base64Image}`;
    const response = await fetch(`https://${hostname}/api/v1/image`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `connect.sid=${apiKey}`
        },
        body: JSON.stringify({
            image: dataUri,
            postId: postId
        })
    });
    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Substack image upload failed: ${response.statusText} - ${errorText}`);
    }
    const data = await response.json();
    return data.url;
}
// Helper function to convert text to ProseMirror JSON format
function textToProseMirror(text) {
    const paragraphs = text.split('\n\n').filter(p => p.trim());
    return {
        type: 'doc',
        content: paragraphs.map(para => ({
            type: 'paragraph',
            content: [{
                    type: 'text',
                    text: para.replace(/\n/g, ' ')
                }]
        }))
    };
}
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
        description: "Create and publish a full Substack blog post with optional cover image",
        inputSchema: {
            type: "object",
            properties: {
                title: { type: "string", description: "The title of the post" },
                subtitle: { type: "string", description: "The subtitle of the post (optional)" },
                body: { type: "string", description: "The body content of the post (HTML or markdown)" },
                cover_image: { type: "string", description: "Path to cover image file (optional, will be uploaded to Substack)" },
                draft: { type: "boolean", description: "Save as draft instead of publishing (default: true)" }
            },
            required: ["title", "body"]
        }
    }
];
const server = new index_js_1.Server({ name: "substack-mcp", version: "2.3.0" }, { capabilities: { tools: {} } });
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
            const { title, subtitle = "", body, cover_image, draft = true } = args;
            // Step 1: Get user_id from publication_user endpoint
            const userResponse = await fetch(`https://${process.env.SUBSTACK_HOSTNAME}/api/v1/publication_user`, {
                method: 'GET',
                headers: {
                    'Cookie': `connect.sid=${apiKey}`
                }
            });
            if (!userResponse.ok) {
                throw new Error(`Failed to get user info: ${userResponse.statusText}`);
            }
            const userData = await userResponse.json();
            const userId = userData.pub_users[0].user_id;
            // Step 2: Create draft post with draft_bylines
            const draftResponse = await fetch(`https://${process.env.SUBSTACK_HOSTNAME}/api/v1/drafts`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `connect.sid=${apiKey}`
                },
                body: JSON.stringify({
                    draft_bylines: [{
                            id: userId,
                            user_id: userId
                        }]
                })
            });
            if (!draftResponse.ok) {
                throw new Error(`Failed to create draft: ${draftResponse.statusText}`);
            }
            const draftData = await draftResponse.json();
            const postId = draftData.id;
            // Step 3: Upload cover image if provided
            let coverImageUrl;
            if (cover_image) {
                try {
                    coverImageUrl = await uploadImageToSubstack(cover_image, postId, process.env.SUBSTACK_HOSTNAME || "substack.com", apiKey);
                }
                catch (error) {
                    throw new Error(`Failed to upload cover image: ${error.message}`);
                }
            }
            // Step 4: Convert body to ProseMirror JSON format
            const proseMirrorBody = textToProseMirror(body);
            // Step 5: Update draft with content using correct field names
            const updatePayload = {
                draft_title: title,
                draft_subtitle: subtitle,
                draft_body: JSON.stringify(proseMirrorBody)
            };
            if (coverImageUrl) {
                updatePayload.cover_image = coverImageUrl;
            }
            const updateResponse = await fetch(`https://${process.env.SUBSTACK_HOSTNAME}/api/v1/drafts/${postId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `connect.sid=${apiKey}`
                },
                body: JSON.stringify(updatePayload)
            });
            if (!updateResponse.ok) {
                throw new Error(`Failed to update draft: ${updateResponse.statusText}`);
            }
            return { content: [{ type: "text", text: JSON.stringify({
                            success: true,
                            post_id: postId,
                            title,
                            cover_image_url: coverImageUrl,
                            draft: draft,
                            message: draft ? "Draft post created successfully" : "Post published successfully",
                            url: `https://${process.env.SUBSTACK_HOSTNAME}/publish/post/${postId}`
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
    console.error("Substack MCP Server v2.2.0 running with cover image support");
}
main().catch((error) => { console.error("Fatal error:", error); process.exit(1); });
