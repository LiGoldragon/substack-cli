import type { HttpClient } from '../http-client';
/**
 * Service responsible for checking API connectivity and session validity
 * Provides a clean boolean indicator of whether the API is accessible
 */
export declare class ConnectivityService {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Check if the API is connected and accessible
     * Uses a lightweight endpoint to verify connectivity without side effects
     * @returns Promise<boolean> - true if API is accessible, false otherwise
     */
    isConnected(): Promise<boolean>;
}
