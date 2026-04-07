"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FullPost = exports.PreviewPost = void 0;
const comment_1 = require("./comment");
/**
 * PreviewPost entity representing a Substack post with truncated content
 */
class PreviewPost {
    constructor(rawData, client, commentService, postService) {
        this.client = client;
        this.commentService = commentService;
        this.postService = postService;
        this.id = rawData.id;
        this.title = rawData.title;
        this.subtitle = rawData.subtitle || '';
        this.truncatedBody = rawData.truncated_body_text || '';
        this.body = rawData.truncated_body_text || '';
        this.likesCount = 0; // TODO: Extract from rawData when available
        this.publishedAt = new Date(rawData.post_date);
        // TODO: Extract author information from rawData
        // For now, use placeholder values
        this.author = {
            id: 0,
            name: 'Unknown Author',
            handle: 'unknown',
            avatarUrl: ''
        };
    }
    /**
     * Fetch the full post data with HTML body content
     * @returns Promise<FullPost> - A FullPost instance with complete content
     * @throws {Error} When full post retrieval fails
     */
    async fullPost() {
        try {
            const fullPostData = await this.postService.getPostById(this.id);
            return new FullPost(fullPostData, this.client, this.commentService, this.postService);
        }
        catch (error) {
            throw new Error(`Failed to fetch full post ${this.id}: ${error.message}`);
        }
    }
    /**
     * Get comments for this post
     * @throws {Error} When comment retrieval fails or API is unavailable
     */
    async *comments(options = {}) {
        try {
            const commentsData = await this.commentService.getCommentsForPost(this.id);
            let count = 0;
            for (const commentData of commentsData) {
                if (options.limit && count >= options.limit)
                    break;
                yield new comment_1.Comment(commentData, this.client);
                count++;
            }
        }
        catch (error) {
            throw new Error(`Failed to get comments for post ${this.id}: ${error.message}`);
        }
    }
    /**
     * Like this post
     */
    async like() {
        // Implementation will like the post via the client
        throw new Error('Post liking not implemented yet - requires like API');
    }
    /**
     * Add a comment to this post
     */
    async addComment(_data) {
        // Implementation will add comment via the client
        throw new Error('Comment creation not implemented yet - requires comment creation API');
    }
}
exports.PreviewPost = PreviewPost;
/**
 * FullPost entity representing a Substack post with complete HTML content
 */
class FullPost extends PreviewPost {
    constructor(rawData, client, commentService, postService) {
        super(rawData, client, commentService, postService);
        // Prefer body_html from the full post response, fall back to htmlBody for backward compatibility
        this.htmlBody = rawData.body_html || rawData.htmlBody || '';
        this.slug = rawData.slug;
        this.createdAt = new Date(rawData.post_date);
        this.reactions = rawData.reactions;
        this.restacks = rawData.restacks;
        this.postTags = rawData.postTags;
        this.coverImage = rawData.cover_image;
    }
}
exports.FullPost = FullPost;
