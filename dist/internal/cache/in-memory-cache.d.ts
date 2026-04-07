import type { Cache } from './cache';
/**
 * Simple in-memory cache implementation using Map
 */
export declare class InMemoryCache<K, V> implements Cache<K, V> {
    private readonly storage;
    has(key: K): boolean;
    get(key: K): V | undefined;
    set(key: K, value: V): void;
    clear(): void;
    size(): number;
}
