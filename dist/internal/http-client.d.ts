/**
 * HTTP client utility for Substack API requests
 */
import type { SubstackConfig } from '../types';
export declare class HttpClient {
    private readonly baseUrl;
    private readonly cookie;
    private readonly perPage;
    constructor(baseUrl: string, config: SubstackConfig);
    /**
     * Get the configured items per page for pagination
     */
    getPerPage(): number;
    private makeRequest;
    request<T>(path: string, options?: RequestInit): Promise<T>;
    get<T>(path: string): Promise<T>;
    post<T>(path: string, data?: unknown): Promise<T>;
}
