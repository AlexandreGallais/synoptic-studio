//! The metrics of the agent's work (`the records' conventions, area metrics`, EN-107), rewritten from the TypeScript
//! scripts with their output unchanged: the metrics report (`target/release/metrics`), its findings of the
//! rules added, and the session report (`target/release/session-report`), both read from Claude Code's
//! transcripts incrementally.

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

pub mod work;
