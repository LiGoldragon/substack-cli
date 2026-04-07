"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConnectivityService = void 0;
/**
 * Service responsible for checking API connectivity and session validity
 * Provides a clean boolean indicator of whether the API is accessible
 */
class ConnectivityService {
    constructor(httpClient) {
        this.httpClient = httpClient;
    }
    /**
     * Check if the API is connected and accessible
     * Uses a lightweight endpoint to verify connectivity without side effects
     * @returns Promise<boolean> - true if API is accessible, false otherwise
     */
    async isConnected() {
        try {
            await this.httpClient.get('/api/v1/feed/following');
            return true;
        }
        catch (_a) {
            return false;
        }
    }
}
exports.ConnectivityService = ConnectivityService;
