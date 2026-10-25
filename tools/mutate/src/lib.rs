//! Mutation runner (EN-058, F18.AC15): changes one thing at a time in each source file — a
//! comparison flipped, an operator swapped, a constant changed, a condition negated, a return
//! replaced — in a temporary copy of the repository, runs the file's related tests, and reports the
//! mutations caught and those that survive. Stryker waits for stryker-js issue 6210.

mod mutations;
mod runner;

pub use mutations::{Kind, Language, Mutation, code_mask, mutations, selected};
pub use runner::{
    Outcome, Request, VITEST_RELATED, Verdict, is_build_failure, left_copy_pid, run, summary,
    test_file,
};
