//! The API pages of the documentation site, written from the library's exports and their
//! `TSDoc` (EN-072, replacing `TypeDoc`): one Markdown page per exported function, type alias and
//! constant, their index and the sidebar file `VitePress` reads, after the three checks
//! `TypeDoc`'s configuration made.

pub mod api;
pub mod doc;
pub mod extract;
pub mod index;
pub mod model;
pub mod order;
pub mod page;

use std::fs;
use std::io;
use std::path::Path;

use model::Export;
use page::{Names, Page};

/// The file `VitePress` reads the API sidebar from, next to the pages.
pub const SIDEBAR_FILE: &str = "api-sidebar.json";

/// Where the pages go and how the site names them.
pub struct Output<'p> {
    /// The folder of the pages, emptied first.
    pub folder: &'p Path,
    /// The pages' path on the site, `/api/`.
    pub site_path: &'p str,
    /// The title of the index and of each breadcrumb.
    pub title: &'p str,
}

/// The exports by name.
#[must_use]
pub fn names(exports: &[Export]) -> Names {
    exports
        .iter()
        .map(|each| (each.name.clone(), each.kind))
        .collect()
}

/// The pages of an API, by path relative to the output folder.
#[must_use]
pub fn pages(exports: &[Export], output: &Output) -> Vec<(String, String)> {
    let names = names(exports);
    let mut pages: Vec<(String, String)> = exports
        .iter()
        .map(|export| {
            (
                format!("{}/{}.md", export.kind.folder(), export.name),
                Page::new(&names, export.kind).render(export, output.title),
            )
        })
        .collect();
    pages.push(("index.md".to_owned(), index::index(exports, output.title)));
    pages.push((
        SIDEBAR_FILE.to_owned(),
        index::sidebar(exports, output.site_path),
    ));
    pages
}

/// What stops the pages: what cannot be read.
#[must_use]
pub fn problems(api: &api::Api) -> Vec<String> {
    api.problems.clone()
}

/// The pages written into an emptied folder.
///
/// # Errors
///
/// When the folder cannot be emptied or a page cannot be written.
pub fn write(pages: &[(String, String)], folder: &Path) -> io::Result<()> {
    if folder.exists() {
        fs::remove_dir_all(folder)?;
    }
    for (path, text) in pages {
        let path = folder.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, text)?;
    }
    Ok(())
}
