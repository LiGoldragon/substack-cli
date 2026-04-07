"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FolloweeService = void 0;
/**
 * Service responsible for followee-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
class FolloweeService {
    constructor(httpClient) {
        this.httpClient = httpClient;
    }
    /**
     * Get users that the authenticated user follows
     * @returns Promise<number[]> - Array of user IDs that the user follows
     * @throws {Error} When following list cannot be retrieved
     */
    async getFollowees() {
        return await this.httpClient.get('/api/v1/feed/following');
    }
}
exports.FolloweeService = FolloweeService;
