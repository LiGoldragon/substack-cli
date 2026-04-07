"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OwnProfile = void 0;
const profile_1 = require("./profile");
const note_1 = require("./note");
const note_builder_1 = require("./note-builder");
/**
 * OwnProfile extends Profile with write capabilities for the authenticated user
 */
class OwnProfile extends profile_1.Profile {
    constructor(rawData, client, profileService, postService, noteService, commentService, followeeService, resolvedSlug, slugResolver) {
        super(rawData, client, profileService, postService, noteService, commentService, resolvedSlug, slugResolver);
        this.followeeService = followeeService;
    }
    /**
     * Create a new note using the builder pattern
     */
    newNote() {
        return new note_builder_1.NoteBuilder(this.client);
    }
    /**
     * Create a new note with a link attachment using the builder pattern
     */
    newNoteWithLink(link) {
        return new note_builder_1.NoteWithLinkBuilder(this.client, link);
    }
    /**
     * Get users that the authenticated user follows
     */
    async *followees(options = {}) {
        // Use FolloweeService to get the list of user IDs that the user follows
        const followingUserIds = await this.followeeService.getFollowees();
        // Then, for each user ID, fetch their detailed profile
        let count = 0;
        for (const userId of followingUserIds) {
            if (options.limit && count >= options.limit)
                break;
            try {
                const profileResponse = await this.profileService.getProfileById(userId);
                // Use the same slug resolution as the main client if available
                let resolvedSlug = profileResponse.handle;
                if (this.slugResolver) {
                    resolvedSlug =
                        (await this.slugResolver(userId, profileResponse.handle)) || profileResponse.handle;
                }
                yield new profile_1.Profile(profileResponse, this.client, this.profileService, this.postService, this.noteService, this.commentService, resolvedSlug, this.slugResolver);
                count++;
            }
            catch (_a) {
                // Skip profiles that can't be fetched (e.g., deleted accounts, private profiles)
                // This ensures the iterator continues working even if some profiles are inaccessible
                continue;
            }
        }
    }
    /**
     * Get notes from the authenticated user's profile
     */
    async *notes(options = {}) {
        try {
            let cursor = undefined;
            let totalYielded = 0;
            while (true) {
                // Use NoteService to fetch notes for the authenticated user with cursor-based pagination
                const paginatedNotes = await this.noteService.getNotesForLoggedUser({
                    cursor
                });
                if (!paginatedNotes.notes) {
                    break; // No more notes to fetch
                }
                for (const noteData of paginatedNotes.notes) {
                    if (options.limit && totalYielded >= options.limit) {
                        return; // Stop if we've reached the requested limit
                    }
                    yield new note_1.Note(noteData, this.client);
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
            // If the endpoint doesn't exist or fails, return empty iterator
            yield* [];
        }
    }
}
exports.OwnProfile = OwnProfile;
