"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Comment = void 0;
/**
 * Comment entity representing a comment on a post or note
 */
class Comment {
    constructor(rawData, client) {
        this.rawData = rawData;
        this.client = client;
        this.id = rawData.id;
        this.body = rawData.body;
        this.author = {
            id: rawData.author_id,
            name: rawData.author_name,
            isAdmin: rawData.author_is_admin
        };
        this.createdAt = new Date(rawData.created_at);
        this.likesCount = undefined; // TODO: Extract from rawData when available
    }
}
exports.Comment = Comment;
