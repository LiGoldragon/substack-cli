import type { SubstackComment } from '../types';
import type { HttpClient } from '../http-client';
/**
 * Service responsible for comment-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
export declare class CommentService {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Get comments for a post
     * @param postId - The post ID
     * @returns Promise<SubstackComment[]> - Raw comment data from API (validated)
     * @throws {Error} When comments cannot be retrieved or validation fails
     */
    getCommentsForPost(postId: number): Promise<SubstackComment[]>;
    /**
     * Get a specific comment by ID
     * @param id - The comment ID
     * @returns Promise<SubstackComment> - Raw comment data from API (validated)
     * @throws {Error} When comment is not found, API request fails, or validation fails
     */
    getCommentById(id: number): Promise<SubstackComment>;
}
