//! The docs a rule touches (EN-059, F18.AC16; RET-003, row 30): a story whose commits change a rule
//! in `docs/domain/` brings, by its `done` commit at the latest, the Q row of
//! `docs/domain/README.md`, its feature's VAL decisions to confirm and its guide page — or the
//! `done` commit's message holds a line saying none is affected. Checked by the `commit-msg` hook.

pub mod check;
