//! The metrics of the agent's work (`docs/tooling/metrics.md`, EN-107), rewritten from the TypeScript
//! scripts with their output unchanged: the metrics report (`target/release/metrics`), its findings of the
//! rules added, and the session report (`target/release/session-report`), both read from Claude Code's
//! transcripts incrementally; what the agents do (`target/release/activity`); the watchdog of a
//! background task under Claude Code's Monitor (`watch-task`).

pub mod activity;
pub mod cache;
pub mod findings;
pub mod format;
pub mod git;
pub mod json;
pub mod report;
pub mod session;
pub mod tables;
pub mod time;
pub mod tools;
pub mod transcript;
pub mod turn_time;
pub mod turns;
pub mod watch;
pub mod work;
