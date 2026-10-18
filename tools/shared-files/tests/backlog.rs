//! Tests of the one finder of a backlog item's file by its id (AUD-025).

use std::fs;

use shared_files::backlog::{STORIES, file_of, names_in, story_file};

#[test]
fn finds_an_item_file_holding_its_id_or_starting_with_it() {
    let names = [
        "E17-F21-EN-103-a.md".to_owned(),
        "E17-F21-EN-1030-b.md".to_owned(),
        "F21-x.md".to_owned(),
    ];

    // `-EN-103-` is inside the first name only: `EN-1030` is another story.
    assert_eq!(
        file_of(&names, "EN-103").as_deref(),
        Some("E17-F21-EN-103-a.md")
    );
    // A feature's id is inside its stories' names too: the first name holding it wins.
    assert_eq!(
        file_of(&names, "F21").as_deref(),
        Some("E17-F21-EN-103-a.md")
    );
    assert_eq!(
        file_of(&names[1..], "F21").as_deref(),
        Some("E17-F21-EN-1030-b.md")
    );
    // `F21-x.md` starts with the id, `-F21-` nowhere inside it.
    assert_eq!(file_of(&names[2..], "F21").as_deref(), Some("F21-x.md"));
    assert_eq!(file_of(&names, "EN-9"), None);
    assert_eq!(file_of(&[], "EN-103"), None);
}

#[test]
fn reads_a_folder_sorted_and_finds_a_story_file_there() {
    let root = std::env::temp_dir().join(format!("backlog-finder-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join(STORIES)).unwrap();

    // Written out of order: the finder reads them sorted, so the older name wins.
    for name in ["E17-F23-EN-137-b.md", "E17-F22-EN-137-a.md", "README.md"] {
        fs::write(root.join(STORIES).join(name), "").unwrap();
    }

    assert_eq!(
        names_in(&root.join(STORIES)).unwrap(),
        ["E17-F22-EN-137-a.md", "E17-F23-EN-137-b.md", "README.md"]
    );
    assert_eq!(
        story_file(&root, "EN-137").unwrap().as_deref(),
        Some("E17-F22-EN-137-a.md")
    );
    assert_eq!(story_file(&root, "EN-138").unwrap(), None);
    // A checkout without a backlog: an error, not "no story".
    assert!(story_file(&root.join("missing"), "EN-137").is_err());

    let _ = fs::remove_dir_all(&root);
}
