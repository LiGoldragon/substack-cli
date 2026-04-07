"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.InMemoryCache = void 0;
/**
 * Simple in-memory cache implementation using Map
 */
class InMemoryCache {
    constructor() {
        this.storage = new Map();
    }
    has(key) {
        return this.storage.has(key);
    }
    get(key) {
        return this.storage.get(key);
    }
    set(key, value) {
        this.storage.set(key, value);
    }
    clear() {
        this.storage.clear();
    }
    size() {
        return this.storage.size;
    }
}
exports.InMemoryCache = InMemoryCache;
