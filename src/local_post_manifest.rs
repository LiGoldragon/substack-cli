use crate::error::Error;
use crate::types::{Hostname, PostId};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalPostManifest {
    #[serde(default)]
    posts: Vec<LocalPostRecord>,
}

impl LocalPostManifest {
    fn find(&self, source_path: &SourcePath) -> Option<PublishedLocalPost> {
        self.posts
            .iter()
            .find(|post| post.source_path == *source_path)
            .and_then(LocalPostRecord::published_post)
    }

    fn find_record(&self, source_path: &SourcePath) -> Option<&LocalPostRecord> {
        self.posts.iter().find(|post| post.source_path == *source_path)
    }

    fn upsert(&mut self, record: LocalPostRecord) {
        if let Some(existing) = self
            .posts
            .iter_mut()
            .find(|post| post.source_path == record.source_path)
        {
            existing.merge(record);
            return;
        }
        self.posts.push(record);
        self.posts.sort_by(|a, b| a.source_path.as_str().cmp(b.source_path.as_str()));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocalPostRecord {
    source_path: SourcePath,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    post_id: Option<PostId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    banner_image: Option<SourcePath>,
}

impl LocalPostRecord {
    fn published_post(&self) -> Option<PublishedLocalPost> {
        Some(PublishedLocalPost {
            post_id: self.post_id.clone()?,
            slug: self.slug.clone()?,
        })
    }

    fn merge(&mut self, update: LocalPostRecord) {
        if let Some(post_id) = update.post_id {
            self.post_id = Some(post_id);
        }
        if let Some(slug) = update.slug {
            self.slug = Some(slug);
        }
        if let Some(banner_image) = update.banner_image {
            self.banner_image = Some(banner_image);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SourcePath(String);

impl SourcePath {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SourcePath {
    fn from(path: String) -> Self {
        Self(path)
    }
}

#[derive(Debug, Clone)]
pub struct PublishedLocalPost {
    post_id: PostId,
    slug: String,
}

impl PublishedLocalPost {
    pub fn canonical_url(&self, hostname: &Hostname) -> String {
        format!("https://{}/p/{}", hostname, self.slug)
    }

    pub fn post_id(&self) -> &PostId {
        &self.post_id
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }
}

pub struct LocalPostManifestFile {
    path: PathBuf,
    manifest: LocalPostManifest,
}

impl LocalPostManifestFile {
    pub fn discover(
        explicit_path: Option<&str>,
        base_dir: Option<&Path>,
    ) -> Result<Self, Error> {
        let path = match explicit_path {
            Some(path) => PathBuf::from(path),
            None => Self::default_path(base_dir)?,
        };
        Self::load(path)
    }

    fn default_path(base_dir: Option<&Path>) -> Result<PathBuf, Error> {
        let start = match base_dir {
            Some(path) => path.to_path_buf(),
            None => std::env::current_dir()?,
        };
        let absolute = absolute_path(&start)?;
        let mut root = absolute.clone();
        for candidate in absolute.ancestors() {
            if candidate.join(".jj").exists() || candidate.join(".git").exists() {
                root = candidate.to_path_buf();
                break;
            }
        }
        Ok(root.join(".substack-posts.json"))
    }

    fn load(path: PathBuf) -> Result<Self, Error> {
        if path.exists() {
            let text = std::fs::read_to_string(&path)?;
            let manifest = serde_json::from_str(&text)?;
            Ok(Self { path, manifest })
        } else {
            Ok(Self {
                path,
                manifest: LocalPostManifest::default(),
            })
        }
    }

    pub fn published_post(&self, source_path: &Path) -> Result<Option<PublishedLocalPost>, Error> {
        let source_path = self.source_path(source_path)?;
        Ok(self.manifest.find(&source_path))
    }

    pub fn record_post(
        &mut self,
        source_path: &Path,
        post_id: PostId,
        slug: String,
    ) -> Result<PublishedLocalPost, Error> {
        let source_path = self.source_path(source_path)?;
        let record = LocalPostRecord {
            source_path,
            post_id: Some(post_id),
            slug: Some(slug),
            banner_image: None,
        };
        let published_post = record.published_post().expect("published post");
        self.manifest.upsert(record);
        Ok(published_post)
    }

    pub fn record_banner_image(
        &mut self,
        source_path: &Path,
        banner_image: &Path,
    ) -> Result<(), Error> {
        let source_path = self.source_path(source_path)?;
        let banner_image = self.source_path(banner_image)?;
        let record = LocalPostRecord {
            source_path,
            post_id: None,
            slug: None,
            banner_image: Some(banner_image),
        };
        self.manifest.upsert(record);
        Ok(())
    }

    pub fn banner_image_path(&self, source_path: &Path) -> Result<Option<PathBuf>, Error> {
        let source_path = self.source_path(source_path)?;
        let Some(record) = self.manifest.find_record(&source_path) else {
            return Ok(None);
        };
        let Some(banner_image) = record.banner_image.as_ref() else {
            return Ok(None);
        };
        Ok(Some(self.manifest_root()?.join(banner_image.as_str())))
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = serde_json::to_string_pretty(&self.manifest)?;
        std::fs::write(&self.path, format!("{text}\n"))?;
        Ok(())
    }

    fn source_path(&self, source_path: &Path) -> Result<SourcePath, Error> {
        let source_path = source_path.canonicalize().map_err(|_| Error::MissingLinkedFile {
            path: source_path.display().to_string(),
        })?;
        let manifest_root = self.manifest_root()?;
        let relative = source_path
            .strip_prefix(&manifest_root)
            .map_err(|_| Error::LinkedFileOutsideManifestRoot {
                path: source_path.display().to_string(),
                manifest_root: manifest_root.display().to_string(),
            })?;
        Ok(SourcePath::from(relative.to_string_lossy().to_string()))
    }

    fn manifest_root(&self) -> Result<PathBuf, Error> {
        let root = self
            .path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        if root.exists() {
            return Ok(root.canonicalize()?);
        }
        Ok(absolute_path(&root)?)
    }
}

fn absolute_path(path: &Path) -> Result<PathBuf, Error> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    Ok(std::env::current_dir()?.join(path))
}
