//! The metrics of the agent's work (`docs/tooling/metrics.md`, EN-107), ported from the TypeScript
//! scripts with their output unchanged: the session report (`npm run metrics:session`), read from
//! Claude Code's transcripts incrementally.

pub mod cache;
pub mod format;
pub mod git;
pub mod json;
pub mod session;
pub mod transcript;
pub mod turns;
pub mod work;
