use std::path::{Path, PathBuf};
use substack_cli::Error;
use substack_cli::image_file::{ImageFile, Mime};

fn tmp_path(ext: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "substack-cli-image-{}-{ext}.{ext}",
        std::process::id(),
    ))
}

#[test]
fn png_file_encodes_as_base64_data_uri() {
    let path = tmp_path("png");
    std::fs::write(&path, [1_u8, 2, 3]).unwrap();

    let image = ImageFile::try_from(path.as_path()).unwrap();
    assert_eq!(image.mime(), Mime::PNG);
    assert_eq!(image.to_data_uri().as_str(), "data:image/png;base64,AQID");

    let _ = std::fs::remove_file(path);
}

#[test]
fn unsupported_extension_rejected() {
    let path = tmp_path("txt");
    std::fs::write(&path, [1_u8, 2, 3]).unwrap();

    let result = ImageFile::try_from(path.as_path());

    assert!(matches!(result, Err(Error::UnsupportedImageFormat { .. })));
    let _ = std::fs::remove_file(path);
}

#[test]
fn from_bytes_builds_data_uri_directly() {
    let uri = ImageFile::from_bytes(vec![1, 2, 3], Mime::PNG).to_data_uri();
    assert_eq!(uri.as_str(), "data:image/png;base64,AQID");
}

#[test]
fn mime_try_from_extension_covers_known_types() {
    assert_eq!(Mime::try_from("png").unwrap(), Mime::PNG);
    assert_eq!(Mime::try_from("jpg").unwrap(), Mime::JPEG);
    assert_eq!(Mime::try_from("jpeg").unwrap(), Mime::JPEG);
    assert_eq!(Mime::try_from("gif").unwrap(), Mime::GIF);
    assert_eq!(Mime::try_from("webp").unwrap(), Mime::WEBP);
    assert_eq!(Mime::try_from("svg").unwrap(), Mime::SVG);
    assert!(Mime::try_from("xyz").is_err());
    let _ = Path::new("/dev/null");
}
