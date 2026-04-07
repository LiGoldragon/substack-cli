import { PublishNoteRequest, PublishNoteResponse } from '../internal';
import { HttpClient } from '../internal/http-client';
interface TextSegment {
    text: string;
    type: 'bold' | 'italic' | 'code' | 'underline' | 'link' | 'simple';
    url?: string;
}
interface ListItem {
    segments: TextSegment[];
}
interface List {
    type: 'bullet' | 'numbered';
    items: ListItem[];
}
export type { TextSegment, ListItem, List };
interface ListItemBuilderState {
    segments: TextSegment[];
}
interface ListBuilderState {
    type: 'bullet' | 'numbered';
    items: ListItem[];
}
interface ParagraphBuilderState {
    segments: TextSegment[];
    lists: List[];
}
interface NoteBuilderState {
    paragraphs: Array<{
        segments: TextSegment[];
        lists: List[];
    }>;
    attachmentIds?: string[];
}
/**
 * Builder for constructing list items - similar to paragraph but no nested lists allowed
 */
export declare class ListItemBuilder {
    private readonly listBuilder;
    private readonly state;
    constructor(listBuilder: ListBuilder, state?: ListItemBuilderState);
    /**
     * Add plain text to the current list item
     */
    text(text: string): ListItemBuilder;
    /**
     * Add bold text to the current list item
     */
    bold(text: string): ListItemBuilder;
    /**
     * Add italic text to the current list item
     */
    italic(text: string): ListItemBuilder;
    /**
     * Add code text to the current list item
     */
    code(text: string): ListItemBuilder;
    /**
     * Add underlined text to the current list item
     */
    underline(text: string): ListItemBuilder;
    /**
     * Add a link to the current list item
     */
    link(text: string, url: string): ListItemBuilder;
    /**
     * Get the current segments (used by ListBuilder)
     */
    getSegments(): TextSegment[];
    /**
     * Return to the list builder to add another item
     */
    item(): ListItemBuilder;
    /**
     * Finish the list and return to paragraph
     */
    finish(): ParagraphBuilder;
}
/**
 * Builder for constructing lists within a paragraph
 */
export declare class ListBuilder {
    private readonly paragraphBuilder;
    private readonly state;
    constructor(type: 'bullet' | 'numbered', paragraphBuilder: ParagraphBuilder, state?: ListBuilderState);
    /**
     * Add an item to the current list
     */
    addItem(item: ListItem): ListBuilder;
    /**
     * Start a new list item
     */
    item(): ListItemBuilder;
    /**
     * Finish the list and return to paragraph
     */
    finish(): ParagraphBuilder;
}
/**
 * Builder for constructing rich text within a paragraph
 */
export declare class ParagraphBuilder {
    private readonly noteBuilder;
    private readonly state;
    constructor(noteBuilder: NoteBuilder, state?: ParagraphBuilderState);
    /**
     * Add plain text to the current paragraph
     */
    text(text: string): ParagraphBuilder;
    /**
     * Add bold text to the current paragraph
     */
    bold(text: string): ParagraphBuilder;
    /**
     * Add italic text to the current paragraph
     */
    italic(text: string): ParagraphBuilder;
    /**
     * Add code text to the current paragraph
     */
    code(text: string): ParagraphBuilder;
    /**
     * Add underlined text to the current paragraph
     */
    underline(text: string): ParagraphBuilder;
    /**
     * Add a link to the current paragraph
     */
    link(text: string, url: string): ParagraphBuilder;
    /**
     * Start a bullet list in the current paragraph
     */
    bulletList(): ListBuilder;
    /**
     * Start a numbered list in the current paragraph
     */
    numberedList(): ListBuilder;
    /**
     * Add a list to the current paragraph (used by ListBuilder)
     */
    addList(list: List): ParagraphBuilder;
    /**
     * Get the current paragraph content (used by NoteBuilder)
     */
    getParagraphContent(): {
        segments: TextSegment[];
        lists: List[];
    };
    /**
     * Start a new paragraph
     */
    paragraph(): ParagraphBuilder;
    /**
     * Build and validate the note
     */
    build(): PublishNoteRequest;
    /**
     * Publish the note directly
     */
    publish(): Promise<PublishNoteResponse>;
}
export declare class NoteBuilder {
    protected readonly client: HttpClient;
    protected readonly state: NoteBuilderState;
    constructor(client: HttpClient, state?: NoteBuilderState);
    /**
     * Add a paragraph to the note (used by ParagraphBuilder)
     */
    addParagraph(paragraph: {
        segments: TextSegment[];
        lists: List[];
    }): NoteBuilder;
    /**
     * Start a paragraph
     */
    paragraph(): ParagraphBuilder;
    /**
     * Convert the builder's content to Substack's note format
     */
    private toNoteRequest;
    /**
     * Convert a text segment to Substack content format
     */
    protected segmentToContent(segment: TextSegment): {
        type: "text";
        text: string;
    } | {
        marks: {
            type: "link";
            attrs: {
                href: string;
            };
        }[];
        type: "text";
        text: string;
    } | {
        marks: {
            type: "bold" | "italic" | "code" | "underline";
        }[];
        type: "text";
        text: string;
    };
    /**
     * Build and validate the note
     */
    build(): PublishNoteRequest;
    /**
     * Publish the note
     */
    publish(): Promise<PublishNoteResponse>;
}
/**
 * Extended NoteBuilder that creates an attachment for a link and publishes the note with the attachment
 */
export declare class NoteWithLinkBuilder extends NoteBuilder {
    private readonly linkUrl;
    constructor(client: HttpClient, linkUrl: string);
    /**
     * Add a paragraph to the note (used by ParagraphBuilder) - returns NoteWithLinkBuilder to preserve attachment logic
     */
    addParagraph(paragraph: {
        segments: TextSegment[];
        lists: List[];
    }): NoteWithLinkBuilder;
    /**
     * Copy state to new instance - helper method
     */
    private copyState;
    /**
     * Publish the note with the link attachment
     */
    publish(): Promise<PublishNoteResponse>;
    /**
     * Convert the builder's content to Substack's note format with custom state
     */
    private toNoteRequestWithState;
}
