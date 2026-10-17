//! `backlog-status <ID> <status>` (`npm run backlog:status`, RET-002, EN-103), from the
//! repository's root: the item's front matter, its folder's index and its parent's table. A story's
//! index is written again by the `backlog:index` script of `package.json`, run in the shell npm
//! would run it in, its output shown.

use std::env;
use std::process::ExitCode;

use repo_tools::backlog_status::{item_file, write_status};

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let id = arguments.first().map_or("", String::as_str);
    let status = arguments.get(1).map_or("", String::as_str);
    let root = env::current_dir().unwrap_or_default();
    let file = item_file(&root, id);

    if file.is_empty() || status.is_empty() {
        eprintln!("usage: npm run backlog:status -- <ID> <status>; no item {id}");
        return ExitCode::from(1);
    }

    match write_status(&root, &file, id, status) {
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
