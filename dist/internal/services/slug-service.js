"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SlugService = void 0;
/**
 * Service responsible for slug resolution and user handle management
 * Returns internal types that can be transformed into domain models
 * This is a pure service without caching - use CachingSlugService for cached behavior
 */
class SlugService {
    constructor(httpClient) {
        this.httpClient = httpClient;
    }
    /**
     * Get or build the user_id to slug mapping from subscriptions
     * @returns Promise<Map<number, string>> - Mapping of user IDs to slugs
     * @throws {Error} When subscriptions cannot be fetched (falls back to empty mapping)
     */
    async getSlugMapping() {
        var _a;
        try {
            // Fetch subscriptions data
            const subscriptionsResponse = await this.httpClient.get('/api/v1/subscriptions');
            // Build user_id -> slug mapping
            const mapping = new Map();
            for (const publication of subscriptionsResponse.publications) {
                if (publication.author_id) {
                    // Use author_handle as the slug, but only if it's not empty
                    const slug = ((_a = publication.author_handle) === null || _a === void 0 ? void 0 : _a.trim()) || undefined;
                    if (slug) {
                        mapping.set(publication.author_id, slug);
                    }
                }
            }
            return mapping;
        }
        catch (_b) {
            // If subscriptions endpoint fails, return empty mapping
            // This ensures graceful fallback
            return new Map();
        }
    }
    /**
     * Get slug for a user ID, with fallback to handle from profile data
     * @param userId - The user ID to resolve slug for
     * @param fallbackHandle - Optional fallback handle to use if slug not found
     * @returns Promise<string | undefined> - Resolved slug or undefined if not found
     */
    async getSlugForUserId(userId, fallbackHandle) {
        const slugMapping = await this.getSlugMapping();
        return slugMapping.get(userId) || fallbackHandle;
    }
}
exports.SlugService = SlugService;
