use crate::error::Error;

pub struct ImageFile;

impl ImageFile {
    pub fn data_uri(path: &str) -> Result<String, Error> {
        let data = std::fs::read(path)?;
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let mime = match ext {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => return Err(Error::InvalidImage(format!("unsupported format: {ext}"))),
        };

        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
        Ok(format!("data:{mime};base64,{b64}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_png_data_uri() {
        let path = std::env::temp_dir().join(format!(
            "substack-cli-image-{}-{}.png",
            std::process::id(),
            "png"
        ));
        std::fs::write(&path, [1_u8, 2, 3]).unwrap();

        let data_uri = ImageFile::data_uri(path.to_str().unwrap()).unwrap();

        assert_eq!(data_uri, "data:image/png;base64,AQID");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn rejects_unsupported_extension() {
        let path = std::env::temp_dir().join(format!(
            "substack-cli-image-{}-{}.txt",
            std::process::id(),
            "txt"
        ));
        std::fs::write(&path, [1_u8, 2, 3]).unwrap();

        let result = ImageFile::data_uri(path.to_str().unwrap());

        assert!(matches!(result, Err(Error::InvalidImage(_))));
        let _ = std::fs::remove_file(path);
    }
}
