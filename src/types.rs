use serde::{Deserialize, Serialize};

// ── Identity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostId(pub u64);

// ── Publication ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    pub name: Option<String>,
    pub subdomain: Option<String>,
    pub hero_text: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<String>,
    pub logo_url_wide: Option<String>,
    pub cover_photo_url: Option<String>,
    pub email_banner_url: Option<String>,
    pub copyright: Option<String>,
    pub community_enabled: Option<bool>,
    pub homepage_type: Option<String>,
    pub theme_var_background_pop: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicationUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url_wide: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_banner_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_var_background_pop: Option<String>,
}

// ── Post ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub slug: Option<String>,
    pub post_date: Option<String>,
    #[serde(rename = "type")]
    pub post_type: Option<String>,
    pub audience: Option<String>,
    pub cover_image: Option<String>,
    pub wordcount: Option<u64>,
    pub reaction_count: Option<u64>,
    pub comment_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFull {
    #[serde(flatten)]
    pub meta: Post,
    pub body_json: Option<serde_json::Value>,
    pub body_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: PostId,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub post_date: Option<String>,
    pub audience: Option<String>,
    pub wordcount: Option<u64>,
}

// ── Draft ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Draft {
    pub id: PostId,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftCreate {
    pub draft_bylines: Vec<DraftByline>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftByline {
    pub id: u64,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftUpdate {
    pub draft_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_subtitle: Option<String>,
    pub draft_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
}

// ── Published ────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Published {
    #[serde(default)]
    pub slug: Option<String>,
}

// ── Image ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUpload {
    pub url: String,
}

// ── Internal API shapes ──────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PubUser {
    pub user_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct PubUsersResponse {
    pub pub_users: Vec<PubUser>,
}
