//! The project's records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>/records.sqlite` (E49,
//! research note 0035): where it lives, its store, the knowledge tables first (EN-151), and the
//! documents a branch adds, reads and marks merged (EN-152), its daily snapshot into the synced
//! folder (EN-153), and the backlog in tables of its own, read and written by free SQL (EN-154).

pub mod add;
pub mod backlog;
pub mod document;
pub mod front_matter;
pub mod links;
pub mod paths;
pub mod project;
pub mod read;
pub mod records;
pub mod snapshot;
pub mod sql;
pub mod store;
pub mod sync;
