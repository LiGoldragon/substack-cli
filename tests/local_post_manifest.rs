use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use substack_cli::local_post_manifest::LocalPostManifestFile;
use substack_cli::types::PostId;

#[test]
fn records_and_loads_published_local_posts() {
    let root = unique_temp_dir();
    std::fs::create_dir_all(root.join("water")).unwrap();
    std::fs::write(root.join("water/Keep_the_Plasma.md"), "# Keep the Plasma\n").unwrap();
    std::fs::create_dir_all(root.join("generated-images")).unwrap();
    std::fs::write(root.join("generated-images/keep-the-plasma-banner.png"), "fake").unwrap();
    let manifest_path = root.join(".substack-posts.json");

    let mut manifest =
        LocalPostManifestFile::discover(Some(manifest_path.to_str().unwrap()), Some(&root))
            .unwrap();
    manifest
        .record_banner_image(
            &root.join("water/Keep_the_Plasma.md"),
            &root.join("generated-images/keep-the-plasma-banner.png"),
        )
        .unwrap();
    let published = manifest
        .record_post(
            &root.join("water/Keep_the_Plasma.md"),
            PostId::from(195628661),
            "keep-the-plasma".to_string(),
        )
        .unwrap();
    manifest.save().unwrap();

    assert_eq!(published.post_id().as_u64(), 195628661);
    assert_eq!(published.slug(), "keep-the-plasma");

    let manifest =
        LocalPostManifestFile::discover(Some(manifest_path.to_str().unwrap()), Some(&root))
            .unwrap();
    let loaded = manifest
        .published_post(&root.join("water/Keep_the_Plasma.md"))
        .unwrap()
        .unwrap();
    assert_eq!(loaded.post_id().as_u64(), 195628661);
    assert_eq!(loaded.slug(), "keep-the-plasma");
    assert_eq!(
        manifest
            .banner_image_path(&root.join("water/Keep_the_Plasma.md"))
            .unwrap()
            .unwrap(),
        root.join("generated-images/keep-the-plasma-banner.png")
    );
}

#[test]
fn records_banner_for_unpublished_local_post() {
    let root = unique_temp_dir();
    std::fs::create_dir_all(root.join("water")).unwrap();
    std::fs::create_dir_all(root.join("generated-images")).unwrap();
    std::fs::write(root.join("water/The_Distilled_Water_Paradox.md"), "# The Distilled Water Paradox\n").unwrap();
    std::fs::write(
        root.join("generated-images/the-distilled-water-paradox-banner.png"),
        "fake",
    )
    .unwrap();
    let manifest_path = root.join(".substack-posts.json");

    let mut manifest =
        LocalPostManifestFile::discover(Some(manifest_path.to_str().unwrap()), Some(&root))
            .unwrap();
    manifest
        .record_banner_image(
            &root.join("water/The_Distilled_Water_Paradox.md"),
            &root.join("generated-images/the-distilled-water-paradox-banner.png"),
        )
        .unwrap();
    manifest.save().unwrap();

    let manifest =
        LocalPostManifestFile::discover(Some(manifest_path.to_str().unwrap()), Some(&root))
            .unwrap();
    assert!(manifest
        .published_post(&root.join("water/The_Distilled_Water_Paradox.md"))
        .unwrap()
        .is_none());
    assert_eq!(
        manifest
            .banner_image_path(&root.join("water/The_Distilled_Water_Paradox.md"))
            .unwrap()
            .unwrap(),
        root.join("generated-images/the-distilled-water-paradox-banner.png")
    );
}

fn unique_temp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("substack-cli-test-{timestamp}"));
    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }
    std::fs::create_dir_all(&root).unwrap();
    root
}
