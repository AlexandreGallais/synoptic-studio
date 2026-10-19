//! Claude Code hooks of the repository (`docs/tooling/agent.md`), and what they share: reading the
//! JSON of hook inputs and transcript lines, their times, and the background tasks a session runs.
//! Written for the hooks of EN-047 and EN-101; every hook and the status line since EN-104.
//! The first request of each subagent is measured by `context-size` (EN-065).

pub mod context;
pub mod forbidden;
pub mod git;
pub mod guard;
pub mod handback;
pub mod heredoc;
pub mod json;
pub mod limit;
pub mod pages;
pub mod place;
pub mod record;
pub mod session;
pub mod shell;
pub mod status;
pub mod tasks;
pub mod time;
pub mod tokens;
pub mod worktrees;
pub mod written;
