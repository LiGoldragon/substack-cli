use crate::cli::{
    CommandLine, ImageCommand, ImageUploadArguments, PostCommand, PostCreateArguments,
    PostDeleteArguments, PostGetArguments, PostListArguments, PostUpdateArguments,
    PublicationCommand, PublicationImageArguments, PublicationUpdateArguments, RootCommand,
};
use crate::client::Client;
use crate::error::Error;
use crate::image_file::ImageFile;
use crate::prosemirror;
use crate::types::{DraftUpdate, ImageUpload, PostId, PostSummary, Publication, PublicationUpdate};
use serde_json::Value;

pub struct ApplicationConfig {
    hostname: String,
    api_key: String,
}

impl ApplicationConfig {
    pub fn from_environment() -> Result<Self, Error> {
        let api_key = std::env::var("SUBSTACK_API_KEY")
            .map_err(|_| Error::Usage("SUBSTACK_API_KEY must be set".into()))?;
        let hostname = std::env::var("SUBSTACK_HOSTNAME")
            .map_err(|_| Error::Usage("SUBSTACK_HOSTNAME must be set".into()))?;

        Ok(Self { hostname, api_key })
    }

    pub fn client(&self) -> Client {
        Client::new(&self.hostname, &self.api_key)
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

pub struct Application {
    client: Client,
    hostname: String,
}

impl Application {
    pub fn new(client: Client, hostname: String) -> Self {
        Self { client, hostname }
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
            logo_url: arguments.logo_url,
            logo_url_wide: arguments.logo_url_wide,
            cover_photo_url: arguments.cover_photo_url,
            email_banner_url: arguments.email_banner_url,
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
        let data_uri = ImageFile::data_uri(file_path)?;
        self.client.upload_image(&data_uri, None).await
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
        let raw = self.read_post_body(&arguments)?;
        let draft_only = arguments.draft;
        let prepared_post = self
            .prepare_post(
                arguments.title,
                arguments.subtitle,
                raw,
                arguments.cover_image.as_deref(),
            )
            .await?;

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
            println!("Draft saved: {} (id: {})", prepared_post.title, draft.id.0);
        } else {
            let post = self.client.publish(&draft.id).await?;
            let slug = post.slug.unwrap_or_default();
            println!("Published: {}", prepared_post.title);
            println!("https://{}/p/{slug}", self.hostname);
        }

        Ok(())
    }

    async fn update_post(&self, arguments: PostUpdateArguments) -> Result<(), Error> {
        let raw = self.read_post_update_body(&arguments)?;
        let prepared_post = self
            .prepare_post(
                arguments.title,
                arguments.subtitle,
                raw,
                arguments.cover_image.as_deref(),
            )
            .await?;

        let post_id = PostId(arguments.post_id);
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
        println!("https://{}/p/{slug}", self.hostname);
        Ok(())
    }

    async fn prepare_post(
        &self,
        title: Option<String>,
        subtitle: Option<String>,
        raw: String,
        cover_image: Option<&str>,
    ) -> Result<PreparedPost, Error> {
        let (frontmatter, body) = prosemirror::strip_frontmatter(&raw);
        let title = title
            .or_else(|| prosemirror::frontmatter_field(&frontmatter, "title"))
            .or_else(|| prosemirror::extract_first_heading(&body))
            .unwrap_or_else(|| "Untitled".into());
        let subtitle =
            subtitle.or_else(|| prosemirror::frontmatter_field(&frontmatter, "subtitle"));
        let body = prosemirror::strip_leading_heading(&body, &title);
        let body = prosemirror::from_markdown(&body);
        let cover_image_url = match cover_image {
            Some(path) => Some(self.upload_image_path(path).await?.url),
            None => None,
        };

        Ok(PreparedPost {
            title,
            subtitle,
            body,
            cover_image_url,
        })
    }

    fn read_post_body(&self, arguments: &PostCreateArguments) -> Result<String, Error> {
        match (&arguments.body, &arguments.file_path) {
            (Some(body), None) => Ok(body.clone()),
            (None, Some(path)) => Ok(std::fs::read_to_string(path)?),
            (Some(_), Some(_)) => Err(Error::Usage(
                "provide --body or --file-path, not both".into(),
            )),
            (None, None) => Err(Error::Usage("--body or --file-path is required".into())),
        }
    }

    fn read_post_update_body(&self, arguments: &PostUpdateArguments) -> Result<String, Error> {
        match (&arguments.body, &arguments.file_path) {
            (Some(body), None) => Ok(body.clone()),
            (None, Some(path)) => Ok(std::fs::read_to_string(path)?),
            (Some(_), Some(_)) => Err(Error::Usage(
                "provide --body or --file-path, not both".into(),
            )),
            (None, None) => Err(Error::Usage("--body or --file-path is required".into())),
        }
    }

    async fn list_posts(&self, arguments: PostListArguments) -> Result<(), Error> {
        let posts = self.client.list_posts(arguments.limit).await?;
        let summaries: Vec<PostSummary> = posts
            .into_iter()
            .map(|post| PostSummary {
                id: post.id,
                title: post.title,
                slug: post.slug,
                post_date: post.post_date,
                audience: post.audience,
                wordcount: post.wordcount,
            })
            .collect();

        self.print_json(&summaries)
    }

    async fn get_post(&self, arguments: PostGetArguments) -> Result<(), Error> {
        let post = self.client.get_post(&PostId(arguments.post_id)).await?;
        let mut saved_files = Vec::new();

        if let Some(path) = arguments.save_html {
            let body = post.body_html.as_deref().ok_or_else(|| {
                Error::UnexpectedResponse("post response did not include body_html".into())
            })?;
            std::fs::write(&path, body)?;
            saved_files.push(SavedFile { kind: "html", path });
        }

        if let Some(path) = arguments.save_json {
            let body = post.body_json.as_ref().ok_or_else(|| {
                Error::UnexpectedResponse("post response did not include body_json".into())
            })?;
            std::fs::write(&path, serde_json::to_string_pretty(body)?)?;
            saved_files.push(SavedFile { kind: "json", path });
        }

        if arguments.full {
            self.print_json(&post)
        } else if saved_files.is_empty() {
            self.print_json(&post.meta)
        } else {
            self.print_json(&saved_files)
        }
    }

    async fn delete_post(&self, arguments: PostDeleteArguments) -> Result<(), Error> {
        self.client.delete_post(&PostId(arguments.post_id)).await?;
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
    fn publication_update(&self, url: String) -> PublicationUpdate {
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
    uploaded_url: String,
    publication: Publication,
}

#[derive(serde::Serialize)]
struct SavedFile {
    kind: &'static str,
    path: String,
}

struct PreparedPost {
    title: String,
    subtitle: Option<String>,
    body: Value,
    cover_image_url: Option<String>,
}
