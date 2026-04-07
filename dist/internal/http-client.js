"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HttpClient = void 0;
class HttpClient {
    constructor(baseUrl, config) {
        if (!config.apiKey) {
            throw new Error('apiKey is required in SubstackConfig');
        }
        this.baseUrl = baseUrl;
        this.cookie = `connect.sid=${config.apiKey}`;
        this.perPage = config.perPage || 25;
    }
    /**
     * Get the configured items per page for pagination
     */
    getPerPage() {
        return this.perPage;
    }
    async makeRequest(url, options = {}) {
        const response = await fetch(url, {
            headers: {
                Cookie: this.cookie,
                'Content-Type': 'application/json',
                ...options.headers
            },
            ...options
        });
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        return response.json();
    }
    async request(path, options = {}) {
        const url = `${this.baseUrl}${path}`;
        return this.makeRequest(url, options);
    }
    async get(path) {
        return this.request(path);
    }
    async post(path, data) {
        return this.request(path, {
            method: 'POST',
            body: data ? JSON.stringify(data) : undefined
        });
    }
}
exports.HttpClient = HttpClient;
