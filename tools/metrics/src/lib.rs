//! The metrics of the agent's work (`docs/tooling/metrics.md`, EN-107), ported from the TypeScript
//! scripts with their output unchanged: the metrics report (`npm run metrics`), its findings of the
//! rules added, and the session report (`npm run metrics:session`), both read from Claude Code's
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
