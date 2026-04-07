import type { HttpClient } from '../http-client';
/**
 * Service responsible for followee-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
export declare class FolloweeService {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Get users that the authenticated user follows
     * @returns Promise<number[]> - Array of user IDs that the user follows
     * @throws {Error} When following list cannot be retrieved
     */
    getFollowees(): Promise<number[]>;
}
