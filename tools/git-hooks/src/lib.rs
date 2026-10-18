//! The repository's Git hooks (EN-108): the short shell files of `.githooks/` call the binary
//! `git-hooks`, which installs them (`install`) and runs their checks: the tasks on the staged
//! files (`staged`, `pre_commit`).

pub mod install;
pub mod pre_commit;
pub mod staged;
