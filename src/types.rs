use serde::{Deserialize, Serialize};

// ── Identity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostId(u64);

impl PostId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for PostId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(u64);

impl UserId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

// ── URL ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl(String);

impl ImageUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ImageUrl {
    fn from(url: String) -> Self {
        Self(url)
    }
}

impl AsRef<str> for ImageUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// ── Host & credentials ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Hostname(String);

impl Hostname {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Hostname {
    fn from(hostname: String) -> Self {
        Self(hostname)
    }
}

impl AsRef<str> for Hostname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ApiKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiKey(<redacted>)")
    }
}

// ── Publication ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    pub name: Option<String>,
    pub subdomain: Option<String>,
    pub hero_text: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<ImageUrl>,
    pub logo_url_wide: Option<ImageUrl>,
    pub cover_photo_url: Option<ImageUrl>,
    pub email_banner_url: Option<ImageUrl>,
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
    pub logo_url: Option<ImageUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url_wide: Option<ImageUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_photo_url: Option<ImageUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_banner_url: Option<ImageUrl>,
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
    pub cover_image: Option<ImageUrl>,
    pub wordcount: Option<u64>,
    pub reaction_count: Option<u64>,
    pub comment_count: Option<u64>,
    #[serde(default)]
    pub body_json: Option<serde_json::Value>,
    #[serde(default)]
    pub body_html: Option<String>,
}

impl Post {
    pub fn summary(&self) -> PostSummary {
        PostSummary {
            id: self.id.clone(),
            title: self.title.clone(),
            slug: self.slug.clone(),
            post_date: self.post_date.clone(),
            audience: self.audience.clone(),
            wordcount: self.wordcount,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
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
    pub id: UserId,
    pub user_id: UserId,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftUpdate {
    pub draft_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_subtitle: Option<String>,
    pub draft_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<ImageUrl>,
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
    pub url: ImageUrl,
}

// ── Internal API shapes ──────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PubUser {
    pub user_id: UserId,
}

#[derive(Debug, Deserialize)]
pub struct PubUsersResponse {
    pub pub_users: Vec<PubUser>,
}
