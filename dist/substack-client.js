"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SubstackClient = void 0;
const http_client_1 = require("./internal/http-client");
const domain_1 = require("./domain");
const services_1 = require("./internal/services");
const cache_1 = require("./internal/cache");
/**
 * Modern SubstackClient with entity-based API
 */
class SubstackClient {
    constructor(config) {
        // Create HTTP client for publication-specific endpoints
        const protocol = config.protocol || 'https';
        const publicationBaseUrl = `${protocol}://${config.hostname || 'substack.com'}`;
        this.publicationClient = new http_client_1.HttpClient(publicationBaseUrl, config);
        // Create HTTP client for global Substack endpoints
        const substackBaseUrl = config.substackBaseUrl || 'https://substack.com';
        this.substackClient = new http_client_1.HttpClient(substackBaseUrl, config);
        // Initialize services
        this.postService = new services_1.PostService(this.substackClient, this.publicationClient);
        this.noteService = new services_1.NoteService(this.publicationClient);
        this.profileService = new services_1.ProfileService(this.publicationClient);
        // Create caching slug service using decorator pattern
        const baseSlugService = new services_1.SlugService(this.publicationClient);
        const slugCache = new cache_1.InMemoryCache();
        this.slugService = new services_1.CachingSlugService(slugCache, baseSlugService);
        this.commentService = new services_1.CommentService(this.publicationClient);
        this.followeeService = new services_1.FolloweeService(this.publicationClient);
        this.connectivityService = new services_1.ConnectivityService(this.publicationClient);
    }
    /**
     * Test API connectivity
     */
    async testConnectivity() {
        return await this.connectivityService.isConnected();
    }
    /**
     * Get the authenticated user's own profile with write capabilities
     * @throws {Error} When authentication fails or user profile cannot be retrieved
     */
    async ownProfile() {
        try {
            const profile = await this.profileService.getOwnProfile();
            // Get user_id for slug resolution
            const subscription = await this.publicationClient.get('/api/v1/subscription');
            const userId = subscription.user_id;
            // Resolve slug from slug service
            const resolvedSlug = await this.slugService.getSlugForUserId(userId, profile.handle);
            return new domain_1.OwnProfile(profile, this.publicationClient, this.profileService, this.postService, this.noteService, this.commentService, this.followeeService, resolvedSlug, this.slugService.getSlugForUserId.bind(this.slugService));
        }
        catch (error) {
            throw new Error(`Failed to get own profile: ${error.message}`);
        }
    }
    /**
     * Get a profile by user ID
     */
    async profileForId(id) {
        try {
            const profile = await this.profileService.getProfileById(id);
            // Resolve slug from slug service
            const resolvedSlug = await this.slugService.getSlugForUserId(id, profile.handle);
            return new domain_1.Profile(profile, this.publicationClient, this.profileService, this.postService, this.noteService, this.commentService, resolvedSlug, this.slugService.getSlugForUserId.bind(this.slugService));
        }
        catch (error) {
            throw new Error(`Profile with ID ${id} not found: ${error.message}`);
        }
    }
    /**
     * Get a profile by handle/slug
     */
    async profileForSlug(slug) {
        if (!slug || slug.trim() === '') {
            throw new Error('Profile slug cannot be empty');
        }
        try {
            const profile = await this.profileService.getProfileBySlug(slug);
            // For profiles fetched by slug, we can use the provided slug as the resolved slug
            // but still check slug service for consistency
            const resolvedSlug = await this.slugService.getSlugForUserId(profile.id, slug);
            return new domain_1.Profile(profile, this.publicationClient, this.profileService, this.postService, this.noteService, this.commentService, resolvedSlug, this.slugService.getSlugForUserId.bind(this.slugService));
        }
        catch (error) {
            throw new Error(`Profile with slug '${slug}' not found: ${error.message}`);
        }
    }
    /**
     * Get a specific post by ID
     */
    async postForId(id) {
        if (typeof id !== 'number') {
            throw new TypeError('Post ID must be a number');
        }
        try {
            const post = await this.postService.getPostById(id);
            return new domain_1.FullPost(post, this.publicationClient, this.commentService, this.postService);
        }
        catch (error) {
            throw new Error(`Post with ID ${id} not found: ${error.message}`);
        }
    }
    /**
     * Get a specific note by ID
     */
    async noteForId(id) {
        if (typeof id !== 'number') {
            throw new TypeError('Note ID must be a number');
        }
        try {
            const noteData = await this.noteService.getNoteById(id);
            return new domain_1.Note(noteData, this.publicationClient);
        }
        catch (_a) {
            throw new Error(`Note with ID ${id} not found`);
        }
    }
    /**
     * Get a specific comment by ID
     */
    async commentForId(id) {
        if (typeof id !== 'number') {
            throw new TypeError('Comment ID must be a number');
        }
        try {
            const commentData = await this.commentService.getCommentById(id);
            return new domain_1.Comment(commentData, this.publicationClient);
        }
        catch (error) {
            throw new Error(`Comment with ID ${id} not found: ${error.message}`);
        }
    }
}
exports.SubstackClient = SubstackClient;
