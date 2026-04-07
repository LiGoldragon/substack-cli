import type { SubstackComment } from '../internal';
import type { HttpClient } from '../internal/http-client';
/**
 * Comment entity representing a comment on a post or note
 */
export declare class Comment {
    private readonly rawData;
    private readonly client;
    readonly id: number;
    readonly body: string;
    readonly author: {
        id: number;
        name: string;
        isAdmin?: boolean;
    };
    readonly createdAt: Date;
    readonly likesCount?: number;
    constructor(rawData: SubstackComment, client: HttpClient);
}
