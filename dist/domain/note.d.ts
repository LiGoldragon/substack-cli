import type { SubstackNote } from '../internal';
import type { HttpClient } from '../internal/http-client';
import { Comment } from './comment';
/**
 * Note entity representing a Substack note
 */
export declare class Note {
    private readonly rawData;
    private readonly client;
    readonly id: string;
    readonly body: string;
    readonly likesCount: number;
    readonly author: {
        id: number;
        name: string;
        handle: string;
        avatarUrl: string;
    };
    readonly publishedAt: Date;
    constructor(rawData: SubstackNote, client: HttpClient);
    /**
     * Get parent comments for this note
     */
    comments(): AsyncIterable<Comment>;
    /**
     * Like this note
     */
    like(): Promise<void>;
    /**
     * Add a comment to this note
     */
    addComment(_text: string): Promise<Comment>;
}
