use std::sync::Arc;

use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};

use crate::client::Client;
use crate::prosemirror;
use crate::types::*;

// ── MCP parameter types ──────────────────────────────────────────

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreatePostParams {
    /// Post title. If omitted, extracted from first heading or frontmatter.
    #[serde(default)]
    pub title: Option<String>,
    /// Post subtitle
    #[serde(default)]
    pub subtitle: Option<String>,
    /// Post body in markdown (mutually exclusive with file_path)
    #[serde(default)]
    pub body: Option<String>,
    /// Path to a markdown file (mutually exclusive with body)
    #[serde(default)]
    pub file_path: Option<String>,
    /// Path to a cover image file (optional, uploaded to Substack CDN)
    #[serde(default)]
    pub cover_image: Option<String>,
    /// Save as draft only (default: false)
    #[serde(default)]
    pub draft: Option<bool>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPublicationParams {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdatePublicationParams {
    /// Publication name
    #[serde(default)]
    pub name: Option<String>,
    /// Tagline displayed on homepage
    #[serde(default)]
    pub hero_text: Option<String>,
    /// ISO language code (e.g. "en", "es")
    #[serde(default)]
    pub language: Option<String>,
    /// Copyright line
    #[serde(default)]
    pub copyright: Option<String>,
    /// Logo URL (use upload_image first to get a URL)
    #[serde(default)]
    pub logo_url: Option<String>,
    /// Wide logo URL
    #[serde(default)]
    pub logo_url_wide: Option<String>,
    /// Cover photo URL
    #[serde(default)]
    pub cover_photo_url: Option<String>,
    /// Email banner URL
    #[serde(default)]
    pub email_banner_url: Option<String>,
    /// Accent color hex (e.g. "#FF6719")
    #[serde(default)]
    pub theme_var_background_pop: Option<String>,
    /// Enable community features
    #[serde(default)]
    pub community_enabled: Option<bool>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UploadImageParams {
    /// Path to the image file on disk
    pub file_path: String,
    /// Purpose: "logo", "cover", "banner", or "post"
    #[serde(default = "default_post_purpose")]
    pub purpose: String,
}

fn default_post_purpose() -> String {
    "post".into()
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListPostsParams {
    /// Number of posts to list (default: 10)
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPostParams {
    /// Post ID
    pub post_id: u64,
    /// Optional file path to save the post body as markdown
    #[serde(default)]
    pub save_to: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DeletePostParams {
    /// Post ID to delete
    pub post_id: u64,
}

// ── Server ───────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Server {
    client: Arc<Client>,
    tool_router: ToolRouter<Self>,
}

impl Server {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

    fn err(msg: impl std::fmt::Display) -> String {
        format!("{{\"error\": \"{msg}\"}}")
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Substack MCP server — create, publish, and manage posts and publication settings."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tool_router]
impl Server {
    // ── Publication ──────────────────────────────────────────

    #[tool(description = "Get current publication settings (name, tagline, language, logo, cover, etc.)")]
    async fn get_publication(
        &self,
        Parameters(_): Parameters<GetPublicationParams>,
    ) -> String {
        match self.client.publication().await {
            Ok(pub_) => serde_json::to_string_pretty(&pub_).unwrap_or_else(|e| Self::err(e)),
            Err(e) => Self::err(e),
        }
    }

    #[tool(description = "Update publication settings. Only provided fields are changed.")]
    async fn update_publication(
        &self,
        Parameters(params): Parameters<UpdatePublicationParams>,
    ) -> String {
        let update = PublicationUpdate {
            name: params.name,
            hero_text: params.hero_text,
            language: params.language,
            copyright: params.copyright,
            logo_url: params.logo_url,
            logo_url_wide: params.logo_url_wide,
            cover_photo_url: params.cover_photo_url,
            email_banner_url: params.email_banner_url,
            theme_var_background_pop: params.theme_var_background_pop,
            community_enabled: params.community_enabled,
            homepage_type: None,
        };
        match self.client.update_publication(&update).await {
            Ok(pub_) => serde_json::to_string_pretty(&pub_).unwrap_or_else(|e| Self::err(e)),
            Err(e) => Self::err(e),
        }
    }

    // ── Image ────────────────────────────────────────────────

    #[tool(description = "Upload an image file to Substack CDN. Returns the URL. Use the URL with update_publication to set logo/cover/banner.")]
    async fn upload_image(
        &self,
        Parameters(params): Parameters<UploadImageParams>,
    ) -> String {
        let purpose = ImagePurpose::from_str(&params.purpose);
        if purpose.is_none() {
            return Self::err("purpose must be: logo, cover, banner, or post");
        }

        match image_to_data_uri(&params.file_path) {
            Ok(data_uri) => {
                match self.client.upload_image(&data_uri, None).await {
                    Ok(upload) => {
                        serde_json::to_string_pretty(&upload).unwrap_or_else(|e| Self::err(e))
                    }
                    Err(e) => Self::err(e),
                }
            }
            Err(e) => Self::err(e),
        }
    }

    // ── Posts ─────────────────────────────────────────────────

    #[tool(description = "Create and publish a Substack post from markdown text or a file path. Strips YAML frontmatter if present.")]
    async fn create_post(
        &self,
        Parameters(params): Parameters<CreatePostParams>,
    ) -> String {
        match self.do_create_post(params).await {
            Ok(msg) => msg,
            Err(e) => Self::err(e),
        }
    }

    #[tool(description = "List published posts")]
    async fn list_posts(
        &self,
        Parameters(params): Parameters<ListPostsParams>,
    ) -> String {
        let limit = params.limit.unwrap_or(10);
        match self.client.list_posts(limit).await {
            Ok(posts) => {
                let summaries: Vec<PostSummary> = posts
                    .into_iter()
                    .map(|p| PostSummary {
                        id: p.id,
                        title: p.title,
                        slug: p.slug,
                        post_date: p.post_date,
                        audience: p.audience,
                        wordcount: p.wordcount,
                    })
                    .collect();
                serde_json::to_string_pretty(&summaries).unwrap_or_else(|e| Self::err(e))
            }
            Err(e) => Self::err(e),
        }
    }

    #[tool(description = "Get a post by ID. Optionally save body to a file instead of returning it.")]
    async fn get_post(
        &self,
        Parameters(params): Parameters<GetPostParams>,
    ) -> String {
        let post_id = PostId(params.post_id);
        match self.client.get_post(&post_id).await {
            Ok(post) => {
                if let Some(path) = params.save_to {
                    let body = post.body_html.as_deref().unwrap_or("");
                    match std::fs::write(&path, body) {
                        Ok(()) => format!("Saved to {path}"),
                        Err(e) => Self::err(e),
                    }
                } else {
                    serde_json::to_string_pretty(&post.meta).unwrap_or_else(|e| Self::err(e))
                }
            }
            Err(e) => Self::err(e),
        }
    }

    #[tool(description = "Delete a post or draft by ID")]
    async fn delete_post(
        &self,
        Parameters(params): Parameters<DeletePostParams>,
    ) -> String {
        let post_id = PostId(params.post_id);
        match self.client.delete_post(&post_id).await {
            Ok(()) => format!("Deleted post {}", params.post_id),
            Err(e) => Self::err(e),
        }
    }
}

// ── Post creation logic ──────────────────────────────────────────

impl Server {
    async fn do_create_post(
        &self,
        params: CreatePostParams,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let raw = match (&params.body, &params.file_path) {
            (Some(body), None) => body.clone(),
            (None, Some(path)) => std::fs::read_to_string(path)?,
            (Some(_), Some(_)) => return Err("provide body or file_path, not both".into()),
            (None, None) => return Err("body or file_path required".into()),
        };

        let (frontmatter, body) = prosemirror::strip_frontmatter(&raw);

        let title = params
            .title
            .or_else(|| prosemirror::frontmatter_field(&frontmatter, "title"))
            .or_else(|| prosemirror::extract_first_heading(&body))
            .unwrap_or_else(|| "Untitled".into());

        let subtitle = params
            .subtitle
            .or_else(|| prosemirror::frontmatter_field(&frontmatter, "subtitle"));

        let body = prosemirror::strip_leading_heading(&body, &title);
        let doc = prosemirror::from_markdown(&body);
        let draft = params.draft.unwrap_or(false);

        let cover_image_url = match &params.cover_image {
            Some(path) => {
                let data_uri = image_to_data_uri(path)?;
                let upload = self.client.upload_image(&data_uri, None).await
                    .map_err(|e| -> Box<dyn std::error::Error> { e.to_string().into() })?;
                Some(upload.url)
            }
            None => None,
        };

        let user_id = self.client.user_id().await?;
        let d = self.client.create_draft(user_id).await?;

        let update = DraftUpdate {
            draft_title: title.clone(),
            draft_subtitle: subtitle,
            draft_body: serde_json::to_string(&doc)?,
            cover_image: cover_image_url,
        };
        self.client.update_draft(&d.id, &update).await?;

        if !draft {
            let post = self.client.publish(&d.id).await?;
            let slug = post.slug.unwrap_or_default();
            let hostname = std::env::var("SUBSTACK_HOSTNAME").unwrap_or_default();
            Ok(format!("Published: {title}\nhttps://{hostname}/p/{slug}"))
        } else {
            Ok(format!("Draft saved: {title} (id: {})", d.id.0))
        }
    }
}

// ── Image helpers ────────────────────────────────────────────────

fn image_to_data_uri(path: &str) -> Result<String, crate::error::Error> {
    let data = std::fs::read(path)?;
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let mime = match ext {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => return Err(crate::error::Error::InvalidImage(format!("unsupported format: {ext}"))),
    };

    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
    Ok(format!("data:{mime};base64,{b64}"))
}
