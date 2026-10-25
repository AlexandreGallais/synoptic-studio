//! The project's records in one SQLite file, `$AGENT_RECORDS_ROOT/<repo>.sqlite` (E49, research
//! note 0035): where it lives, its store (EN-151), its daily snapshot into the synced folder
//! (EN-153), the backlog and every other record in tables of their own, read and written by free
//! SQL (EN-154), and read by `search`, `show` and `list` from those tables and their full-text
//! index (AUD-057).

pub mod add;
pub mod backlog;
pub mod data_files;
pub mod document;
pub mod front_matter;
pub mod moved;
pub mod paths;
pub mod private_records;
pub mod project;
pub mod read;
pub mod records;
pub mod snapshot;
pub mod sql;
pub mod store;
pub mod sync;
pub mod transcripts;
