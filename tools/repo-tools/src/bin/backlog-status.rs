//! `backlog-status <ID> <status>` (`npm run backlog:status`, RET-002, EN-103), from the
//! repository's root: the item's front matter, its folder's index and its parent's table. A story's
//! index is written again by the `backlog:index` script of `package.json`, run in the shell npm
//! would run it in, its output shown.

use std::env;
use std::io;
use std::path::Path;
use std::process::ExitCode;

use repo_tools::backlog_status::{
    file_of, folder_of, names_in, parent_of, rewrite, with_front_status, with_row_status,
};
use repo_tools::scripts::run_script;

/// Changes an item's status in its file, its index and its parent's table; false when the stories
/// index could not be written.
fn write_status(root: &Path, folder: &str, file: &str, id: &str, status: &str) -> io::Result<bool> {
    let backlog = root.join("docs").join("backlog");
    let path = backlog.join(folder).join(file);
    let text = std::fs::read_to_string(&path)?;
    let parent = parent_of(&backlog, folder, &text)?;
    let mut indexed = true;

    rewrite(&path, |text| with_front_status(text, status))?;
    if folder == "stories" {
        indexed = run_script(root, "backlog:index")?.success();
    } else {
        rewrite(&backlog.join(folder).join("README.md"), |text| {
            with_row_status(text, id, status)
        })?;
    }
    if folder != "epics" {
        rewrite(&parent, |text| with_row_status(text, id, status))?;
    }
    Ok(indexed)
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let id = arguments.first().map_or("", String::as_str);
    let status = arguments.get(1).map_or("", String::as_str);
    let root = env::current_dir().unwrap_or_default();
    let folder = folder_of(id);
    let file = names_in(&root.join("docs").join("backlog").join(folder))
        .map(|names| file_of(&names, id))
        .unwrap_or_default();

    if file.is_empty() || status.is_empty() {
        eprintln!("usage: npm run backlog:status -- <ID> <status>; no item {id}");
        return ExitCode::from(1);
    }

    match write_status(&root, folder, &file, id, status) {
        Ok(indexed) => {
            println!("{id} {status}: {file}, its index and its parent");
            if indexed {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(error) => {
            eprintln!("backlog-status: {error}");
            ExitCode::from(1)
        }
    }
}
