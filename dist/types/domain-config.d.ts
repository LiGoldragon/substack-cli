/**
 * Configuration interfaces for the Substack API client
 */
export interface SubstackConfig {
    hostname: string;
    apiVersion?: string;
    apiKey: string;
    perPage?: number;
    cacheTTL?: number;
    protocol?: 'http' | 'https';
    substackBaseUrl?: string;
}
export interface PaginationParams {
    limit?: number;
    offset?: number;
}
export interface SearchParams extends PaginationParams {
    query: string;
    sort?: 'top' | 'new';
    author?: string;
}
