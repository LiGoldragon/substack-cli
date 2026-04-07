import type { SubstackPost, SubstackFullPost } from '../internal';
import type { HttpClient } from '../internal/http-client';
import type { CommentService, PostService } from '../internal/services';
import { Comment } from './comment';
/**
 * PreviewPost entity representing a Substack post with truncated content
 */
export declare class PreviewPost {
    private readonly client;
    private readonly commentService;
    private readonly postService;
    readonly id: number;
    readonly title: string;
    readonly subtitle: string;
    readonly body: string;
    readonly truncatedBody: string;
    readonly likesCount: number;
    readonly author: {
        id: number;
        name: string;
        handle: string;
        avatarUrl: string;
    };
    readonly publishedAt: Date;
    constructor(rawData: SubstackPost, client: HttpClient, commentService: CommentService, postService: PostService);
    /**
     * Fetch the full post data with HTML body content
     * @returns Promise<FullPost> - A FullPost instance with complete content
     * @throws {Error} When full post retrieval fails
     */
    fullPost(): Promise<FullPost>;
    /**
     * Get comments for this post
     * @throws {Error} When comment retrieval fails or API is unavailable
     */
    comments(options?: {
        limit?: number;
    }): AsyncIterable<Comment>;
    /**
     * Like this post
     */
    like(): Promise<void>;
    /**
     * Add a comment to this post
     */
    addComment(_data: {
        body: string;
    }): Promise<Comment>;
}
/**
 * FullPost entity representing a Substack post with complete HTML content
 */
export declare class FullPost extends PreviewPost {
    readonly htmlBody: string;
    readonly slug: string;
    readonly createdAt: Date;
    readonly reactions?: Record<string, number>;
    readonly restacks?: number;
    readonly postTags?: string[];
    readonly coverImage?: string;
    constructor(rawData: SubstackFullPost, client: HttpClient, commentService: CommentService, postService: PostService);
}
