# ADR-0073 — Every new script, hook or agent tool written in Rust

**Status**: Accepted (Product Owner, decided 2026-10-16 for E17, recorded on 2026-10-17 at the review of the ADRs, EN-094). Supersedes ADR-0050's decision 1 (the agent's tools written in TypeScript). Decision 1 partly superseded by ADR-0078 (2026-10-18): the hooks, the status line, the Git hooks and the agent's own commands call `target/release/<binary>` by its path.

## Context

- ADR-0050 (RET-002) made the agent's tools repository scripts in TypeScript, and left a project of Rust tools until a tool's own execution, not its writing, was measured as the cost.
- E17 measured it: the agent runs the repository's tools hundreds of times a session, and a slow tool costs it rounds of waiting and checking again (ADR-0069). Its first Rust tools — the heavy lock, the hooks, the worktree tool, the formatter's table pass, `mutate` — replaced TypeScript scripts or `mkdir` locks.
- On 2026-10-16 the Product Owner decided that from then on a new script, hook or agent tool is written in Rust, so that no new TypeScript tool has to be rewritten later ([E17](../backlog/epics/E17-fast-tooling-with-oxc-and-rust.md)). The rule was applied by the skills and the worker brief, but no ADR recorded it.

## Decision

1. **Every new script, hook or agent tool is written in Rust**, never in TypeScript: a crate of `tools/` in the root Cargo workspace, run by `cargo run --release -q -p <crate>`.
2. **Its checks**: `[lints] workspace = true` in its `Cargo.toml`, checked by `check:rust` (rustfmt, clippy, advisories, coverage, a Windows build), its tests in the crate.
3. **Its dependencies**: no crates.io crate other than the reviewed ones of `[workspace.dependencies]`, each at an exact version with its row in `docs/tooling/dependencies.md` (Crates), its release notes read, at least 7 days old.
4. **The existing TypeScript tools** may still be edited until F21 rewrites them in Rust.
5. **The library stays TypeScript** (ADR-0025): the rule covers the tools around it, not `src/` nor the playground.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Every new tool in Rust (chosen) | fast tools for the agent (ADR-0069); no new TypeScript tool to rewrite; Oxc's crates usable directly | a second language in the repository; a first build of each crate |
| TypeScript tools, as ADR-0050 decided | one language; no compile step | each tool starts Node.js; every new one would be rewritten in F21 |
| Rust only where a tool is measured slow | less rewriting | the measure comes after the tool is written, then rewritten |

## Consequences

- `AGENTS.md`, the story skill and the run skill's worker brief state the rule and cite this ADR.
- F21 rewrites the remaining TypeScript tools and hooks; E17 extends the direction to everything around the library (Product Owner, 2026-10-17, [E17](../backlog/epics/E17-fast-tooling-with-oxc-and-rust.md)).
- ADR-0050's decisions 4 and 5 (merged branches deleted by GitHub, the state of the session in each call's record) stand; its decisions 2 and 3 are superseded by ADR-0072.

## References

ADR-0025, ADR-0050, ADR-0069, ADR-0072, [E17](../backlog/epics/E17-fast-tooling-with-oxc-and-rust.md), `docs/tooling/agent-tools.md`, `docs/tooling/dependencies.md`
