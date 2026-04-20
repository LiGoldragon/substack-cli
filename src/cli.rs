use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "substack")]
#[command(about = "Create, publish, and manage Substack posts")]
pub struct CommandLine {
    #[command(subcommand)]
    pub command: RootCommand,
}

impl CommandLine {
    pub fn read() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum RootCommand {
    #[command(subcommand)]
    Publication(PublicationCommand),
    #[command(subcommand)]
    Image(ImageCommand),
    #[command(subcommand)]
    Post(PostCommand),
}

#[derive(Debug, Subcommand)]
pub enum PublicationCommand {
    Get,
    Update(PublicationUpdateArguments),
    #[command(name = "set-logo")]
    SetLogo(PublicationImageArguments),
    #[command(name = "set-wide-logo")]
    SetWideLogo(PublicationImageArguments),
    #[command(name = "set-cover-photo")]
    SetCoverPhoto(PublicationImageArguments),
    #[command(name = "set-email-banner")]
    SetEmailBanner(PublicationImageArguments),
}

#[derive(Debug, Args)]
pub struct PublicationUpdateArguments {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub hero_text: Option<String>,
    #[arg(long)]
    pub language: Option<String>,
    #[arg(long)]
    pub copyright: Option<String>,
    #[arg(long)]
    pub logo_url: Option<String>,
    #[arg(long)]
    pub logo_url_wide: Option<String>,
    #[arg(long)]
    pub cover_photo_url: Option<String>,
    #[arg(long)]
    pub email_banner_url: Option<String>,
    #[arg(long)]
    pub theme_var_background_pop: Option<String>,
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "community_disabled")]
    pub community_enabled: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    pub community_disabled: bool,
}

impl PublicationUpdateArguments {
    pub fn community_enabled(&self) -> Option<bool> {
        if self.community_enabled {
            Some(true)
        } else if self.community_disabled {
            Some(false)
        } else {
            None
        }
    }
}

#[derive(Debug, Args)]
pub struct PublicationImageArguments {
    #[arg(long = "file")]
    pub file_path: String,
}

#[derive(Debug, Subcommand)]
pub enum ImageCommand {
    Upload(ImageUploadArguments),
}

#[derive(Debug, Args)]
pub struct ImageUploadArguments {
    #[arg(long = "file")]
    pub file_path: String,
}

#[derive(Debug, Subcommand)]
pub enum PostCommand {
    Create(PostCreateArguments),
    List(PostListArguments),
    Get(PostGetArguments),
    Delete(PostDeleteArguments),
}

#[derive(Debug, Args)]
pub struct PostCreateArguments {
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub subtitle: Option<String>,
    #[arg(long, conflicts_with = "file_path")]
    pub body: Option<String>,
    #[arg(long)]
    pub file_path: Option<String>,
    #[arg(long)]
    pub cover_image: Option<String>,
    #[arg(long, action = ArgAction::SetTrue)]
    pub draft: bool,
}

#[derive(Debug, Args)]
pub struct PostListArguments {
    #[arg(long, default_value_t = 10)]
    pub limit: u32,
}

#[derive(Debug, Args)]
pub struct PostGetArguments {
    pub post_id: u64,
    #[arg(long, action = ArgAction::SetTrue)]
    pub full: bool,
    #[arg(long)]
    pub save_html: Option<String>,
    #[arg(long)]
    pub save_json: Option<String>,
}

#[derive(Debug, Args)]
pub struct PostDeleteArguments {
    pub post_id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

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
}
