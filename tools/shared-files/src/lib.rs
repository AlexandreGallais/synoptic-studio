//! The story a branch is for, read from its name (`shared`), the one reader of `git-hooks`,
//! `story-finish` and `check-all`. The rows a story file held for the logs, which `move-rows` moved
//! in its `done` commit (ADR-0091), went with the backlog into the records (EN-154): a story has
//! no file nor `done` commit any more.

pub mod shared;
