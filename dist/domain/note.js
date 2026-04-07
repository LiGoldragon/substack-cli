"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Note = void 0;
const comment_1 = require("./comment");
/**
 * Note entity representing a Substack note
 */
class Note {
    constructor(rawData, client) {
        var _a, _b;
        this.rawData = rawData;
        this.client = client;
        this.id = rawData.entity_key;
        this.body = ((_a = rawData.comment) === null || _a === void 0 ? void 0 : _a.body) || '';
        this.likesCount = ((_b = rawData.comment) === null || _b === void 0 ? void 0 : _b.reaction_count) || 0;
        this.publishedAt = new Date(rawData.context.timestamp);
        // Extract author info from context users
        const firstUser = rawData.context.users[0];
        this.author = {
            id: (firstUser === null || firstUser === void 0 ? void 0 : firstUser.id) || 0,
            name: (firstUser === null || firstUser === void 0 ? void 0 : firstUser.name) || 'Unknown',
            handle: (firstUser === null || firstUser === void 0 ? void 0 : firstUser.handle) || 'unknown',
            avatarUrl: (firstUser === null || firstUser === void 0 ? void 0 : firstUser.photo_url) || ''
        };
    }
    /**
     * Get parent comments for this note
     */
    async *comments() {
        // Convert parent comments to Comment entities
        for (const parentComment of this.rawData.parentComments || []) {
            if (parentComment) {
                // Convert note comment format to SubstackComment format
                const comment = parentComment;
                const commentData = {
                    id: comment.id,
                    body: comment.body,
                    created_at: comment.date,
                    parent_post_id: comment.post_id || 0,
                    author_id: comment.user_id,
                    author_name: comment.name,
                    author_is_admin: false // Not available in note comment format
                };
                yield new comment_1.Comment(commentData, this.client);
            }
        }
    }
    /**
     * Like this note
     */
    async like() {
        // Implementation will like the note via the client
        // This requires authentication and proper API endpoints
        throw new Error('Note liking not implemented yet - requires like API');
    }
    /**
     * Add a comment to this note
     */
    async addComment(_text) {
        // Implementation will add a comment via the client
        // This requires authentication and proper API endpoints
        throw new Error('Note commenting not implemented yet - requires comment API');
    }
}
exports.Note = Note;
