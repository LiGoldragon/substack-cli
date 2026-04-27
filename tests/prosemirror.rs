use substack_cli::prosemirror::Markdown;

fn render(md: &str) -> serde_json::Value {
    Markdown::from(md).to_prosemirror().into_value()
}

#[test]
fn pipe_table_emits_one_paragraph_per_row_with_bold_header() {
    let doc = render("| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |");
    let content = doc["content"].as_array().unwrap();
    assert_eq!(content.len(), 3);

    for p in content {
        assert_eq!(p["type"], "paragraph");
    }

    let header_first_text = &content[0]["content"][0];
    assert_eq!(header_first_text["text"], "A");
    let marks = header_first_text["marks"].as_array().unwrap();
    assert!(marks.iter().any(|m| m["type"] == "bold"));

    let body_first_text = &content[1]["content"][0];
    assert_eq!(body_first_text["text"], "1");
    assert!(body_first_text.get("marks").is_none());

    let sep = &content[1]["content"][1];
    assert_eq!(sep["text"], " | ");
}

#[test]
fn non_table_pipe_line_stays_paragraph() {
    let doc = render("This | is | not | a | table");
    assert_eq!(doc["content"][0]["type"], "paragraph");
}

#[test]
fn standalone_image_block_becomes_captioned_image() {
    let doc = render("![A thali](https://cdn.example.com/thali.png)");
    let content = doc["content"].as_array().unwrap();

    assert_eq!(content.len(), 1);
    assert_eq!(content[0]["type"], "captionedImage");
    let inner = &content[0]["content"][0];
    assert_eq!(inner["type"], "image2");
    assert_eq!(inner["attrs"]["src"], "https://cdn.example.com/thali.png");
    assert_eq!(inner["attrs"]["alt"], "A thali");
}

#[test]
fn image_with_trailing_text_is_not_treated_as_image_block() {
    let doc = render("![alt](./img.png) but with caption");
    let content = doc["content"].as_array().unwrap();
    assert_eq!(content[0]["type"], "paragraph");
}

#[test]
fn inline_markdown_link_becomes_link_mark() {
    let doc = render("See [Plasma Recycling Manual](./Plasma_Recycling_Manual.md).");
    let content = doc["content"].as_array().unwrap();
    let paragraph = &content[0]["content"];

    assert_eq!(paragraph[0]["text"], "See ");
    assert_eq!(paragraph[1]["text"], "Plasma Recycling Manual");
    assert_eq!(paragraph[1]["marks"][0]["type"], "link");
    assert_eq!(
        paragraph[1]["marks"][0]["attrs"]["href"],
        "./Plasma_Recycling_Manual.md"
    );
    assert_eq!(paragraph[2]["text"], ".");
}

#[test]
fn inline_markdown_link_renders_inside_blockquote() {
    let doc = render("> Consult [the manual](https://example.com/manual).");
    let content = doc["content"].as_array().unwrap();
    let quote = &content[0]["content"][0]["content"];

    assert_eq!(quote[0]["text"], "Consult ");
    assert_eq!(quote[1]["text"], "the manual");
    assert_eq!(quote[1]["marks"][0]["type"], "link");
    assert_eq!(
        quote[1]["marks"][0]["attrs"]["href"],
        "https://example.com/manual"
    );
    assert_eq!(quote[2]["text"], ".");
}

#[test]
fn blockquote_spacer_lines_do_not_render_literal_markers() {
    let doc = render("> first\n> >\n> second");
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
