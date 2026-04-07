"use strict";
/**
 * Internal API response types - not exported from the public API
 * These represent raw response shapes from Substack's API endpoints
 * Using io-ts codecs for runtime validation
 */
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.SubstackSearchResultCodec = exports.SubstackCommentResponseCodec = exports.SubstackCommentCodec = exports.SubstackFullPostCodec = exports.SubstackPostCodec = exports.SubstackPublicationCodec = void 0;
const t = __importStar(require("io-ts"));
/**
 * Raw API response shape for publications - flattened
 */
exports.SubstackPublicationCodec = t.intersection([
    t.type({
        name: t.string,
        hostname: t.string,
        subdomain: t.string
    }),
    t.partial({
        logo_url: t.string,
        description: t.string
    })
]);
/**
 * Raw API response shape for posts
 */
exports.SubstackPostCodec = t.intersection([
    t.type({
        id: t.number,
        title: t.string,
        slug: t.string,
        post_date: t.string,
        canonical_url: t.string,
        type: t.union([t.literal('newsletter'), t.literal('podcast'), t.literal('thread')])
    }),
    t.partial({
        subtitle: t.string,
        description: t.string,
        audience: t.string,
        cover_image: t.string,
        podcast_url: t.string,
        published: t.boolean,
        paywalled: t.boolean,
        truncated_body_text: t.string,
        htmlBody: t.string
    })
]);
/**
 * Raw API response shape for full posts from /posts/by-id/:id endpoint
 * Includes body_html and additional fields not present in preview responses
 *
 * Key differences from SubstackPostCodec:
 * - body_html is required (contains full HTML content)
 * - Includes postTags, reactions, restacks, and publication fields
 * - Used specifically for FullPost construction via getPostById()
 * - SubstackPostCodec should be used for preview/list responses
 */
exports.SubstackFullPostCodec = t.intersection([
    t.type({
        id: t.number,
        title: t.string,
        slug: t.string,
        post_date: t.string,
        canonical_url: t.string,
        type: t.union([t.literal('newsletter'), t.literal('podcast'), t.literal('thread')]),
        body_html: t.string
    }),
    t.partial({
        subtitle: t.string,
        description: t.string,
        audience: t.string,
        cover_image: t.string,
        podcast_url: t.string,
        published: t.boolean,
        paywalled: t.boolean,
        truncated_body_text: t.string,
        htmlBody: t.string, // Legacy field for backward compatibility
        postTags: t.array(t.string),
        reactions: t.record(t.string, t.number),
        restacks: t.number,
        publication: t.type({
            id: t.number,
            name: t.string,
            subdomain: t.string
        })
    })
]);
/**
 * Raw API response shape for comments - flattened
 */
exports.SubstackCommentCodec = t.intersection([
    t.type({
        id: t.number,
        body: t.string,
        created_at: t.string,
        parent_post_id: t.number,
        author_id: t.number,
        author_name: t.string
    }),
    t.partial({
        author_is_admin: t.boolean
    })
]);
/**
 * Response structure from /api/v1/reader/comment/{id} endpoint - keeping wrapper structure
 */
exports.SubstackCommentResponseCodec = t.type({
    item: t.type({
        comment: t.intersection([
            t.type({
                id: t.number,
                body: t.string,
                user_id: t.number,
                name: t.string,
                date: t.string
            }),
            t.partial({
                post_id: t.union([t.number, t.null])
            })
        ])
    })
});
/**
 * Raw API response shape for search results
 */
exports.SubstackSearchResultCodec = t.type({
    total: t.number,
    results: t.array(exports.SubstackPostCodec)
});
