/**
 * Internal API response types - not exported from the public API
 * These represent raw response shapes from Substack's API endpoints
 * Using io-ts codecs for runtime validation
 */
import * as t from 'io-ts';
/**
 * Raw API response shape for publications - flattened
 */
export declare const SubstackPublicationCodec: t.IntersectionC<[t.TypeC<{
    name: t.StringC;
    hostname: t.StringC;
    subdomain: t.StringC;
}>, t.PartialC<{
    logo_url: t.StringC;
    description: t.StringC;
}>]>;
export type SubstackPublication = t.TypeOf<typeof SubstackPublicationCodec>;
/**
 * Raw API response shape for posts
 */
export declare const SubstackPostCodec: t.IntersectionC<[t.TypeC<{
    id: t.NumberC;
    title: t.StringC;
    slug: t.StringC;
    post_date: t.StringC;
    canonical_url: t.StringC;
    type: t.UnionC<[t.LiteralC<"newsletter">, t.LiteralC<"podcast">, t.LiteralC<"thread">]>;
}>, t.PartialC<{
    subtitle: t.StringC;
    description: t.StringC;
    audience: t.StringC;
    cover_image: t.StringC;
    podcast_url: t.StringC;
    published: t.BooleanC;
    paywalled: t.BooleanC;
    truncated_body_text: t.StringC;
    htmlBody: t.StringC;
}>]>;
export type SubstackPost = t.TypeOf<typeof SubstackPostCodec>;
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
export declare const SubstackFullPostCodec: t.IntersectionC<[t.TypeC<{
    id: t.NumberC;
    title: t.StringC;
    slug: t.StringC;
    post_date: t.StringC;
    canonical_url: t.StringC;
    type: t.UnionC<[t.LiteralC<"newsletter">, t.LiteralC<"podcast">, t.LiteralC<"thread">]>;
    body_html: t.StringC;
}>, t.PartialC<{
    subtitle: t.StringC;
    description: t.StringC;
    audience: t.StringC;
    cover_image: t.StringC;
    podcast_url: t.StringC;
    published: t.BooleanC;
    paywalled: t.BooleanC;
    truncated_body_text: t.StringC;
    htmlBody: t.StringC;
    postTags: t.ArrayC<t.StringC>;
    reactions: t.RecordC<t.StringC, t.NumberC>;
    restacks: t.NumberC;
    publication: t.TypeC<{
        id: t.NumberC;
        name: t.StringC;
        subdomain: t.StringC;
    }>;
}>]>;
export type SubstackFullPost = t.TypeOf<typeof SubstackFullPostCodec>;
/**
 * Raw API response shape for comments - flattened
 */
export declare const SubstackCommentCodec: t.IntersectionC<[t.TypeC<{
    id: t.NumberC;
    body: t.StringC;
    created_at: t.StringC;
    parent_post_id: t.NumberC;
    author_id: t.NumberC;
    author_name: t.StringC;
}>, t.PartialC<{
    author_is_admin: t.BooleanC;
}>]>;
export type SubstackComment = t.TypeOf<typeof SubstackCommentCodec>;
/**
 * Response structure from /api/v1/reader/comment/{id} endpoint - keeping wrapper structure
 */
export declare const SubstackCommentResponseCodec: t.TypeC<{
    item: t.TypeC<{
        comment: t.IntersectionC<[t.TypeC<{
            id: t.NumberC;
            body: t.StringC;
            user_id: t.NumberC;
            name: t.StringC;
            date: t.StringC;
        }>, t.PartialC<{
            post_id: t.UnionC<[t.NumberC, t.NullC]>;
        }>]>;
    }>;
}>;
export type SubstackCommentResponse = t.TypeOf<typeof SubstackCommentResponseCodec>;
/**
 * Raw API response shape for search results
 */
export declare const SubstackSearchResultCodec: t.TypeC<{
    total: t.NumberC;
    results: t.ArrayC<t.IntersectionC<[t.TypeC<{
        id: t.NumberC;
        title: t.StringC;
        slug: t.StringC;
        post_date: t.StringC;
        canonical_url: t.StringC;
        type: t.UnionC<[t.LiteralC<"newsletter">, t.LiteralC<"podcast">, t.LiteralC<"thread">]>;
    }>, t.PartialC<{
        subtitle: t.StringC;
        description: t.StringC;
        audience: t.StringC;
        cover_image: t.StringC;
        podcast_url: t.StringC;
        published: t.BooleanC;
        paywalled: t.BooleanC;
        truncated_body_text: t.StringC;
        htmlBody: t.StringC;
    }>]>>;
}>;
export type SubstackSearchResult = t.TypeOf<typeof SubstackSearchResultCodec>;
/**
 * Subscription types for internal API responses
 */
export interface SubstackSubscription {
    id: number;
    user_id: number;
    publication_id: number;
    expiry: string | null;
    email_disabled: boolean;
    membership_state: string;
    type: string | null;
    gift_user_id: number | null;
    created_at: string;
    gifted_at: string | null;
    paused: string | null;
    is_group_parent: boolean;
    visibility: string;
    is_founding: boolean;
    is_favorite: boolean;
    podcast_rss_token: string;
    email_settings: Record<string, unknown> | null;
    section_podcasts_enabled: string[] | null;
}
export interface SubstackPublicationUser {
    id: number;
    publication_id: number;
    user_id: number;
    public: boolean;
    created_at: string;
    updated_at: string;
    public_rank: number;
    name: string | null;
    bio: string | null;
    photo_url: string | null;
    role: string;
    is_primary: boolean;
    show_badge: boolean | null;
    is_in_notes_feed: boolean;
    twitter_screen_name: string | null;
    email: string | null;
    primary_section_id: number | null;
}
export interface SubstackSubscriptionPublication {
    id: number;
    name: string;
    subdomain: string;
    custom_domain?: string | null;
    is_on_substack: boolean;
    author_id: number;
    author_handle: string;
    created_at: string;
    logo_url?: string;
    cover_photo_url?: string;
    twitter_screen_name?: string | null;
    community_enabled: boolean;
    copyright?: string;
    founding_subscription_benefits?: string[];
    paid_subscription_benefits?: string[];
    free_subscription_benefits?: string[];
    stripe_user_id?: string;
    stripe_country?: string;
    payments_state?: string;
    language?: string;
    email_from_name?: string;
    homepage_type?: string;
    theme_background_pop_color?: string;
    theme_web_bg_color?: string;
    theme_cover_bg_color?: string | null;
}
export interface SubstackSubscriptionsResponse {
    subscriptions: SubstackSubscription[];
    publicationUsers: SubstackPublicationUser[];
    publications: SubstackSubscriptionPublication[];
}
