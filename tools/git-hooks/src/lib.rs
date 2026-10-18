//! The repository's Git hooks (EN-108): the short shell files of `.githooks/` call the binary
//! `git-hooks`, which installs them (`install`) and runs their checks.

pub mod install;
