use std::path::{Path, PathBuf};
use substack_cli::prosemirror::{ImageRef, ImageSource, LinkRef, LinkSource};

#[test]
fn parses_prefix_image_syntax_with_tail() {
    let input = "![a caption](./img.png) rest";
    let parsed = ImageRef::parse_prefix(input).unwrap();
    assert_eq!(parsed.image().alt(), "a caption");
    assert_eq!(parsed.image().src(), "./img.png");
    assert_eq!(&input[parsed.consumed()..], " rest");
}

#[test]
fn parse_prefix_rejects_multiline_alt() {
    assert!(ImageRef::parse_prefix("![line1\nline2](./img.png)").is_none());
}

#[test]
fn parse_prefix_rejects_nested_brackets_in_alt() {
    assert!(ImageRef::parse_prefix("![outer [inner]](./img.png)").is_none());
}

#[test]
fn parse_block_rejects_prefix_with_trailing_content() {
    assert!(ImageRef::parse_block("![alt](./img.png) tail").is_none());
}

#[test]
fn classifies_remote_and_local_sources() {
    assert!(matches!(
        ImageSource::classify("https://example.com/a.png", None),
        ImageSource::Remote(_)
    ));
    assert!(matches!(
        ImageSource::classify("http://example.com/a.png", None),
        ImageSource::Remote(_)
    ));
    assert!(matches!(
        ImageSource::classify("data:image/png;base64,AQID", None),
        ImageSource::Remote(_)
    ));
    assert!(matches!(
        ImageSource::classify("./local.png", None),
        ImageSource::Local(_)
    ));
    assert!(matches!(
        ImageSource::classify("/abs/path.png", None),
        ImageSource::Local(_)
    ));
}

#[test]
fn local_source_resolves_relative_path_against_base_dir() {
    let base = Path::new("/posts/bookofsol");
    match ImageSource::classify("./img/banner.png", Some(base)) {
        ImageSource::Local(path) => {
            assert_eq!(path, PathBuf::from("/posts/bookofsol/./img/banner.png"))
        }
        other => panic!("expected Local, got {other:?}"),
    }
    match ImageSource::classify("img/banner.png", Some(base)) {
        ImageSource::Local(path) => {
            assert_eq!(path, PathBuf::from("/posts/bookofsol/img/banner.png"))
        }
        other => panic!("expected Local, got {other:?}"),
    }
}

#[test]
fn absolute_path_is_unchanged_by_base_dir() {
    let base = Path::new("/posts/bookofsol");
    match ImageSource::classify("/tmp/absolute.png", Some(base)) {
        ImageSource::Local(path) => assert_eq!(path, PathBuf::from("/tmp/absolute.png")),
        other => panic!("expected Local, got {other:?}"),
    }
}

#[test]
fn no_base_dir_leaves_relative_path_unchanged() {
    match ImageSource::classify("./img.png", None) {
        ImageSource::Local(path) => assert_eq!(path, PathBuf::from("./img.png")),
        other => panic!("expected Local, got {other:?}"),
    }
}

#[test]
fn parses_prefix_link_syntax_with_tail() {
    let input = "[manual](./Plasma_Recycling_Manual.md) rest";
    let parsed = LinkRef::parse_prefix(input).unwrap();
    assert_eq!(parsed.link().label(), "manual");
    assert_eq!(parsed.link().href(), "./Plasma_Recycling_Manual.md");
    assert_eq!(&input[parsed.consumed()..], " rest");
}

#[test]
fn link_source_resolves_local_markdown_and_fragment() {
    let base = Path::new("/posts/bookofsol/water");
    match LinkSource::classify("../diet/Ambrosian_Diet.md#closing", Some(base)) {
        LinkSource::Local { path, fragment } => {
            assert_eq!(path, PathBuf::from("/posts/bookofsol/water/../diet/Ambrosian_Diet.md"));
            assert_eq!(fragment.as_deref(), Some("closing"));
        }
        other => panic!("expected Local, got {other:?}"),
    }
}

#[test]
fn anchor_only_link_is_treated_as_remote_passthrough() {
    match LinkSource::classify("#section", None) {
        LinkSource::Remote(raw) => assert_eq!(raw, "#section"),
        other => panic!("expected Remote, got {other:?}"),
    }
}
