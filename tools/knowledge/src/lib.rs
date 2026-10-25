//! The project's records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>/records.sqlite` (E49,
//! research note 0035): where it lives, its store, the knowledge tables first (EN-151), and the
//! documents a branch adds, reads and marks merged (EN-152).

pub mod add;
pub mod document;
pub mod read;
pub mod records;
pub mod store;
pub mod sync;
