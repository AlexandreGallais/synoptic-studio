//! The backlog and repository tools the agent runs at each story and each push, ported from
//! `scripts/` with their output unchanged (EN-103, F21.AC2): one binary each under `src/bin/`,
//! what they decide here, tested without Git where it can be.

pub mod check_all;
pub mod scripts;
