use std::sync::Arc;

use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};

use crate::client;

// ── Parameter types ──────────────────────────────────────────────

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
    /// Save as draft only (default: false)
    #[serde(default)]
    pub draft: Option<bool>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetProfileParams {}

// ── Server ───────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Server {
    client: Arc<client::Client>,
    tool_router: ToolRouter<Self>,
}

impl Server {
    pub fn new(client: Arc<client::Client>) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Substack MCP server — create, draft, and publish posts."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tool_router]
impl Server {
    #[tool(description = "Create and publish a Substack post from markdown text or a file path. Strips YAML frontmatter if present.")]
    async fn create_post(
        &self,
        Parameters(params): Parameters<CreatePostParams>,
    ) -> String {
        match self.do_create_post(params).await {
            Ok(msg) => msg,
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }

    #[tool(description = "Get own Substack profile")]
    async fn get_profile(
        &self,
        Parameters(_params): Parameters<GetProfileParams>,
    ) -> String {
        match self.client.profile().await {
            Ok(p) => serde_json::to_string_pretty(&serde_json::json!({
                "name": p.name,
                "handle": p.handle,
                "bio": p.bio,
            }))
            .unwrap_or_default(),
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }
}

impl Server {
    async fn do_create_post(
        &self,
        params: CreatePostParams,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let raw = match (&params.body, &params.file_path) {
            (Some(body), None) => body.clone(),
            (None, Some(path)) => std::fs::read_to_string(path)?,
            (Some(_), Some(_)) => {
                return Err("provide body or file_path, not both".into())
            }
            (None, None) => return Err("body or file_path required".into()),
        };

        let (frontmatter, body) = strip_frontmatter(&raw);

        let title = params
            .title
            .or_else(|| frontmatter_field(&frontmatter, "title"))
            .or_else(|| extract_first_heading(&body))
            .unwrap_or_else(|| "Untitled".into());

        let subtitle = params
            .subtitle
            .or_else(|| frontmatter_field(&frontmatter, "subtitle"));

        // Strip leading heading if it matches the title
        let body = strip_leading_heading(&body, &title);

        let draft = params.draft.unwrap_or(false);

        let user_id = self.client.user_id().await?;
        let d = self.client.create_draft(user_id).await?;

        let prosemirror = markdown_to_prosemirror(&body);
        let update = client::DraftUpdate {
            draft_title: title.clone(),
            draft_subtitle: subtitle,
            draft_body: serde_json::to_string(&prosemirror)?,
        };
        self.client.update_draft(d.id, &update).await?;

        if !draft {
            let post = self.client.publish(d.id).await?;
            let slug = post.slug.unwrap_or_default();
            let hostname =
                std::env::var("SUBSTACK_HOSTNAME").unwrap_or_default();
            Ok(format!(
                "Published: {title}\nhttps://{hostname}/p/{slug}"
            ))
        } else {
            Ok(format!("Draft saved: {title} (id: {})", d.id))
        }
    }
}

// ── Markdown helpers ─────────────────────────────────────────────

fn strip_frontmatter(text: &str) -> (Option<String>, String) {
    if text.starts_with("---\n") || text.starts_with("---\r\n") {
        if let Some(end) = text[3..].find("\n---") {
            let fm = text[4..3 + end].to_string();
            let body = text[3 + end + 4..].trim_start().to_string();
            return (Some(fm), body);
        }
    }
    (None, text.to_string())
}

fn frontmatter_field(fm: &Option<String>, key: &str) -> Option<String> {
    let fm = fm.as_ref()?;
    for line in fm.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

fn extract_first_heading(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(h) = trimmed.strip_prefix("# ") {
            return Some(h.trim().to_string());
        }
    }
    None
}

fn strip_leading_heading(text: &str, title: &str) -> String {
    let mut lines = text.lines().peekable();
    if let Some(first) = lines.peek() {
        let trimmed = first.trim();
        if let Some(h) = trimmed.strip_prefix("# ") {
            if h.trim() == title {
                lines.next();
                // skip blank line after heading
                if let Some(next) = lines.peek() {
                    if next.trim().is_empty() {
                        lines.next();
                    }
                }
                return lines.collect::<Vec<_>>().join("\n");
            }
        }
    }
    text.to_string()
}

/// Minimal markdown to ProseMirror JSON.
fn markdown_to_prosemirror(md: &str) -> serde_json::Value {
    let mut content = Vec::new();

    for block in md.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if let Some(h) = block.strip_prefix("## ") {
            content.push(serde_json::json!({
                "type": "heading",
                "attrs": { "level": 2 },
                "content": inline_nodes(h.trim())
            }));
        } else if let Some(h) = block.strip_prefix("### ") {
            content.push(serde_json::json!({
                "type": "heading",
                "attrs": { "level": 3 },
                "content": inline_nodes(h.trim())
            }));
        } else if block.starts_with("> ") {
            let lines: Vec<&str> = block
                .lines()
                .map(|l| l.strip_prefix("> ").unwrap_or(l))
                .collect();
            let mut para_content = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                let line = line.strip_suffix('\\').unwrap_or(line).trim();
                para_content.extend(inline_nodes(line));
                if i < lines.len() - 1 {
                    para_content.push(serde_json::json!({ "type": "hardBreak" }));
                }
            }
            content.push(serde_json::json!({
                "type": "blockquote",
                "content": [{
                    "type": "paragraph",
                    "content": para_content
                }]
            }));
        } else {
            content.push(serde_json::json!({
                "type": "paragraph",
                "content": inline_nodes(&block.replace('\n', " "))
            }));
        }
    }

    serde_json::json!({ "type": "doc", "content": content })
}

/// Parse inline markdown: **bold**, *italic*.
fn inline_nodes(text: &str) -> Vec<serde_json::Value> {
    let mut nodes = Vec::new();
    let mut current = String::new();
    let mut i = 0;

    while i < text.len() {
        if text.as_bytes()[i] == b'*' {
            let start = i;
            while i < text.len() && text.as_bytes()[i] == b'*' {
                i += 1;
            }
            let stars = i - start;
            let pattern: String = std::iter::repeat('*').take(stars).collect();

            if let Some(end) = text[i..].find(&pattern) {
                if !current.is_empty() {
                    nodes.push(serde_json::json!({
                        "type": "text", "text": current
                    }));
                    current.clear();
                }

                let inner = &text[i..i + end];
                let mut marks = Vec::new();
                if stars >= 2 {
                    marks.push(serde_json::json!({ "type": "bold" }));
                }
                if stars == 1 || stars == 3 {
                    marks.push(serde_json::json!({ "type": "italic" }));
                }

                let mut node =
                    serde_json::json!({ "type": "text", "text": inner });
                if !marks.is_empty() {
                    node["marks"] = serde_json::Value::Array(marks);
                }
                nodes.push(node);
                i += end + stars;
            } else {
                current.push_str(&pattern);
            }
        } else {
            let ch = text[i..].chars().next().unwrap();
            current.push(ch);
            i += ch.len_utf8();
        }
    }

    if !current.is_empty() {
        nodes.push(serde_json::json!({
            "type": "text", "text": current
        }));
    }

    if nodes.is_empty() {
        nodes.push(serde_json::json!({
            "type": "text", "text": text
        }));
    }

    nodes
}
