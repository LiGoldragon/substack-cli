import { Profile } from './profile';
import { Note } from './note';
import { NoteBuilder, NoteWithLinkBuilder } from './note-builder';
import type { SubstackFullProfile } from '../internal';
import type { HttpClient } from '../internal/http-client';
import type { ProfileService, PostService, NoteService, FolloweeService, CommentService } from '../internal/services';
/**
 * OwnProfile extends Profile with write capabilities for the authenticated user
 */
export declare class OwnProfile extends Profile {
    private readonly followeeService;
    constructor(rawData: SubstackFullProfile, client: HttpClient, profileService: ProfileService, postService: PostService, noteService: NoteService, commentService: CommentService, followeeService: FolloweeService, resolvedSlug?: string, slugResolver?: (userId: number, fallbackHandle?: string) => Promise<string | undefined>);
    /**
     * Create a new note using the builder pattern
     */
    newNote(): NoteBuilder;
    /**
     * Create a new note with a link attachment using the builder pattern
     */
    newNoteWithLink(link: string): NoteWithLinkBuilder;
    /**
     * Get users that the authenticated user follows
     */
    followees(options?: {
        limit?: number;
    }): AsyncIterable<Profile>;
    /**
     * Get notes from the authenticated user's profile
     */
    notes(options?: {
        limit?: number;
    }): AsyncIterable<Note>;
}
