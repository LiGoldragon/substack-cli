use crate::cli::{
    CommandLine, ImageCommand, ImageUploadArguments, PostCommand, PostCreateArguments,
    PostDeleteArguments, PostGetArguments, PostListArguments, PostUpdateArguments,
    PublicationCommand, PublicationImageArguments, PublicationUpdateArguments, RootCommand,
};
use crate::client::Client;
use crate::error::Error;
use crate::image_file::{ImageFile, Mime};
use crate::prosemirror::{
    FrontmatterSplit, ImageRef, ImageSource, Markdown, ProseMirrorDoc, Table,
};
use crate::table_image::TableImage;
use crate::types::{
    ApiKey, DraftUpdate, Hostname, ImageUpload, ImageUrl, PostId, PostSummary, Publication,
    PublicationUpdate,
};
use std::path::{Path, PathBuf};

pub struct ApplicationConfig {
    hostname: Hostname,
    api_key: ApiKey,
}

impl ApplicationConfig {
    pub fn from_environment() -> Result<Self, Error> {
        let api_key = std::env::var("SUBSTACK_API_KEY")
            .map(ApiKey::from)
            .map_err(|_| Error::MissingEnvironmentVariable {
                variable: "SUBSTACK_API_KEY",
            })?;
        let hostname = std::env::var("SUBSTACK_HOSTNAME")
            .map(Hostname::from)
            .map_err(|_| Error::MissingEnvironmentVariable {
                variable: "SUBSTACK_HOSTNAME",
            })?;

        Ok(Self { hostname, api_key })
    }

    pub fn into_application(self) -> Application {
        Application::new(Client::new(self.hostname, self.api_key))
    }
}

pub struct Application {
    client: Client,
}

impl Application {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn run(&self, command_line: CommandLine) -> Result<(), Error> {
        match command_line.command {
            RootCommand::Publication(command) => self.run_publication(command).await,
            RootCommand::Image(command) => self.run_image(command).await,
            RootCommand::Post(command) => self.run_post(command).await,
        }
    }

    async fn run_publication(&self, command: PublicationCommand) -> Result<(), Error> {
        match command {
            PublicationCommand::Get => self.print_json(&self.client.publication().await?),
            PublicationCommand::Update(arguments) => {
                let publication = self.update_publication(arguments).await?;
                self.print_json(&publication)
            }
            PublicationCommand::SetLogo(arguments) => {
                self.set_publication_image(arguments, PublicationImageTarget::Logo)
                    .await
            }
            PublicationCommand::SetWideLogo(arguments) => {
                self.set_publication_image(arguments, PublicationImageTarget::WideLogo)
                    .await
            }
            PublicationCommand::SetCoverPhoto(arguments) => {
                self.set_publication_image(arguments, PublicationImageTarget::CoverPhoto)
                    .await
            }
            PublicationCommand::SetEmailBanner(arguments) => {
                self.set_publication_image(arguments, PublicationImageTarget::EmailBanner)
                    .await
            }
        }
    }

    async fn update_publication(
        &self,
        arguments: PublicationUpdateArguments,
    ) -> Result<Publication, Error> {
        let community_enabled = arguments.community_enabled();
        let update = PublicationUpdate {
            name: arguments.name,
            hero_text: arguments.hero_text,
            language: arguments.language,
            copyright: arguments.copyright,
            logo_url: arguments.logo_url.map(ImageUrl::from),
            logo_url_wide: arguments.logo_url_wide.map(ImageUrl::from),
            cover_photo_url: arguments.cover_photo_url.map(ImageUrl::from),
            email_banner_url: arguments.email_banner_url.map(ImageUrl::from),
            theme_var_background_pop: arguments.theme_var_background_pop,
            community_enabled,
            homepage_type: None,
        };

        self.client.update_publication(&update).await
    }

    async fn set_publication_image(
        &self,
        arguments: PublicationImageArguments,
        target: PublicationImageTarget,
    ) -> Result<(), Error> {
        let upload = self.upload_image_path(&arguments.file_path).await?;
        let update = target.publication_update(upload.url.clone());
        let publication = self.client.update_publication(&update).await?;
        let result = PublicationImageUpdate {
            uploaded_url: upload.url,
            publication,
        };

        self.print_json(&result)
    }

    fn hostname(&self) -> &Hostname {
        self.client.hostname()
    }

    async fn run_image(&self, command: ImageCommand) -> Result<(), Error> {
        match command {
            ImageCommand::Upload(arguments) => self.upload_image(arguments).await,
        }
    }

    async fn upload_image(&self, arguments: ImageUploadArguments) -> Result<(), Error> {
        let upload = self.upload_image_path(&arguments.file_path).await?;
        self.print_json(&upload)
    }

    async fn upload_image_path(&self, file_path: &str) -> Result<ImageUpload, Error> {
        let image = ImageFile::try_from(Path::new(file_path))?;
        self.client.upload_image(&image.to_data_uri(), None).await
    }

    /// Walk the markdown by `\n\n`-separated blocks; for each GFM pipe table,
    /// render it to a PNG, upload, and replace the block with an `![alt](url)`
    /// reference. The downstream image pipeline then emits a `captionedImage`.
    async fn render_and_upload_tables(&self, markdown: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(markdown.len());
        let mut first = true;
        for block in markdown.split("\n\n") {
            if !first {
                out.push_str("\n\n");
            }
            first = false;

            if let Some(table) = Table::parse_block(block.trim()) {
                let png = TableImage::new(table.header(), table.rows()).render_png()?;
                let image = ImageFile::from_bytes(png, Mime::PNG);
                let upload = self.client.upload_image(&image.to_data_uri(), None).await?;
                let alt = table.header().join(" | ").replace(['[', ']'], "");
                out.push_str(&format!("![{alt}]({url})", url = upload.url.as_str()));
            } else {
                out.push_str(block);
            }
        }
        Ok(out)
    }

    /// Walk the markdown body, upload any `![alt](local-path)` whose `src` is
    /// a local file path, and rewrite the markdown to reference the uploaded
    /// Substack CDN URL. Absolute `http(s)://` and `data:` URLs pass through.
    /// Relative paths resolve against `base_dir` when provided, otherwise cwd.
    async fn upload_inline_images(
        &self,
        markdown: &str,
        base_dir: Option<&Path>,
    ) -> Result<String, Error> {
        let mut out = String::with_capacity(markdown.len());
        let mut remaining = markdown;

        while let Some(start) = remaining.find("![") {
            out.push_str(&remaining[..start]);
            let tail = &remaining[start..];
            let Some(parsed) = ImageRef::parse_prefix(tail) else {
                out.push_str(&tail[..2]);
                remaining = &tail[2..];
                continue;
            };
            let consumed = parsed.consumed();
            let image = parsed.into_image();

            let resolved = match ImageSource::classify(image.src(), base_dir) {
                ImageSource::Remote(url) => url,
                ImageSource::Local(path) => self
                    .upload_image_path(&path.to_string_lossy())
                    .await?
                    .url
                    .as_str()
                    .to_string(),
            };
            out.push_str(&format!("![{alt}]({resolved})", alt = image.alt()));
            remaining = &tail[consumed..];
        }
        out.push_str(remaining);
        Ok(out)
    }

    async fn run_post(&self, command: PostCommand) -> Result<(), Error> {
        match command {
            PostCommand::Create(arguments) => self.create_post(arguments).await,
            PostCommand::Update(arguments) => self.update_post(arguments).await,
            PostCommand::List(arguments) => self.list_posts(arguments).await,
            PostCommand::Get(arguments) => self.get_post(arguments).await,
            PostCommand::Delete(arguments) => self.delete_post(arguments).await,
        }
    }

    async fn create_post(&self, arguments: PostCreateArguments) -> Result<(), Error> {
        let draft_only = arguments.draft;
        let request = PreparePostRequest {
            source: self.post_body_source(&arguments)?,
            title: arguments.title,
            subtitle: arguments.subtitle,
            cover_image: arguments.cover_image,
        };
        let prepared_post = self.prepare_post(request).await?;

        let user_id = self.client.user_id().await?;
        let draft = self.client.create_draft(user_id).await?;
        let update = DraftUpdate {
            draft_title: prepared_post.title.clone(),
            draft_subtitle: prepared_post.subtitle,
            draft_body: serde_json::to_string(&prepared_post.body)?,
            cover_image: prepared_post.cover_image_url,
        };

        self.client.update_draft(&draft.id, &update).await?;

        if draft_only {
            println!("Draft saved: {} (id: {})", prepared_post.title, draft.id);
        } else {
            let post = self.client.publish(&draft.id).await?;
            let slug = post.slug.unwrap_or_default();
            println!("Published: {}", prepared_post.title);
            println!("https://{}/p/{slug}", self.hostname());
        }

        Ok(())
    }

    async fn update_post(&self, arguments: PostUpdateArguments) -> Result<(), Error> {
        let request = PreparePostRequest {
            source: self.post_update_body_source(&arguments)?,
            title: arguments.title,
            subtitle: arguments.subtitle,
            cover_image: arguments.cover_image,
        };
        let prepared_post = self.prepare_post(request).await?;

        let post_id = PostId::from(arguments.post_id);
        let update = DraftUpdate {
            draft_title: prepared_post.title.clone(),
            draft_subtitle: prepared_post.subtitle,
            draft_body: serde_json::to_string(&prepared_post.body)?,
            cover_image: prepared_post.cover_image_url,
        };

        self.client.update_draft(&post_id, &update).await?;
        let post = self.client.publish(&post_id).await?;
        let slug = post.slug.unwrap_or_default();
        println!(
            "Updated post {}: {}",
            arguments.post_id, prepared_post.title
        );
        println!("https://{}/p/{slug}", self.hostname());
        Ok(())
    }

    async fn prepare_post(&self, request: PreparePostRequest) -> Result<PreparedPost, Error> {
        let PreparePostRequest {
            source,
            title,
            subtitle,
            cover_image,
        } = request;

        let FrontmatterSplit { frontmatter, body } =
            Markdown::from(source.markdown).split_frontmatter();

        let title = title
            .or_else(|| frontmatter.as_ref().and_then(|f| f.field("title")))
            .or_else(|| body.first_heading())
            .unwrap_or_else(|| "Untitled".into());
        let subtitle = subtitle.or_else(|| frontmatter.as_ref().and_then(|f| f.field("subtitle")));

        let body = body.without_leading_heading(&title);
        let body = self.render_and_upload_tables(body.as_str()).await?;
        let body = self
            .upload_inline_images(&body, source.base_dir.as_deref())
            .await?;
        let body = Markdown::from(body).to_prosemirror();

        let cover_image_url = match cover_image {
            Some(path) => Some(self.upload_image_path(&path).await?.url),
            None => None,
        };

        Ok(PreparedPost {
            title,
            subtitle,
            body,
            cover_image_url,
        })
    }

    fn post_body_source(&self, arguments: &PostCreateArguments) -> Result<PostSource, Error> {
        PostSource::from_body_or_file(arguments.body.as_deref(), arguments.file_path.as_deref())
    }

    fn post_update_body_source(
        &self,
        arguments: &PostUpdateArguments,
    ) -> Result<PostSource, Error> {
        PostSource::from_body_or_file(arguments.body.as_deref(), arguments.file_path.as_deref())
    }

    async fn list_posts(&self, arguments: PostListArguments) -> Result<(), Error> {
        let posts = self.client.list_posts(arguments.limit).await?;
        let summaries: Vec<PostSummary> = posts.iter().map(|post| post.summary()).collect();

        self.print_json(&summaries)
    }

    async fn get_post(&self, arguments: PostGetArguments) -> Result<(), Error> {
        let post = self.client.get_post(&PostId::from(arguments.post_id)).await?;
        let mut saved_files = Vec::new();

        if let Some(path) = arguments.save_html {
            let body = post.body_html.as_deref().ok_or_else(|| {
                Error::UnexpectedResponse("post response did not include body_html".into())
            })?;
            std::fs::write(&path, body)?;
            saved_files.push(SavedFile {
                kind: SavedFileKind::Html,
                path,
            });
        }

        if let Some(path) = arguments.save_json {
            let body = post.body_json.as_ref().ok_or_else(|| {
                Error::UnexpectedResponse("post response did not include body_json".into())
            })?;
            std::fs::write(&path, serde_json::to_string_pretty(body)?)?;
            saved_files.push(SavedFile {
                kind: SavedFileKind::Json,
                path,
            });
        }

        if arguments.full {
            self.print_json(&post)
        } else if saved_files.is_empty() {
            self.print_json(&post.summary())
        } else {
            self.print_json(&saved_files)
        }
    }

    async fn delete_post(&self, arguments: PostDeleteArguments) -> Result<(), Error> {
        self.client
            .delete_post(&PostId::from(arguments.post_id))
            .await?;
        println!("Deleted post {}", arguments.post_id);
        Ok(())
    }

    fn print_json<T: serde::Serialize>(&self, value: &T) -> Result<(), Error> {
        println!("{}", serde_json::to_string_pretty(value)?);
        Ok(())
    }
}

enum PublicationImageTarget {
    Logo,
    WideLogo,
    CoverPhoto,
    EmailBanner,
}

impl PublicationImageTarget {
    fn publication_update(&self, url: ImageUrl) -> PublicationUpdate {
        let mut update = PublicationUpdate::default();

        match self {
            Self::Logo => update.logo_url = Some(url),
            Self::WideLogo => update.logo_url_wide = Some(url),
            Self::CoverPhoto => update.cover_photo_url = Some(url),
            Self::EmailBanner => update.email_banner_url = Some(url),
        }

        update
    }
}

#[derive(serde::Serialize)]
struct PublicationImageUpdate {
    uploaded_url: ImageUrl,
    publication: Publication,
}

#[derive(serde::Serialize)]
struct SavedFile {
    kind: SavedFileKind,
    path: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum SavedFileKind {
    Html,
    Json,
}

struct PreparedPost {
    title: String,
    subtitle: Option<String>,
    body: ProseMirrorDoc,
    cover_image_url: Option<ImageUrl>,
}

struct PreparePostRequest {
    source: PostSource,
    title: Option<String>,
    subtitle: Option<String>,
    cover_image: Option<String>,
}

struct PostSource {
    markdown: String,
    base_dir: Option<PathBuf>,
}

impl PostSource {
    fn from_body_or_file(body: Option<&str>, file_path: Option<&str>) -> Result<Self, Error> {
        match (body, file_path) {
            (Some(body), None) => Ok(Self {
                markdown: body.to_string(),
                base_dir: None,
            }),
            (None, Some(path)) => Ok(Self {
                markdown: std::fs::read_to_string(path)?,
                base_dir: Path::new(path).parent().map(Path::to_path_buf),
            }),
            (Some(_), Some(_)) => Err(Error::Usage(
                "provide --body or --file-path, not both".into(),
            )),
            (None, None) => Err(Error::Usage("--body or --file-path is required".into())),
        }
    }
}
