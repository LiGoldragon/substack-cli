use substack_cli::table_image::TableImage;

#[test]
fn renders_small_table_to_png_bytes() {
    let header = vec!["A".to_string(), "B".to_string()];
    let rows = vec![vec!["1".to_string(), "two".to_string()]];
    let bytes = TableImage::new(&header, &rows).render_png().unwrap();
    assert!(bytes.starts_with(&[0x89, 0x50, 0x4e, 0x47]));
}
