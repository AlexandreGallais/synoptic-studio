---
id: EN-116
epic: E17
feature: F22
title: The CI's tests green on GitHub's runner
status: ready
points: 2
---

# E17 · F22 · EN-116 — The CI's tests green on GitHub's runner

The CI of `develop` → `main` (#259) failed 6 of 6 runs on three tests that pass on the Mac (RET-004, §1): `[F19.AC6]` expects oxlint's plain output, which prints GitHub annotations when `GITHUB_ACTIONS` is set; `[F19.AC9]` parses `cargo metadata`'s output, empty after a crate download failed; `[F02.AC1]`, the guided test's walk, goes over Vitest's 5 s on the runner.

## Acceptance criteria

- Given the CI of a pull request into `main`, when it runs, then `[F19.AC6]`, `[F19.AC9]` and `[F02.AC1]` pass on GitHub's runner and on the Mac, each still failing when the fault it guards is planted.
- Given `cargo metadata` failing, when `[F19.AC9]` runs, then it fails with Cargo's message, not a JSON parse error.

## Tasks

- [x] T1 — The three tests: the cause of each read from the CI's log, the test made to hold on both machines, each broken once on purpose (2 h)

One task = one commit, referenced as `EN-116.T1` in the commit footer.

## Causes

Read from the logs of the runs 39098890101 and 39108964935 of #259 (`gh run view <run> --log-failed`); the four older failed runs of that day have no log left. Each run failed `test` and `check:rust`.

| Test | Seen on the runner | Cause | Change |
| --- | --- | --- | --- |
| `[F19.AC6]` | `::error file=planted.ts,line=2,…::planted.ts:2:9: Type 'string' …` instead of `planted.ts:2:9: error typescript(TS2322): …` | oxlint without `--format` picks its format from the environment: `github` under `GITHUB_ACTIONS` (`REF-OXLINT-OUTPUT-FORMATS`), `agent` under an agent's variables (`CLAUDECODE` or `AI_AGENT`, tried with `env -i`), `default` elsewhere. The expected line is the `agent` format, so the test passed only in an agent's session: in a plain terminal on the Mac it failed too, on the default format's box | `--format agent` named: the same line under the three environments, tried with `env -i` |
| `[F19.AC9]` | `SyntaxError: Unexpected end of JSON input` | `cargo metadata --offline` reads every crate of `Cargo.lock` for every target; the runner's registry, a cache restored by prefix from an older `Cargo.lock`, then filled by the build with the host's crates only, lacked `android_system_properties` and `sqlite-wasm-rs`: Cargo exited 101 with an empty stdout. The same cause failed `check:rust`, whose `cargo deny --offline` runs `cargo metadata` too. Reproduced here with a `CARGO_HOME` filled by `cargo fetch --target aarch64-apple-darwin`: the same error; after `cargo fetch --locked`, `cargo metadata --offline` passes | the CI fetches every crate (`cargo fetch --locked`, `REF-CARGO-FETCH`) before `check:all`; the test fails on Cargo's exit status with its message |
| `[F02.AC1]` | `Test timed out in 5000ms` after 5,183 and 5,815 ms | the 72 steps drawn in happy-dom take 0.5 s alone, 0.7 s with coverage, over ten times that on the runner's loaded `check:all` | `timeout: 20_000`, as the other long playground tests |
| `[F18.AC19]` (`parse-font-metrics.test.ts`) | over 5 s once in a loaded local `check:all` | the same cause: Inter's tables read for every measured block, 0.8 s alone, 1.6 s with coverage | `timeout: 20_000`, as the other tests reading Inter |

Each broken once on purpose: `[F19.AC6]` with the planted value a number, no type error left; `[F19.AC9]` with smallvec 1.6.1 planted, which has no advisory; `[F02.AC1]` with one step's expected radius changed; `[F18.AC19]` read for other characters. Each failed, then passed restored. `[F19.AC9]` run with an empty `CARGO_HOME` fails with `cargo metadata failed (exit 101): error: no matching package named …`, Cargo's message. `[F19.AC6]` passes under `GITHUB_ACTIONS=true` and without the agent's variables, where its former version fails.

## For the main thread

- The `done` commit's footer: `Rule-docs: none affected, tests, the CI workflow and tooling docs only`.
- `release.yml`'s `rust-cache` job saves a registry filled by the build, so with the host's crates only: each pull request into `main` fetches the other targets' crates again (a few seconds). A `cargo fetch --locked` there would save them in the cache; left out, `release.yml` being F22's other stories' file.
