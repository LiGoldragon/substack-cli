import type { SubstackNote, PaginatedSubstackNotes } from '../types';
import type { HttpClient } from '../http-client';
/**
 * Service responsible for note-related HTTP operations
 * Returns internal types that can be transformed into domain models
 */
export declare class NoteService {
    private readonly httpClient;
    constructor(httpClient: HttpClient);
    /**
     * Get a note by ID from the API
     * @param id - The note ID
     * @returns Promise<SubstackNote> - Raw note data from API
     * @throws {Error} When note is not found or API request fails
     */
    getNoteById(id: number): Promise<SubstackNote>;
    /**
     * Get notes for the authenticated user with cursor-based pagination
     * @param options - Pagination options with optional cursor
     * @returns Promise<PaginatedSubstackNotes> - Paginated note data from API
     * @throws {Error} When notes cannot be retrieved
     */
    getNotesForLoggedUser(options?: {
        cursor?: string;
    }): Promise<PaginatedSubstackNotes>;
    /**
     * Get notes for a profile with cursor-based pagination
     * @param profileId - The profile user ID
     * @param options - Pagination options with optional cursor
     * @returns Promise<PaginatedSubstackNotes> - Paginated note data from API
     * @throws {Error} When notes cannot be retrieved
     */
    getNotesForProfile(profileId: number, options?: {
        cursor?: string;
    }): Promise<PaginatedSubstackNotes>;
}
