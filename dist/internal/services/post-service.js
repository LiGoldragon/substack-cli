"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PostService = void 0;
const types_1 = require("../types");
const validation_1 = require("../validation");
/**
 * Service responsible for post-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
class PostService {
    constructor(globalHttpClient, httpClient) {
        this.globalHttpClient = globalHttpClient;
        this.httpClient = httpClient;
    }
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
    async getPostById(id) {
        // Post lookup by ID must use the global substack.com endpoint, not publication-specific hostnames
        const rawResponse = await this.globalHttpClient.get(`/api/v1/posts/by-id/${id}`);
        // Extract the post data from the wrapper object
        if (!rawResponse.post) {
            throw new Error('Invalid response format: missing post data');
        }
        // Transform the raw post data to match our codec expectations
        const postData = this.transformPostData(rawResponse.post);
        // Validate the response with SubstackFullPostCodec for full post data including body_html
        return (0, validation_1.decodeOrThrow)(types_1.SubstackFullPostCodec, postData, 'Full post response');
    }
    /**
     * Transform raw API post data to match our codec structure
     */
    transformPostData(rawPost) {
        const transformedPost = { ...rawPost };
        // Transform postTags from objects to string array
        if (rawPost.postTags && Array.isArray(rawPost.postTags)) {
            transformedPost.postTags = rawPost.postTags.map((tag) => tag.name || tag);
        }
        return transformedPost;
    }
    /**
     * Get posts for a profile
     * @param profileId - The profile user ID
     * @param options - Pagination options
     * @returns Promise<SubstackPost[]> - Raw post data from API (validated)
     * @throws {Error} When posts cannot be retrieved or validation fails
     */
    async getPostsForProfile(profileId, options) {
        const response = await this.httpClient.get(`/api/v1/profile/posts?profile_user_id=${profileId}&limit=${options.limit}&offset=${options.offset}`);
        const posts = response.posts || [];
        // Validate each post with io-ts
        return posts.map((post, index) => (0, validation_1.decodeOrThrow)(types_1.SubstackPostCodec, post, `Post ${index} in profile response`));
    }
}
exports.PostService = PostService;
