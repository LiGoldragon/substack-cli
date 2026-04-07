"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CachingSlugService = void 0;
/**
 * Caching decorator for SlugResolver
 * Implements the Decorator pattern to add caching behavior to any SlugResolver
 */
class CachingSlugService {
    constructor(cache, base) {
        this.cache = cache;
        this.base = base;
    }
    /**
     * Get slug for a user ID with caching, with fallback to handle from profile data
     * @param userId - The user ID to resolve slug for
     * @param fallbackHandle - Optional fallback handle to use if slug not found
     * @returns Promise<string | undefined> - Resolved slug or undefined if not found
     */
    async getSlugForUserId(userId, fallbackHandle) {
        // Check individual cache first
        if (this.cache.has(userId)) {
            return this.cache.get(userId);
        }
        // If not in individual cache, delegate to base service
        const slug = await this.base.getSlugForUserId(userId, fallbackHandle);
        // Cache the result if we found a slug (not fallback or undefined)
        if (slug && slug !== fallbackHandle) {
            this.cache.set(userId, slug);
        }
        return slug;
    }
}
exports.CachingSlugService = CachingSlugService;
