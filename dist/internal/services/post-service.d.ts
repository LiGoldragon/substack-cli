import type { SubstackPost, SubstackFullPost } from '../types';
import type { HttpClient } from '../http-client';
/**
 * Service responsible for post-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
export declare class PostService {
    private readonly globalHttpClient;
    private readonly httpClient;
    constructor(globalHttpClient: HttpClient, httpClient: HttpClient);
    /**
     * Get a post by ID from the API
     * @param id - The post ID
     * @returns Promise<SubstackFullPost> - Raw full post data from API (validated)
     * @throws {Error} When post is not found, API request fails, or validation fails
     *
     * Note: Uses SubstackFullPostCodec to validate the full post response from /posts/by-id/:id
     * which includes body_html, postTags, reactions, and other fields not present in preview responses.
     * This codec is specifically designed for FullPost construction.
     */
    getPostById(id: number): Promise<SubstackFullPost>;
    /**
     * Transform raw API post data to match our codec structure
     */
    private transformPostData;
    /**
     * Get posts for a profile
     * @param profileId - The profile user ID
     * @param options - Pagination options
     * @returns Promise<SubstackPost[]> - Raw post data from API (validated)
     * @throws {Error} When posts cannot be retrieved or validation fails
     */
    getPostsForProfile(profileId: number, options: {
        limit: number;
        offset: number;
    }): Promise<SubstackPost[]>;
}
