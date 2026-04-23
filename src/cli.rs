use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "substack")]
#[command(about = "Create, publish, and manage Substack posts")]
pub struct CommandLine {
    #[command(subcommand)]
    pub command: RootCommand,
}

impl CommandLine {
    pub fn from_args() -> Self {
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
    Update(PostUpdateArguments),
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
pub struct PostUpdateArguments {
    pub post_id: u64,
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

