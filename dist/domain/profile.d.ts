import type { SubstackPublicProfile, SubstackFullProfile } from '../internal';
import type { HttpClient } from '../internal/http-client';
import type { ProfileService, CommentService, PostService, NoteService } from '../internal/services';
import { PreviewPost } from './post';
import { Note } from './note';
/**
 * Base Profile class representing a Substack user profile (read-only)
 */
export declare class Profile {
    protected readonly rawData: SubstackPublicProfile | SubstackFullProfile;
    protected readonly client: HttpClient;
    protected readonly profileService: ProfileService;
    protected readonly postService: PostService;
    protected readonly noteService: NoteService;
    protected readonly commentService: CommentService;
    protected readonly slugResolver?: ((userId: number, fallbackHandle?: string) => Promise<string | undefined>) | undefined;
    readonly id: number;
    readonly slug: string;
    readonly name: string;
    readonly url: string;
    readonly avatarUrl: string;
    readonly bio?: string;
    constructor(rawData: SubstackPublicProfile | SubstackFullProfile, client: HttpClient, profileService: ProfileService, postService: PostService, noteService: NoteService, commentService: CommentService, resolvedSlug?: string, slugResolver?: ((userId: number, fallbackHandle?: string) => Promise<string | undefined>) | undefined);
    /**
     * Get posts from this profile's publications
     */
    posts(options?: {
        limit?: number;
    }): AsyncIterable<PreviewPost>;
    /**
     * Get notes from this profile
     */
    notes(options?: {
        limit?: number;
    }): AsyncIterable<Note>;
}
