"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CommentService = void 0;
const types_1 = require("../types");
const validation_1 = require("../validation");
/**
 * Service responsible for comment-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
class CommentService {
    constructor(httpClient) {
        this.httpClient = httpClient;
    }
    /**
     * Get comments for a post
     * @param postId - The post ID
     * @returns Promise<SubstackComment[]> - Raw comment data from API (validated)
     * @throws {Error} When comments cannot be retrieved or validation fails
     */
    async getCommentsForPost(postId) {
        const response = await this.httpClient.get(`/api/v1/post/${postId}/comments`);
        const comments = response.comments || [];
        // Validate each comment with io-ts
        return comments.map((comment, index) => (0, validation_1.decodeOrThrow)(types_1.SubstackCommentCodec, comment, `Comment ${index} in post response`));
    }
    /**
     * Get a specific comment by ID
     * @param id - The comment ID
     * @returns Promise<SubstackComment> - Raw comment data from API (validated)
     * @throws {Error} When comment is not found, API request fails, or validation fails
     */
    async getCommentById(id) {
        const rawResponse = await this.httpClient.get(`/api/v1/reader/comment/${id}`);
        // Validate the response structure with io-ts
        const response = (0, validation_1.decodeOrThrow)(types_1.SubstackCommentResponseCodec, rawResponse, 'Comment response');
        // Transform the validated API response to match SubstackComment interface
        const commentData = {
            id: response.item.comment.id,
            body: response.item.comment.body,
            created_at: response.item.comment.date,
            parent_post_id: response.item.comment.post_id || 0,
            author_id: response.item.comment.user_id,
            author_name: response.item.comment.name,
            author_is_admin: false // Default value since not in response
        };
        // Validate the transformed data as well
        return (0, validation_1.decodeOrThrow)(types_1.SubstackCommentCodec, commentData, 'Transformed comment data');
    }
}
exports.CommentService = CommentService;
