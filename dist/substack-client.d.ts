import { Profile, OwnProfile, FullPost, Note, Comment } from './domain';
import type { SubstackConfig } from './types';
/**
 * Modern SubstackClient with entity-based API
 */
export declare class SubstackClient {
    private readonly publicationClient;
    private readonly substackClient;
    private readonly postService;
    private readonly noteService;
    private readonly profileService;
    private readonly slugService;
    private readonly commentService;
    private readonly followeeService;
    private readonly connectivityService;
    constructor(config: SubstackConfig);
    /**
     * Test API connectivity
     */
    testConnectivity(): Promise<boolean>;
    /**
     * Get the authenticated user's own profile with write capabilities
     * @throws {Error} When authentication fails or user profile cannot be retrieved
     */
    ownProfile(): Promise<OwnProfile>;
    /**
     * Get a profile by user ID
     */
    profileForId(id: number): Promise<Profile>;
    /**
     * Get a profile by handle/slug
     */
    profileForSlug(slug: string): Promise<Profile>;
    /**
     * Get a specific post by ID
     */
    postForId(id: number): Promise<FullPost>;
    /**
     * Get a specific note by ID
     */
    noteForId(id: number): Promise<Note>;
    /**
     * Get a specific comment by ID
     */
    commentForId(id: number): Promise<Comment>;
}
