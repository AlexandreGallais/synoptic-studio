//! Shared files of the repository (ADR-0065, decision 4; EN-050): the indexes, the logs, the feature
//! files and the checklists, written by the main thread only, at merge. A worker's story branch
//! that edits one is refused at push (`shared`); the rows a worker writes for a log in its story
//! file are moved into that log when the story is merged (`rows`).

pub mod rows;
pub mod shared;
