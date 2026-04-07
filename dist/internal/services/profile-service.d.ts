import type { SubstackFullProfile } from '../types';
import type { HttpClient } from '../http-client';
/**
 * Service responsible for profile-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
export declare class ProfileService {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Get authenticated user's own profile
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When authentication fails or profile cannot be retrieved
     */
    getOwnProfile(): Promise<SubstackFullProfile>;
    /**
     * Get a profile by user ID
     * @param id - The user ID
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When profile is not found or API request fails
     */
    getProfileById(id: number): Promise<SubstackFullProfile>;
    /**
     * Get a profile by handle/slug
     * @param slug - The user handle/slug
     * @returns Promise<SubstackFullProfile> - Raw profile data from API
     * @throws {Error} When profile is not found or API request fails
     */
    getProfileBySlug(slug: string): Promise<SubstackFullProfile>;
}
