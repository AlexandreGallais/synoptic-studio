//! Shared files of the repository (ADR-0065, decision 4; EN-050; ADR-0091): the indexes, the logs,
//! the feature files and the checklists, written by the main thread only. A worker's story branch
//! that edits one is refused at push, except by the story's `done` commit, which the main thread
//! makes (`shared`); the rows and cards a worker writes for a log in its story file are moved into
//! that log by that `done` commit (`rows`), the journal's under a section dated today
//! (`calendar::today`). A change that only pads or unpads a shared file's tables passes
//! (`padding`, EN-073).

pub mod padding;
pub mod rows;
pub mod shared;
