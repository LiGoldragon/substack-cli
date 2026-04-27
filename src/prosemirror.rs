use serde::Serialize;
use serde_json::Value;

/// Raw Markdown text. All parsing / conversion entry points hang off this type.
#[derive(Debug, Clone)]
pub struct Markdown(String);

impl From<String> for Markdown {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for Markdown {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl Markdown {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Split leading YAML frontmatter (if any) from the body.
    pub fn split_frontmatter(self) -> FrontmatterSplit {
        let text = &self.0;
        if text.starts_with("---\n") || text.starts_with("---\r\n") {
            if let Some(end) = text[3..].find("\n---") {
                let fm = Frontmatter(text[4..3 + end].to_string());
                let body = Markdown(text[3 + end + 4..].trim_start().to_string());
                return FrontmatterSplit {
                    frontmatter: Some(fm),
                    body,
                };
            }
        }
        FrontmatterSplit {
            frontmatter: None,
            body: self,
        }
    }

    /// First `# heading` in the body, if present.
    pub fn first_heading(&self) -> Option<String> {
        for line in self.0.lines() {
            if let Some(h) = line.trim().strip_prefix("# ") {
                return Some(h.trim().to_string());
            }
        }
        None
    }

    /// Drop the leading `# heading` line if it matches `title`.
    pub fn without_leading_heading(self, title: &str) -> Self {
        let text = &self.0;
        let mut lines = text.lines().peekable();
        if let Some(first) = lines.peek() {
            if let Some(h) = first.trim().strip_prefix("# ") {
                if h.trim() == title {
                    lines.next();
                    if let Some(next) = lines.peek() {
                        if next.trim().is_empty() {
                            lines.next();
                        }
                    }
                    return Markdown(lines.collect::<Vec<_>>().join("\n"));
                }
            }
        }
        self
    }

    /// Convert to Substack's ProseMirror JSON document format.
    pub fn to_prosemirror(&self) -> ProseMirrorDoc {
        let mut content = Vec::new();

        for block in self.0.split("\n\n") {
            let block = block.trim();
            if block.is_empty() {
                continue;
            }

            if let Some(h) = block.strip_prefix("## ") {
                content.push(heading(2, h.trim()));
            } else if let Some(h) = block.strip_prefix("### ") {
                content.push(heading(3, h.trim()));
            } else if block.starts_with("> ") {
                content.push(blockquote(block));
            } else if let Some(image) = ImageRef::parse_block(block) {
                content.push(image.into_captioned_node());
            } else if let Some(table) = Table::parse_block(block) {
                content.extend(table.into_row_paragraphs());
            } else {
                content.push(paragraph(block));
            }
        }

        ProseMirrorDoc(serde_json::json!({ "type": "doc", "content": content }))
    }
}

/// Result of `Markdown::split_frontmatter`. Named struct, not an anonymous tuple.
pub struct FrontmatterSplit {
    pub frontmatter: Option<Frontmatter>,
    pub body: Markdown,
}

/// YAML-ish frontmatter block.
#[derive(Debug, Clone)]
pub struct Frontmatter(String);

impl Frontmatter {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Look up a scalar field value (`key: value` line).
    pub fn field(&self, key: &str) -> Option<String> {
        for line in self.0.lines() {
            if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
                let val = rest.trim().trim_matches('"').trim_matches('\'');
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
        None
    }
}

/// A rendered ProseMirror document. Serializes transparently to the inner JSON.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ProseMirrorDoc(Value);

impl ProseMirrorDoc {
    pub fn as_value(&self) -> &Value {
        &self.0
    }

    pub fn into_value(self) -> Value {
        self.0
    }
}

fn heading(level: u8, text: &str) -> Value {
    serde_json::json!({
        "type": "heading",
        "attrs": { "level": level },
        "content": inline_nodes(text)
    })
}

fn paragraph(block: &str) -> Value {
    let lines: Vec<&str> = block.lines().collect();
    let has_breaks = lines.iter().any(|l| l.ends_with('\\'));

    if has_breaks {
        let mut para_content = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let line = line.strip_suffix('\\').unwrap_or(line).trim();
            para_content.extend(inline_nodes(line));
            if i < lines.len() - 1 {
                para_content.push(serde_json::json!({ "type": "hardBreak" }));
            }
        }
        serde_json::json!({
            "type": "paragraph",
            "content": para_content
        })
    } else {
        let text = block.replace('\n', " ");
        serde_json::json!({
            "type": "paragraph",
            "content": inline_nodes(&text)
        })
    }
}

fn blockquote(block: &str) -> Value {
    let lines: Vec<&str> = block.lines().map(strip_blockquote_marker).collect();

    let mut para_content = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let line = line.strip_suffix('\\').unwrap_or(line).trim();
        if !line.is_empty() {
            para_content.extend(inline_nodes(line));
        }
        if i < lines.len() - 1 {
            para_content.push(serde_json::json!({ "type": "hardBreak" }));
        }
    }

    serde_json::json!({
        "type": "blockquote",
        "content": [{
            "type": "paragraph",
            "content": para_content
        }]
    })
}

/// A GFM-style Markdown table: a header row, a separator, and zero or more body rows.
pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Parse a block that is a GFM pipe table. Requires at least 2 lines:
    /// a header row (`| a | b |`) followed by a separator (`|---|---|`).
    pub fn parse_block(block: &str) -> Option<Self> {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 2 {
            return None;
        }
        if !lines.iter().all(|line| line.trim_start().starts_with('|')) {
            return None;
        }
        if !is_separator_row(lines[1]) {
            return None;
        }

        let header = parse_table_row(lines[0]);
        let column_count = header.len();
        if column_count == 0 {
            return None;
        }

        let mut rows = Vec::new();
        for line in &lines[2..] {
            let cells = parse_table_row(line);
            if cells.is_empty() {
                continue;
            }
            rows.push(pad_or_truncate(cells, column_count));
        }

        Some(Self { header, rows })
    }

    pub fn header(&self) -> &[String] {
        &self.header
    }

    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Substack's ProseMirror schema rejects `table` nodes (the web editor
    /// has no table button; probes with snake_case and camelCase `table_row`
    /// / `tableRow` both cause the server-side HTML renderer to emit an
    /// empty `<p></p>`). As a fallback, emit each row as its own paragraph
    /// with cells joined by ` | ` so the content at least stays readable
    /// instead of collapsing into one wrapped mush.
    fn into_row_paragraphs(self) -> Vec<Value> {
        let mut out = Vec::with_capacity(1 + self.rows.len());
        out.push(paragraph_from_cells(&self.header, true));
        for row in self.rows {
            out.push(paragraph_from_cells(&row, false));
        }
        out
    }
}

fn paragraph_from_cells(cells: &[String], bold: bool) -> Value {
    let mut content: Vec<Value> = Vec::new();
    for (index, cell) in cells.iter().enumerate() {
        if index > 0 {
            content.push(serde_json::json!({ "type": "text", "text": " | " }));
        }
        for mut node in inline_nodes(cell) {
            if bold && node["type"] == "text" {
                let existing = node
                    .get_mut("marks")
                    .and_then(Value::as_array_mut)
                    .cloned()
                    .unwrap_or_default();
                let mut marks = existing;
                if !marks
                    .iter()
                    .any(|m| m.get("type").and_then(Value::as_str) == Some("bold"))
                {
                    marks.push(serde_json::json!({ "type": "bold" }));
                }
                node["marks"] = Value::Array(marks);
            }
            content.push(node);
        }
    }
    serde_json::json!({
        "type": "paragraph",
        "content": content,
    })
}

fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return false;
    }
    let inner = trimmed.trim_start_matches('|').trim_end_matches('|');
    if inner.is_empty() {
        return false;
    }
    inner.split('|').all(|cell| {
        let cell = cell.trim();
        !cell.is_empty()
            && cell
                .chars()
                .all(|c| matches!(c, '-' | ':' | ' '))
            && cell.contains('-')
    })
}

fn parse_table_row(line: &str) -> Vec<String> {
    let line = line.trim();
    let inner = line.trim_start_matches('|').trim_end_matches('|');
    inner.split('|').map(|cell| cell.trim().to_string()).collect()
}

fn pad_or_truncate(mut cells: Vec<String>, width: usize) -> Vec<String> {
    if cells.len() > width {
        cells.truncate(width);
    } else {
        while cells.len() < width {
            cells.push(String::new());
        }
    }
    cells
}

/// A Markdown link reference: `[label](href)`.
#[derive(Debug, Clone)]
pub struct LinkRef {
    label: String,
    href: String,
}

/// Result of `LinkRef::parse_prefix` — the parsed link plus the number of
/// bytes consumed from the input (i.e. the offset past the closing `)`).
pub struct ParsedLinkRef {
    link: LinkRef,
    consumed: usize,
}

impl ParsedLinkRef {
    pub fn link(&self) -> &LinkRef {
        &self.link
    }

    pub fn consumed(&self) -> usize {
        self.consumed
    }

    pub fn into_link(self) -> LinkRef {
        self.link
    }
}

impl LinkRef {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn href(&self) -> &str {
        &self.href
    }

    pub fn source(&self, base_dir: Option<&std::path::Path>) -> LinkSource {
        LinkSource::classify(&self.href, base_dir)
    }

    /// Parse `[label](href)` starting at the beginning of `input`.
    /// `input` must begin with `[`; returns the link and consumed byte count.
    pub fn parse_prefix(input: &str) -> Option<ParsedLinkRef> {
        let after_open = input.strip_prefix('[')?;
        let label_end = after_open.find("](")?;
        let label = &after_open[..label_end];
        if label.contains('\n') || label.contains('[') {
            return None;
        }
        let after_label = &after_open[label_end + 2..];
        let href_end = after_label.find(')')?;
        let href = &after_label[..href_end];
        if href.contains('\n') {
            return None;
        }
        let consumed = "[".len() + label_end + "](".len() + href_end + ")".len();
        Some(ParsedLinkRef {
            link: Self {
                label: label.to_string(),
                href: href.to_string(),
            },
            consumed,
        })
    }
}

#[derive(Debug, Clone)]
pub enum LinkSource {
    Remote(String),
    Local {
        path: std::path::PathBuf,
        fragment: Option<String>,
    },
}

impl LinkSource {
    pub fn classify(raw: &str, base_dir: Option<&std::path::Path>) -> Self {
        if raw.starts_with("http://")
            || raw.starts_with("https://")
            || raw.starts_with("mailto:")
            || raw.starts_with("tel:")
            || raw.starts_with("data:")
            || raw.starts_with('#')
        {
            return Self::Remote(raw.to_string());
        }

        let (path_part, fragment) = match raw.split_once('#') {
            Some((path, fragment)) => (path, Some(fragment.to_string())),
            None => (raw, None),
        };
        let path = std::path::Path::new(path_part);
        let resolved = if path.is_absolute() {
            path.to_path_buf()
        } else {
            match base_dir {
                Some(base) if !base.as_os_str().is_empty() => base.join(path),
                _ => path.to_path_buf(),
            }
        };
        Self::Local {
            path: resolved,
            fragment,
        }
    }
}

/// A Markdown image reference: `![alt](src)`.
#[derive(Debug, Clone)]
pub struct ImageRef {
    alt: String,
    src: String,
}

/// Result of `ImageRef::parse_prefix` — the parsed image plus the number of
/// bytes consumed from the input (i.e. the offset past the closing `)`).
pub struct ParsedImageRef {
    image: ImageRef,
    consumed: usize,
}

impl ParsedImageRef {
    pub fn image(&self) -> &ImageRef {
        &self.image
    }

    pub fn consumed(&self) -> usize {
        self.consumed
    }

    pub fn into_image(self) -> ImageRef {
        self.image
    }
}

impl ImageRef {
    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    /// Classify this image's `src` as remote or local, resolving relative
    /// local paths against `base_dir` when provided.
    pub fn source(&self, base_dir: Option<&std::path::Path>) -> ImageSource {
        ImageSource::classify(&self.src, base_dir)
    }

    /// Parse a standalone image block. Returns `None` unless the trimmed block
    /// is exactly `![alt](src)` with nothing else around it.
    pub fn parse_block(block: &str) -> Option<Self> {
        let trimmed = block.trim();
        let parsed = Self::parse_prefix(trimmed)?;
        if parsed.consumed == trimmed.len() {
            Some(parsed.image)
        } else {
            None
        }
    }

    /// Parse `![alt](src)` starting at the beginning of `input`.
    /// `input` must begin with `![`; returns the image and consumed byte count.
    pub fn parse_prefix(input: &str) -> Option<ParsedImageRef> {
        let after_bang = input.strip_prefix("![")?;
        let alt_end = after_bang.find("](")?;
        let alt = &after_bang[..alt_end];
        if alt.contains('\n') || alt.contains('[') {
            return None;
        }
        let after_alt = &after_bang[alt_end + 2..];
        let src_end = after_alt.find(')')?;
        let src = &after_alt[..src_end];
        if src.contains('\n') {
            return None;
        }
        let consumed = "![".len() + alt_end + "](".len() + src_end + ")".len();
        Some(ParsedImageRef {
            image: Self {
                alt: alt.to_string(),
                src: src.to_string(),
            },
            consumed,
        })
    }

    fn into_captioned_node(self) -> Value {
        serde_json::json!({
            "type": "captionedImage",
            "content": [{
                "type": "image2",
                "attrs": {
                    "src": self.src,
                    "alt": self.alt,
                    "fullscreen": null,
                    "imageSize": "normal",
                    "height": null,
                    "width": null,
                    "resizeWidth": null,
                    "bytes": null,
                    "title": null,
                    "type": null,
                    "href": null,
                }
            }]
        })
    }
}

/// Classification of an image reference's `src` string.
///
/// `http(s)://` and `data:` URLs are passed through as-is; anything else is
/// treated as a local path, resolved against an optional base directory.
#[derive(Debug, Clone)]
pub enum ImageSource {
    Remote(String),
    Local(std::path::PathBuf),
}

impl ImageSource {
    /// Classify a `src` string from an `ImageRef`, resolving local paths
    /// against `base_dir` when provided.
    pub fn classify(raw: &str, base_dir: Option<&std::path::Path>) -> Self {
        if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with("data:") {
            return Self::Remote(raw.to_string());
        }
        let path = std::path::Path::new(raw);
        let resolved = if path.is_absolute() {
            path.to_path_buf()
        } else {
            match base_dir {
                Some(base) if !base.as_os_str().is_empty() => base.join(path),
                _ => path.to_path_buf(),
            }
        };
        Self::Local(resolved)
    }
}

fn strip_blockquote_marker(line: &str) -> &str {
    let mut rest = line.trim_start();
    while let Some(next) = rest.strip_prefix('>') {
        rest = next.trim_start();
    }
    rest
}

struct InlineMarkdown<'a> {
    remaining: &'a str,
    current: String,
    nodes: Vec<Value>,
}

impl<'a> InlineMarkdown<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            remaining: text,
            current: String::new(),
            nodes: Vec::new(),
        }
    }

    fn into_nodes(mut self) -> Vec<Value> {
        while !self.remaining.is_empty() {
            if self.try_link() || self.try_emphasis() {
                continue;
            }
            self.push_next_char();
        }

        self.flush_text();

        if self.nodes.is_empty() {
            self.nodes.push(text_node(self.remaining, Vec::new()));
        }

        self.nodes
    }

    fn try_link(&mut self) -> bool {
        let Some(parsed) = LinkRef::parse_prefix(self.remaining) else {
            return false;
        };
        let consumed = parsed.consumed();
        let link = parsed.into_link();

        self.flush_text();
        self.nodes.push(text_node(
            link.label(),
            vec![serde_json::json!({
                "type": "link",
                "attrs": {
                    "href": link.href(),
                    "title": null,
                }
            })],
        ));
        self.remaining = &self.remaining[consumed..];
        true
    }

    fn try_emphasis(&mut self) -> bool {
        if !self.remaining.starts_with('*') {
            return false;
        }

        let mut i = 0;
        while i < self.remaining.len() && self.remaining.as_bytes()[i] == b'*' {
            i += 1;
        }
        let stars = i;
        let pattern: String = std::iter::repeat('*').take(stars).collect();

        let Some(end) = self.remaining[i..].find(&pattern) else {
            return false;
        };

        self.flush_text();

        let inner = &self.remaining[i..i + end];
        let mut marks = Vec::new();
        if stars >= 2 {
            marks.push(serde_json::json!({ "type": "bold" }));
        }
        if stars == 1 || stars == 3 {
            marks.push(serde_json::json!({ "type": "italic" }));
        }

        self.nodes.push(text_node(inner, marks));
        self.remaining = &self.remaining[i + end + stars..];
        true
    }

    fn push_next_char(&mut self) {
        let ch = self.remaining.chars().next().unwrap();
        self.current.push(ch);
        self.remaining = &self.remaining[ch.len_utf8()..];
    }

    fn flush_text(&mut self) {
        if self.current.is_empty() {
            return;
        }
        self.nodes.push(text_node(&self.current, Vec::new()));
        self.current.clear();
    }
}

fn text_node(text: &str, marks: Vec<Value>) -> Value {
    let mut node = serde_json::json!({ "type": "text", "text": text });
    if !marks.is_empty() {
        node["marks"] = Value::Array(marks);
    }
    node
}

/// Parse inline markdown: **bold**, *italic*, [links](https://example.com).
fn inline_nodes(text: &str) -> Vec<Value> {
    InlineMarkdown::new(text).into_nodes()
}
