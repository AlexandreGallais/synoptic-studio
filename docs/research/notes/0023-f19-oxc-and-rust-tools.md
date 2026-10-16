# 0023 — Sources of F19: Oxc and Rust tools

Spike SP-021, 2026-10-16, for F19 (E17). VoidZero's site and Oxc's guides were read first, through the local library (`REF-VOIDZERO` to `REF-OXC-CRATE`); the Rust pages after them. Every tool was then tried on a copy of the repository, never in it: oxlint 1.87.0, oxfmt 0.72.0, `@oxlint/migrate` 1.87.0 and `oxlint-tsgolint` 7.0.2003, the latest versions at least 7 days old on 2026-10-16, installed in the spike's scratch folder, nothing added to `package.json`. Machine: Apple M1 Pro, 10 cores, 16 GB.

## The toolchain

| Tool              | What it brings here                                                                        | Source                                                      |
| ----------------- | ------------------------------------------------------------------------------------------ | ----------------------------------------------------------- |
| oxlint            | the linter, 871 rules built in, ESLint's plugin API for the rest                           | `REF-OXLINT`, `REF-OXLINT-PLUGINS`, `REF-OXLINT-JS-PLUGINS` |
| `oxlint-tsgolint` | the type-aware rules and the type check, on typescript-go                                  | `REF-OXLINT-TYPE-AWARE`, `REF-TS-NATIVE-PORT`               |
| oxfmt             | the formatter, Markdown included, natively in Rust                                         | `REF-OXFMT`, `REF-OXFMT-LANGUAGES`, `REF-OXFMT-MIGRATE`     |
| the `oxc` crate   | a parser and a semantic model for rules and tools written in Rust                          | `REF-OXC-CRATE`                                             |
| the transformer   | not needed: the library targets the latest ECMAScript, and Vite 8 already strips the types | `REF-OXC-TRANSFORMER`, `REF-VOIDZERO`                       |
| the resolver      | only if a Rust tool resolves imports                                                       | `REF-OXC-RESOLVER`                                          |
| the minifier      | not needed: Rolldown, inside Vite 8, already uses it                                       | `REF-OXC-MINIFIER`, `REF-VOIDZERO`                          |

## The gathered notes

Four notes were gathered on 2026-10-16 outside the repository: a study of how to make agents cheaper, the research and the facts of RET-003, and the Product Owner's input for this epic. Each claim F19 needs was read again in its source or measured again; the claims of other features are left to their spikes.

| Claim                                                                                             | Verdict                                                                                                                           | Source or measure                       |
| ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| oxlint is "50 to 100 times faster than ESLint"                                                    | kept, as Oxc's own benchmark; measured here: 0.1 s for its native rules on the whole repository, 66.6 s for ESLint in `check:all` | `REF-OXLINT`, measures below            |
| oxlint has "more than 870 rules", with TypeScript, Unicorn, Import and JSDoc plugins              | kept                                                                                                                              | `REF-OXLINT`, `REF-OXLINT-PLUGINS`      |
| ESLint plugins run in oxlint unchanged, "most existing ESLint plugins should work out of the box" | kept with a limit: alpha, and no rule that needs types; 6 of the 18 functional rules fail here                                    | `REF-OXLINT-JS-PLUGINS`, measures below |
| the project's own rules (`eslint/plugin`) may run as JS plugins                                   | confirmed: all 10 run                                                                                                             | measures below                          |
| oxlint reads no Markdown                                                                          | confirmed                                                                                                                         | `REF-OXLINT`                            |
| `tsgolint` carries 59 of 61 type-aware rules, and `--type-check` can replace `tsc --noEmit`       | kept                                                                                                                              | `REF-OXLINT-TYPE-AWARE`                 |
| oxfmt is about 30 times faster than Prettier and treats differences as bugs                       | kept; measured here: 0.24 s against 8.8 s                                                                                         | `REF-OXFMT`, measures below             |
| typescript-go is about 10 times faster than `tsc`                                                 | kept, as Microsoft's figures                                                                                                      | `REF-TS-NATIVE-PORT`                    |
| `vitest related` runs only the tests that import the changed files                                | kept                                                                                                                              | `REF-VITEST-RELATED`                    |
| ESLint's and Prettier's caches, to speed up the lint and format lanes                             | dropped: both tools leave with F19                                                                                                | —                                       |
| `check:all` at 170 to 217 s, lint and test 170 s each, docs build 107 s (RET-003)                 | measured again below: 123 s on 2026-10-16, test the slowest lane                                                                  | measures below                          |
| the agent ran 2,172 `python3` calls, 162 pushes at 48 s median (RET-003)                          | kept as the project's own figures from `npm run metrics`; not a source                                                            | RET-003                                 |
| Rust 1.99.1 with cargo, clippy and rustfmt, and the `wasm32` target, are installed                | confirmed: `cargo 1.99.1`, `rustc 1.99.1`, targets `aarch64-apple-darwin` and `wasm32-unknown-unknown`                            | measure                                 |
| prompt cache prices, model per agent, context diet, `omitClaudeMd`, repository map                | left to F20's spike                                                                                                               | —                                       |
| subagent limit, heavy lock, fragments, worktrees, flaky tests                                     | read and recorded by SP-020 for F18                                                                                               | note 0022                               |
