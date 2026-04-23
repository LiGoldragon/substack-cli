use reqwest::header;

use crate::error::Error;
use crate::image_file::DataUri;
use crate::types::*;

pub struct Client {
    http: reqwest::Client,
    hostname: Hostname,
}

impl Client {
    pub fn new(hostname: Hostname, api_key: ApiKey) -> Self {
        let mut headers = header::HeaderMap::new();
        let cookie = format!("substack.sid={}", api_key.as_str());
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie).expect("invalid api key"),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("failed to build HTTP client");

        Self { http, hostname }
    }

    pub fn hostname(&self) -> &Hostname {
        &self.hostname
    }

    fn base(&self) -> String {
        format!("https://{}", self.hostname.as_str())
    }

    // ── Publication ──────────────────────────────────────────────

    pub async fn publication(&self) -> Result<Publication, Error> {
        Ok(self.get("/api/v1/publication").await?)
    }

    pub async fn update_publication(
        &self,
        update: &PublicationUpdate,
    ) -> Result<Publication, Error> {
        Ok(self.put("/api/v1/publication", update).await?)
    }

    // ── User ─────────────────────────────────────────────────────

    pub async fn user_id(&self) -> Result<UserId, Error> {
        let resp: PubUsersResponse = self.get("/api/v1/publication_user").await?;
        resp.pub_users
            .into_iter()
            .next()
            .map(|u| u.user_id)
            .ok_or(Error::NoUser)
    }

    // ── Drafts / Posts ───────────────────────────────────────────

    pub async fn create_draft(&self, user_id: UserId) -> Result<Draft, Error> {
        let body = DraftCreate {
            draft_bylines: vec![DraftByline {
                id: user_id.clone(),
                user_id,
            }],
        };
        Ok(self.post("/api/v1/drafts", &body).await?)
    }

    pub async fn update_draft(&self, draft_id: &PostId, update: &DraftUpdate) -> Result<(), Error> {
        let path = format!("/api/v1/drafts/{}", draft_id.as_u64());
        let resp = self
            .http
            .put(format!("{}{path}", self.base()))
            .json(update)
            .send()
            .await?;
        Self::check(resp).await?;
        Ok(())
    }

    pub async fn publish(&self, draft_id: &PostId) -> Result<Published, Error> {
        let path = format!("/api/v1/drafts/{}/publish", draft_id.as_u64());
        Ok(self.post(&path, &serde_json::json!({})).await?)
    }

    pub async fn list_posts(&self, limit: u32) -> Result<Vec<Post>, Error> {
        let path = format!("/api/v1/archive?sort=new&limit={limit}");
        Ok(self.get(&path).await?)
    }

    pub async fn get_post(&self, post_id: &PostId) -> Result<Post, Error> {
        let path = format!("/api/v1/drafts/{}", post_id.as_u64());
        Ok(self.get(&path).await?)
    }

    pub async fn delete_post(&self, post_id: &PostId) -> Result<(), Error> {
        let path = format!("/api/v1/drafts/{}", post_id.as_u64());
        let resp = self
            .http
            .delete(format!("{}{path}", self.base()))
            .send()
            .await?;
        Self::check(resp).await?;
        Ok(())
    }

    // ── Image ────────────────────────────────────────────────────

    pub async fn upload_image(
        &self,
        data_uri: &DataUri,
        post_id: Option<&PostId>,
    ) -> Result<ImageUpload, Error> {
        let mut body = serde_json::json!({ "image": data_uri.as_str() });
        if let Some(pid) = post_id {
            body["postId"] = serde_json::json!(pid.as_u64());
        }
        Ok(self.post("/api/v1/image", &body).await?)
    }

    // ── HTTP helpers ─────────────────────────────────────────────

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let resp = self
            .http
            .get(format!("{}{path}", self.base()))
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let resp = self
            .http
            .post(format!("{}{path}", self.base()))
            .json(body)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    async fn put<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let resp = self
            .http
            .put(format!("{}{path}", self.base()))
            .json(body)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    async fn check(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status: status.as_u16(),
                body,
            });
        }
        Ok(resp)
    }
}
