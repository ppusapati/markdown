use markdown_app::search::SearchService;
use std::fs;

#[test]
fn indexes_markdown_content_tags_headings_and_mermaid() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::write(root.join("note.md"), "# Project Plan\n\nThis covers offline search. #rust\n\n```mermaid\ngraph TD; A-->B;\n```\n").unwrap();

    let mut service = SearchService::default();
    let indexed = service.build_index(root, None).unwrap();
    assert_eq!(indexed, 1);

    let tag_hits = service.query(root, "rust", 10, None).unwrap();
    assert_eq!(tag_hits[0].path.to_string_lossy(), "note.md");

    let mermaid_hits = service.query(root, "graph", 10, None).unwrap();
    assert_eq!(mermaid_hits[0].title, "Project Plan");
}

#[test]
fn updates_changed_files_and_removes_deleted_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::write(root.join("note.md"), "# Alpha\n\nfirst").unwrap();

    let mut service = SearchService::default();
    service.build_index(root, None).unwrap();
    fs::write(root.join("note.md"), "# Beta\n\nsecond").unwrap();
    service
        .update_index(root, &["note.md".into()], None)
        .unwrap();
    assert!(service.query(root, "second", 10, None).unwrap().len() == 1);

    fs::remove_file(root.join("note.md")).unwrap();
    service
        .update_index(root, &["note.md".into()], None)
        .unwrap();
    assert!(service.query(root, "second", 10, None).unwrap().is_empty());
}
