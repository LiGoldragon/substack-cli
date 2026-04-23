use std::path::Path;

use crate::error::Error;

/// An image's MIME type. Construct via the `PNG` / `JPEG` / … constants or
/// `Mime::from_extension` for extension-based dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mime(&'static str);

impl Mime {
    pub const PNG: Self = Self("image/png");
    pub const JPEG: Self = Self("image/jpeg");
    pub const GIF: Self = Self("image/gif");
    pub const WEBP: Self = Self("image/webp");
    pub const SVG: Self = Self("image/svg+xml");

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl TryFrom<&str> for Mime {
    type Error = Error;

    /// Resolve a filename extension (case-sensitive, no leading dot) to a MIME.
    fn try_from(ext: &str) -> Result<Self, Error> {
        match ext {
            "png" => Ok(Self::PNG),
            "jpg" | "jpeg" => Ok(Self::JPEG),
            "gif" => Ok(Self::GIF),
            "webp" => Ok(Self::WEBP),
            "svg" => Ok(Self::SVG),
            other => Err(Error::UnsupportedImageFormat {
                extension: other.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

/// A `data:` URI suitable for Substack's image upload endpoint.
#[derive(Debug, Clone)]
pub struct DataUri(String);

impl DataUri {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DataUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DataUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An image file loaded into memory with its MIME type.
#[derive(Debug, Clone)]
pub struct ImageFile {
    data: Vec<u8>,
    mime: Mime,
}

impl ImageFile {
    pub fn from_bytes(data: Vec<u8>, mime: Mime) -> Self {
        Self { data, mime }
    }

    pub fn mime(&self) -> Mime {
        self.mime
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Encode as a `data:<mime>;base64,...` URI.
    pub fn to_data_uri(&self) -> DataUri {
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.data);
        DataUri(format!("data:{};base64,{b64}", self.mime))
    }
}

impl TryFrom<&Path> for ImageFile {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Error> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let mime = Mime::try_from(ext)?;
        let data = std::fs::read(path)?;
        Ok(Self { data, mime })
    }
}
