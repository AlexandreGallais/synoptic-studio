---
id: EN-121
epic: E17
feature: F33
title: The floors, versions and paths checked
status: done
points: 2
---

# E17 · F33 · EN-121 — The floors, versions and paths checked

From RET-004's "Checks to add" (I37, I54, I55, I56, I58, I60, I81, I99, P8). Moved from F23 on 2026-10-18 (SP-024): its builds and lanes went to F29 (EN-129, EN-130, EN-131), the checks of the docs and the agent pages to EN-140, the `bench` binary to EN-141. Today `rust:coverage` fails under 93 % of lines and functions (`package.json`), and `lint` and `fix` pass `--type-check` to oxlint on their command line.

## Acceptance criteria

- Given `rust:coverage`, when the workspace's coverage falls under 95 % of lines or of functions, then it fails, its message naming the floor; `.claude/rules/tooling.md` and `docs/tooling/commands.md` say 95 %.
- Given `.oxlintrc.json`, when `tests/oxlint-config.test.ts` reads it, then `typeCheck` is set there, and no script passes `--type-check` beside it.
- Given the cargo tools' versions (`CARGO_LLVM_COV_VERSION`, `CARGO_DENY_VERSION` in the workflows, the commands of `docs/tooling/commands.md`), when `deps:outdated` runs, then each is checked against crates.io and `docs/tooling/dependencies.md` as a crate is, a newer release 7 days old reported.
- Given `.vscode/settings.json`, when a test reads it, then its TypeScript path names the installed TypeScript 7 folder, which exists.

## Tasks

- [x] T1 — Each criterion's test, failing (1 h)
- [x] T2 — The floor, the configuration, the versions' check and the path, the docs (2 h)

One task = one commit, referenced as `EN-121.T1` in the commit footer.

## For the main thread

- The floor holds on today's code: `cargo llvm-cov --workspace --summary-only` on 2026-10-19, before the story, gave 95.30 % of lines and 96.47 % of functions (TOTAL row), so 95 % of both, as RET-004 set it.
- `.claude/rules/tooling.md` is left to the main thread, as an instruction file a worker does not change on an agent's request; the first criterion asks it to say 95 %. Its bullet "Every new script, hook or agent tool is a Rust crate of `tools/`" to read "… and `rust:coverage` (95 % of lines and functions covered). …"; and its bullet "A new dev package needs its row …" to end "… a GitHub Action, a Node.js bump, a crate, `Cargo.lock`'s included, and a cargo tool of the Rust checks are checked by `npm run deps:outdated` too; a cargo tool's version changes in every place it is written at once, its row included (EN-121)." Then, in `tests/check-targets.test.ts`, the test `[F33.AC1] gives the floor of 95 % in the pages that name it` takes back the rules: its title "… in the rules and the pages that name it", and `expect(readFileSync(join(ROOT, ".claude/rules/tooling.md"), "utf8")).toContain("`rust:coverage` (95 % of lines and functions covered)");` before its other assertions.
- ADR-0075 (accepted) says `.vscode/settings.json` no longer points VS Code at `node_modules/typescript/lib`, which holds no `tsserver.js`. The story now sets the TypeScript 7 extension's own settings, `js/ts.experimental.useTsgo` and `js/ts.tsdk.path` to `./node_modules/typescript` (`REF-TS7-VSCODE`), and recommends `TypeScriptTeam.native-preview`: a different setting for the reason the ADR gives, so the ADR is left as it is.
- Rule-docs: none affected, a tooling story that changes no rule of `docs/domain/`.
