//! Claude Code hooks of the repository (`docs/tooling/agent.md`), and what they share: reading the
//! JSON of hook inputs and transcript lines, their times, and the background tasks a session runs.
//! Written for the hooks of EN-047; the status line joins them when F19 ports it to Rust.

pub mod json;
pub mod limit;
pub mod tasks;
pub mod time;
