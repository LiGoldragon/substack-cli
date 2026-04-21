use serde_json::Value;

/// Convert markdown to Substack's ProseMirror JSON document format.
pub fn from_markdown(md: &str) -> Value {
    let mut content = Vec::new();

    for block in md.split("\n\n") {
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
        } else {
            content.push(paragraph(block));
        }
    }

    serde_json::json!({ "type": "doc", "content": content })
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

fn strip_blockquote_marker(line: &str) -> &str {
    let mut rest = line.trim_start();
    while let Some(next) = rest.strip_prefix('>') {
        rest = next.trim_start();
    }
    rest
}

/// Parse inline markdown: **bold**, *italic*.
fn inline_nodes(text: &str) -> Vec<Value> {
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
                flush_text(&mut current, &mut nodes);

                let inner = &text[i..i + end];
                let mut marks = Vec::new();
                if stars >= 2 {
                    marks.push(serde_json::json!({ "type": "bold" }));
                }
                if stars == 1 || stars == 3 {
                    marks.push(serde_json::json!({ "type": "italic" }));
                }

                let mut node = serde_json::json!({ "type": "text", "text": inner });
                if !marks.is_empty() {
                    node["marks"] = Value::Array(marks);
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

    flush_text(&mut current, &mut nodes);

    if nodes.is_empty() {
        nodes.push(serde_json::json!({ "type": "text", "text": text }));
    }

    nodes
}

fn flush_text(buf: &mut String, nodes: &mut Vec<Value>) {
    if !buf.is_empty() {
        nodes.push(serde_json::json!({ "type": "text", "text": *buf }));
        buf.clear();
    }
}

// ── Markdown helpers ─────────────────────────────────────────────

/// Strip YAML frontmatter from markdown, returning (frontmatter, body).
pub fn strip_frontmatter(text: &str) -> (Option<String>, String) {
    if text.starts_with("---\n") || text.starts_with("---\r\n") {
        if let Some(end) = text[3..].find("\n---") {
            let fm = text[4..3 + end].to_string();
            let body = text[3 + end + 4..].trim_start().to_string();
            return (Some(fm), body);
        }
    }
    (None, text.to_string())
}

/// Extract a field value from YAML-like frontmatter.
pub fn frontmatter_field(fm: &Option<String>, key: &str) -> Option<String> {
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

/// Extract the first `# heading` from markdown.
pub fn extract_first_heading(text: &str) -> Option<String> {
    for line in text.lines() {
        if let Some(h) = line.trim().strip_prefix("# ") {
            return Some(h.trim().to_string());
        }
    }
    None
}

/// Strip the leading `# heading` if it matches the title.
pub fn strip_leading_heading(text: &str, title: &str) -> String {
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
                return lines.collect::<Vec<_>>().join("\n");
            }
        }
    }
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockquote_spacer_lines_do_not_render_literal_markers() {
        let doc = from_markdown("> first\n> >\n> second");
        let content = doc["content"].as_array().unwrap();
        let quote = &content[0]["content"][0]["content"];

        assert_eq!(
            quote,
            &serde_json::json!([
                { "type": "text", "text": "first" },
                { "type": "hardBreak" },
                { "type": "hardBreak" },
                { "type": "text", "text": "second" }
            ])
        );
    }
}
