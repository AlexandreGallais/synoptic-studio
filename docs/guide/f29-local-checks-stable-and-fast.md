# F29 — The local checks stable and fast

`check:all`, run before each push, no longer fails at random under load, skips the Rust lanes when a branch changed no Rust input, and the release binaries follow a pull by themselves. A tooling feature, validated by the agent (VAL-030).

## What changed

| Before | After |
| --- | --- |
| `wrapped-paragraph`'s 16 000 words timed at 1 s failed about one push in three under load (1.14 s seen) | every timing limit of `src/`, `tests/` and `playground/` written beside its measure alone and at least four times it, `[F29.AC1]`; that test's limit 3 s (128 ms alone) |
| `check:rust` ran on every push, Markdown-only included | the Rust lanes (`check:rust`, `rust:coverage`, now apart) skipped when a branch changed none of their inputs against its base; every lane runs in the CI and on `main` or `develop` |
| after a pull, the release binaries stayed stale until a rebuild by hand | `post-merge` and `post-rewrite` rebuild them when the pull changed a Rust input (ADR-0089) |

## After the pull that brings F29

The first pull bringing these hooks still runs the old `git-hooks`, which has no `post-merge`: run `npm run rust:build` once in each checkout. A merge finished by hand after a conflict runs no `post-merge` either; `rust:fresh` names the stale binaries.

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. timing limits hold under load | `[F29.AC1]` (21 limits, made-up faults seen failing); `check:all` five times in a row at load averages up to 23 | passed |
| 2. the Rust lanes skipped without a Rust change | `f29_ac2_*` (inputs, every root entry classified, the CI guard) | passed |
| 3. the binaries rebuilt after a pull | `f29_ac3_*` on real pulls in temporary repositories, the failed build exiting 0 | passed |
