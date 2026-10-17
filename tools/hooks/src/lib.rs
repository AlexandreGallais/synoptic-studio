//! Claude Code hooks of the repository (`docs/tooling/agent.md`), and what they share: reading the
//! JSON of hook inputs and transcript lines, their times, and the background tasks a session runs.
//! Written for the hooks of EN-047 and EN-101; the status line joins them when F19 ports it to Rust.
//! The first request of each subagent is measured by `context:size` (EN-065).

pub mod context;
pub mod handback;
pub mod json;
pub mod limit;
pub mod pages;
pub mod tasks;
pub mod time;
