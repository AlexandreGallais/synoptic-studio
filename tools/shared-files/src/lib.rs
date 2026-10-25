//! The rows a story's file holds for the logs (ADR-0091): the rows and cards a worker writes
//! under `## For the main thread` are moved into the logs by the story's `done` commit (`rows`),
//! the journal's under a section dated today (`calendar::today`). Since ADR-0102 a worker may write
//! the logs, the indexes and the feature files itself, so the move is no longer required, and no
//! push is refused for them. A story's file is found by its id (`backlog`), a branch's story by
//! its name (`shared`).

pub mod backlog;
pub mod rows;
pub mod shared;
