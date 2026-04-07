"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ProfileService = void 0;
/**
 * Service responsible for profile-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
class ProfileService {
    constructor(httpClient) {
        this.httpClient = httpClient;
    }
    /**
     * Get authenticated user's own profile
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When authentication fails or profile cannot be retrieved
     */
    async getOwnProfile() {
        // Step 1: Get user_id from subscription endpoint
        const subscription = await this.httpClient.get('/api/v1/subscription');
        const userId = subscription.user_id;
        // Step 2: Get full profile using the user_id
        return await this.httpClient.get(`/api/v1/user/${userId}/profile`);
    }
    /**
     * Get a profile by user ID
     * @param id - The user ID
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When profile is not found or API request fails
     */
    async getProfileById(id) {
        return await this.httpClient.get(`/api/v1/user/${id}/profile`);
    }
    /**
     * Get a profile by handle/slug
     * @param slug - The user handle/slug
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When profile is not found or API request fails
     */
    async getProfileBySlug(slug) {
        return await this.httpClient.get(`/api/v1/user/${slug}/public_profile`);
    }
}
exports.ProfileService = ProfileService;
