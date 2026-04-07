use reqwest::header;
use serde::{Deserialize, Serialize};

pub struct Client {
    http: reqwest::Client,
    base: String,
}

#[derive(Debug, Deserialize)]
pub struct PubUser {
    pub user_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct PubUsersResponse {
    pub pub_users: Vec<PubUser>,
}

#[derive(Debug, Deserialize)]
pub struct Draft {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub struct DraftUpdate {
    pub draft_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_subtitle: Option<String>,
    pub draft_body: String,
}

#[derive(Debug, Deserialize)]
pub struct PublishedPost {
    #[serde(default)]
    pub slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub handle: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
}

impl Client {
    pub fn new(hostname: &str, api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        let cookie = format!("substack.sid={api_key}");
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie).expect("invalid api key"),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build HTTP client");

        Self {
            http,
            base: format!("https://{hostname}"),
        }
    }

    pub async fn user_id(&self) -> Result<u64, Error> {
        let resp: PubUsersResponse = self
            .http
            .get(format!("{}/api/v1/publication_user", self.base))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        resp.pub_users
            .first()
            .map(|u| u.user_id)
            .ok_or(Error::NoUser)
    }

    pub async fn create_draft(&self, user_id: u64) -> Result<Draft, Error> {
        let draft: Draft = self
            .http
            .post(format!("{}/api/v1/drafts", self.base))
            .json(&serde_json::json!({
                "draft_bylines": [{ "id": user_id, "user_id": user_id }]
            }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(draft)
    }

    pub async fn update_draft(
        &self,
        draft_id: u64,
        update: &DraftUpdate,
    ) -> Result<(), Error> {
        self.http
            .put(format!("{}/api/v1/drafts/{draft_id}", self.base))
            .json(update)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn publish(&self, draft_id: u64) -> Result<PublishedPost, Error> {
        let post: PublishedPost = self
            .http
            .post(format!("{}/api/v1/drafts/{draft_id}/publish", self.base))
            .json(&serde_json::json!({}))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(post)
    }

    pub async fn profile(&self) -> Result<ProfileResponse, Error> {
        let resp: ProfileResponse = self
            .http
            .get(format!("{}/api/v1/subscriber/profile", self.base))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp)
    }
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    NoUser,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "{e}"),
            Self::NoUser => write!(f, "no publication user found"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}
