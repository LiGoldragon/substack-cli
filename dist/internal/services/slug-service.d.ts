import type { HttpClient } from '../http-client';
import type { SlugResolver } from './slug-resolver';
/**
 * Service responsible for slug resolution and user handle management
 * Returns internal types that can be transformed into domain models
 * This is a pure service without caching - use CachingSlugService for cached behavior
 */
export declare class SlugService implements SlugResolver {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Get or build the user_id to slug mapping from subscriptions
     * @returns Promise<Map<number, string>> - Mapping of user IDs to slugs
     * @throws {Error} When subscriptions cannot be fetched (falls back to empty mapping)
     */
    getSlugMapping(): Promise<Map<number, string>>;
    /**
     * Get slug for a user ID, with fallback to handle from profile data
     * @param userId - The user ID to resolve slug for
     * @param fallbackHandle - Optional fallback handle to use if slug not found
     * @returns Promise<string | undefined> - Resolved slug or undefined if not found
     */
    getSlugForUserId(userId: number, fallbackHandle?: string): Promise<string | undefined>;
}
