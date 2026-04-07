"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Profile = void 0;
const post_1 = require("./post");
const note_1 = require("./note");
/**
 * Base Profile class representing a Substack user profile (read-only)
 */
class Profile {
    constructor(rawData, client, profileService, postService, noteService, commentService, resolvedSlug, slugResolver) {
        this.rawData = rawData;
        this.client = client;
        this.profileService = profileService;
        this.postService = postService;
        this.noteService = noteService;
        this.commentService = commentService;
        this.slugResolver = slugResolver;
        this.id = rawData.id;
        // Use resolved slug from subscriptions cache if available, otherwise fallback to handle
        this.slug = resolvedSlug || rawData.handle;
        this.name = rawData.name;
        this.url = `https://substack.com/@${this.slug}`;
        this.avatarUrl = rawData.photo_url;
        this.bio = rawData.bio;
    }
    /**
     * Get posts from this profile's publications
     */
    async *posts(options = {}) {
        try {
            // Get the perPage configuration from the client
            const perPageConfig = this.client.getPerPage();
            let offset = 0;
            let totalYielded = 0;
            while (true) {
                // Use PostService to get posts
                const postsData = await this.postService.getPostsForProfile(this.id, {
                    limit: perPageConfig,
                    offset
                });
                if (!postsData) {
                    break; // No more posts to fetch
                }
                for (const postData of postsData) {
                    if (options.limit && totalYielded >= options.limit) {
                        return; // Stop if we've reached the requested limit
                    }
                    yield new post_1.PreviewPost(postData, this.client, this.commentService, this.postService);
                    totalYielded++;
                }
                // If we got fewer posts than requested, we've reached the end
                if (postsData.length < perPageConfig) {
                    break;
                }
                offset += perPageConfig;
            }
        }
        catch (_a) {
            // If the endpoint doesn't exist or fails, return empty iterator
            yield* [];
        }
    }
    /**
     * Get notes from this profile
     */
    async *notes(options = {}) {
        try {
            let cursor = undefined;
            let totalYielded = 0;
            while (true) {
                // Use NoteService to get notes for this profile with cursor-based pagination
                const paginatedNotes = await this.noteService.getNotesForProfile(this.id, {
                    cursor
                });
                if (!paginatedNotes.notes) {
                    break; // No more notes to fetch
                }
                for (const item of paginatedNotes.notes) {
                    if (options.limit && totalYielded >= options.limit) {
                        return; // Stop if we've reached the requested limit
                    }
                    yield new note_1.Note(item, this.client);
                    totalYielded++;
                }
                // If there's no next cursor, we've reached the end
                if (!paginatedNotes.nextCursor) {
                    break;
                }
                cursor = paginatedNotes.nextCursor;
            }
        }
        catch (_a) {
            // If both endpoints fail, return empty iterator
            yield* [];
        }
    }
}
exports.Profile = Profile;
