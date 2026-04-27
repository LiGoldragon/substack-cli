use clap::Parser;
use substack_cli::CommandLine;
use substack_cli::cli::{PostCommand, PublicationCommand, RootCommand};

#[test]
fn parses_post_get_save_outputs() {
    let command_line = CommandLine::try_parse_from([
        "substack",
        "post",
        "get",
        "42",
        "--save-html",
        "post.html",
        "--save-json",
        "post.json",
    ])
    .unwrap();

    match command_line.command {
        RootCommand::Post(PostCommand::Get(arguments)) => {
            assert_eq!(arguments.post_id, 42);
            assert_eq!(arguments.save_html.as_deref(), Some("post.html"));
            assert_eq!(arguments.save_json.as_deref(), Some("post.json"));
        }
        _ => panic!("expected post get command"),
    }
}

#[test]
fn rejects_post_create_body_and_file_path_together() {
    let result = CommandLine::try_parse_from([
        "substack",
        "post",
        "create",
        "--body",
        "hello",
        "--file-path",
        "post.md",
    ]);

    assert!(result.is_err());
}

#[test]
fn parses_post_update() {
    let command_line = CommandLine::try_parse_from([
        "substack",
        "post",
        "update",
        "42",
        "--file-path",
        "post.md",
        "--publish-linked-files",
        "--link-manifest",
        "links.json",
    ])
    .unwrap();

    match command_line.command {
        RootCommand::Post(PostCommand::Update(arguments)) => {
            assert_eq!(arguments.post_id, 42);
            assert_eq!(arguments.file_path.as_deref(), Some("post.md"));
            assert!(arguments.publish_linked_files);
            assert_eq!(arguments.link_manifest.as_deref(), Some("links.json"));
        }
        _ => panic!("expected post update command"),
    }
}

#[test]
fn parses_publication_image_command_without_purpose() {
    let command_line = CommandLine::try_parse_from([
        "substack",
        "publication",
        "set-logo",
        "--file",
        "logo.png",
    ])
    .unwrap();

    match command_line.command {
        RootCommand::Publication(PublicationCommand::SetLogo(arguments)) => {
            assert_eq!(arguments.file_path, "logo.png");
        }
        _ => panic!("expected publication set-logo command"),
    }
}
