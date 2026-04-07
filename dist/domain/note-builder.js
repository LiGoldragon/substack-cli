"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.NoteWithLinkBuilder = exports.NoteBuilder = exports.ParagraphBuilder = exports.ListBuilder = exports.ListItemBuilder = void 0;
/**
 * Builder for constructing list items - similar to paragraph but no nested lists allowed
 */
class ListItemBuilder {
    constructor(listBuilder, state = { segments: [] }) {
        this.listBuilder = listBuilder;
        this.state = state;
    }
    /**
     * Add plain text to the current list item
     */
    text(text) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'simple' }]
        });
    }
    /**
     * Add bold text to the current list item
     */
    bold(text) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'bold' }]
        });
    }
    /**
     * Add italic text to the current list item
     */
    italic(text) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'italic' }]
        });
    }
    /**
     * Add code text to the current list item
     */
    code(text) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'code' }]
        });
    }
    /**
     * Add underlined text to the current list item
     */
    underline(text) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'underline' }]
        });
    }
    /**
     * Add a link to the current list item
     */
    link(text, url) {
        return new ListItemBuilder(this.listBuilder, {
            segments: [...this.state.segments, { text, type: 'link', url }]
        });
    }
    /**
     * Get the current segments (used by ListBuilder)
     */
    getSegments() {
        return this.state.segments;
    }
    /**
     * Return to the list builder to add another item
     */
    item() {
        // Commit current item and create new one
        return this.listBuilder.addItem({ segments: this.state.segments }).item();
    }
    /**
     * Finish the list and return to paragraph
     */
    finish() {
        // Commit current item
        return this.listBuilder.addItem({ segments: this.state.segments }).finish();
    }
}
exports.ListItemBuilder = ListItemBuilder;
/**
 * Builder for constructing lists within a paragraph
 */
class ListBuilder {
    constructor(type, paragraphBuilder, state) {
        this.paragraphBuilder = paragraphBuilder;
        this.state = state || { type, items: [] };
    }
    /**
     * Add an item to the current list
     */
    addItem(item) {
        return new ListBuilder(this.state.type, this.paragraphBuilder, {
            type: this.state.type,
            items: [...this.state.items, item]
        });
    }
    /**
     * Start a new list item
     */
    item() {
        return new ListItemBuilder(this);
    }
    /**
     * Finish the list and return to paragraph
     */
    finish() {
        // Add the completed list to the paragraph
        return this.paragraphBuilder.addList({ type: this.state.type, items: this.state.items });
    }
}
exports.ListBuilder = ListBuilder;
/**
 * Builder for constructing rich text within a paragraph
 */
class ParagraphBuilder {
    constructor(noteBuilder, state = { segments: [], lists: [] }) {
        this.noteBuilder = noteBuilder;
        this.state = state;
    }
    /**
     * Add plain text to the current paragraph
     */
    text(text) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'simple' }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Add bold text to the current paragraph
     */
    bold(text) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'bold' }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Add italic text to the current paragraph
     */
    italic(text) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'italic' }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Add code text to the current paragraph
     */
    code(text) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'code' }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Add underlined text to the current paragraph
     */
    underline(text) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'underline' }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Add a link to the current paragraph
     */
    link(text, url) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments, { text, type: 'link', url }],
            lists: [...this.state.lists]
        });
    }
    /**
     * Start a bullet list in the current paragraph
     */
    bulletList() {
        return new ListBuilder('bullet', this);
    }
    /**
     * Start a numbered list in the current paragraph
     */
    numberedList() {
        return new ListBuilder('numbered', this);
    }
    /**
     * Add a list to the current paragraph (used by ListBuilder)
     */
    addList(list) {
        return new ParagraphBuilder(this.noteBuilder, {
            segments: [...this.state.segments],
            lists: [...this.state.lists, list]
        });
    }
    /**
     * Get the current paragraph content (used by NoteBuilder)
     */
    getParagraphContent() {
        return { segments: this.state.segments, lists: this.state.lists };
    }
    /**
     * Start a new paragraph
     */
    paragraph() {
        // Commit the current paragraph
        return this.noteBuilder.addParagraph(this.getParagraphContent()).paragraph();
    }
    /**
     * Build and validate the note
     */
    build() {
        // Commit the current paragraph before building
        return this.noteBuilder.addParagraph(this.getParagraphContent()).build();
    }
    /**
     * Publish the note directly
     */
    async publish() {
        // Commit the current paragraph before publishing
        return this.noteBuilder.addParagraph(this.getParagraphContent()).publish();
    }
}
exports.ParagraphBuilder = ParagraphBuilder;
class NoteBuilder {
    constructor(client, state = { paragraphs: [] }) {
        this.client = client;
        this.state = state;
    }
    /**
     * Add a paragraph to the note (used by ParagraphBuilder)
     */
    addParagraph(paragraph) {
        return new NoteBuilder(this.client, {
            paragraphs: [...this.state.paragraphs, paragraph],
            attachmentIds: this.state.attachmentIds
        });
    }
    /**
     * Start a paragraph
     */
    paragraph() {
        return new ParagraphBuilder(this);
    }
    /**
     * Convert the builder's content to Substack's note format
     */
    toNoteRequest() {
        // Validation: must have at least one paragraph
        if (this.state.paragraphs.length === 0) {
            throw new Error('Note must contain at least one paragraph');
        }
        // Validation: each paragraph must have content
        for (const paragraph of this.state.paragraphs) {
            if (paragraph.segments.length === 0 && paragraph.lists.length === 0) {
                throw new Error('Each paragraph must contain at least one content block');
            }
        }
        const content = this.state.paragraphs.flatMap((paragraph) => {
            const elements = [];
            // Add paragraph content if it has segments
            if (paragraph.segments.length > 0) {
                elements.push({
                    type: 'paragraph',
                    content: paragraph.segments.map((segment) => this.segmentToContent(segment))
                });
            }
            // Add list content
            for (const list of paragraph.lists) {
                elements.push({
                    type: list.type === 'bullet' ? 'bulletList' : 'orderedList',
                    content: list.items.map((item) => ({
                        type: 'listItem',
                        content: [
                            {
                                type: 'paragraph',
                                content: item.segments.map((segment) => this.segmentToContent(segment))
                            }
                        ]
                    }))
                });
            }
            return elements;
        });
        const request = {
            bodyJson: {
                type: 'doc',
                attrs: {
                    schemaVersion: 'v1'
                },
                content
            },
            tabId: 'for-you',
            surface: 'feed',
            replyMinimumRole: 'everyone'
        };
        if (this.state.attachmentIds && this.state.attachmentIds.length > 0) {
            request.attachmentIds = this.state.attachmentIds;
        }
        return request;
    }
    /**
     * Convert a text segment to Substack content format
     */
    segmentToContent(segment) {
        const base = {
            type: 'text',
            text: segment.text
        };
        if (segment.type === 'simple') {
            return base;
        }
        if (segment.type === 'link') {
            if (!segment.url) {
                throw new Error('Link segments must have a URL');
            }
            return {
                ...base,
                marks: [{ type: 'link', attrs: { href: segment.url } }]
            };
        }
        // For other formatting types
        return {
            ...base,
            marks: [{ type: segment.type }]
        };
    }
    /**
     * Build and validate the note
     */
    build() {
        return this.toNoteRequest();
    }
    /**
     * Publish the note
     */
    async publish() {
        return this.client.post('/api/v1/comment/feed', this.toNoteRequest());
    }
}
exports.NoteBuilder = NoteBuilder;
/**
 * Extended NoteBuilder that creates an attachment for a link and publishes the note with the attachment
 */
class NoteWithLinkBuilder extends NoteBuilder {
    constructor(client, linkUrl) {
        super(client);
        this.linkUrl = linkUrl;
    }
    /**
     * Add a paragraph to the note (used by ParagraphBuilder) - returns NoteWithLinkBuilder to preserve attachment logic
     */
    addParagraph(paragraph) {
        return new NoteWithLinkBuilder(this.client, this.linkUrl).copyState({
            paragraphs: [...this.state.paragraphs, paragraph],
            attachmentIds: this.state.attachmentIds
        });
    }
    /**
     * Copy state to new instance - helper method
     */
    copyState(state) {
        const newBuilder = new NoteWithLinkBuilder(this.client, this.linkUrl);
        newBuilder.state = state;
        return newBuilder;
    }
    /**
     * Publish the note with the link attachment
     */
    async publish() {
        // First, create the attachment for the link
        const attachmentRequest = {
            url: this.linkUrl,
            type: 'link'
        };
        const attachmentResponse = await this.client.post('/api/v1/comment/attachment', attachmentRequest);
        // Update the state with the attachment ID
        const updatedState = {
            paragraphs: this.state.paragraphs,
            attachmentIds: [attachmentResponse.id]
        };
        // Create the request with attachment
        const request = this.toNoteRequestWithState(updatedState);
        // Publish the note with attachment
        return this.client.post('/api/v1/comment/feed', request);
    }
    /**
     * Convert the builder's content to Substack's note format with custom state
     */
    toNoteRequestWithState(state) {
        // Validation: must have at least one paragraph
        if (state.paragraphs.length === 0) {
            throw new Error('Note must contain at least one paragraph');
        }
        // Validation: each paragraph must have content
        for (const paragraph of state.paragraphs) {
            if (paragraph.segments.length === 0 && paragraph.lists.length === 0) {
                throw new Error('Each paragraph must contain at least one content block');
            }
        }
        const content = state.paragraphs.flatMap((paragraph) => {
            const elements = [];
            // Add paragraph content if it has segments
            if (paragraph.segments.length > 0) {
                elements.push({
                    type: 'paragraph',
                    content: paragraph.segments.map((segment) => this.segmentToContent(segment))
                });
            }
            // Add list content
            for (const list of paragraph.lists) {
                elements.push({
                    type: list.type === 'bullet' ? 'bulletList' : 'orderedList',
                    content: list.items.map((item) => ({
                        type: 'listItem',
                        content: [
                            {
                                type: 'paragraph',
                                content: item.segments.map((segment) => this.segmentToContent(segment))
                            }
                        ]
                    }))
                });
            }
            return elements;
        });
        const request = {
            bodyJson: {
                type: 'doc',
                attrs: {
                    schemaVersion: 'v1'
                },
                content
            },
            tabId: 'for-you',
            surface: 'feed',
            replyMinimumRole: 'everyone'
        };
        if (state.attachmentIds && state.attachmentIds.length > 0) {
            request.attachmentIds = state.attachmentIds;
        }
        return request;
    }
}
exports.NoteWithLinkBuilder = NoteWithLinkBuilder;
