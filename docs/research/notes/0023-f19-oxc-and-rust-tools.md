# 0023 — Sources of F19: Oxc and Rust tools

Spike SP-021, 2026-10-16, for F19 (E17). VoidZero's site and Oxc's guides were read first, through the local library (`REF-VOIDZERO` to `REF-OXC-CRATE`); the Rust pages after them. Every tool was then tried on a copy of the repository, never in it: oxlint 1.87.0, oxfmt 0.72.0, `@oxlint/migrate` 1.87.0 and `oxlint-tsgolint` 7.0.2003, the latest versions at least 7 days old on 2026-10-16, installed in the spike's scratch folder, nothing added to `package.json`. Machine: Apple M1 Pro, 10 cores, 16 GB.

## The toolchain

| Tool | What it brings here | Source |
| --- | --- | --- |
| oxlint | the linter, 871 rules built in, ESLint's plugin API for the rest | `REF-OXLINT`, `REF-OXLINT-PLUGINS`, `REF-OXLINT-JS-PLUGINS` |
| `oxlint-tsgolint` | the type-aware rules and the type check, on typescript-go | `REF-OXLINT-TYPE-AWARE`, `REF-TS-NATIVE-PORT` |
| oxfmt | the formatter, Markdown included, natively in Rust | `REF-OXFMT`, `REF-OXFMT-LANGUAGES`, `REF-OXFMT-MIGRATE` |
| the `oxc` crate | a parser and a semantic model for rules and tools written in Rust | `REF-OXC-CRATE` |
| the transformer | not needed: the library targets the latest ECMAScript, and Vite 8 already strips the types | `REF-OXC-TRANSFORMER`, `REF-VOIDZERO` |
| the resolver | only if a Rust tool resolves imports | `REF-OXC-RESOLVER` |
| the minifier | not needed: Rolldown, inside Vite 8, already uses it | `REF-OXC-MINIFIER`, `REF-VOIDZERO` |

## The gathered notes

Four notes were gathered on 2026-10-16 outside the repository: a study of how to make agents cheaper, the research and the facts of RET-003, and the Product Owner's input for this epic. Each claim F19 needs was read again in its source or measured again; the claims of other features are left to their spikes.

| Claim | Verdict | Source or measure |
| --- | --- | --- |
| oxlint is "50 to 100 times faster than ESLint" | kept, as Oxc's own benchmark; measured here: 0.1 s for its native rules on the whole repository, 66.6 s for ESLint in `check:all` | `REF-OXLINT`, measures below |
| oxlint has "more than 870 rules", with TypeScript, Unicorn, Import and JSDoc plugins | kept | `REF-OXLINT`, `REF-OXLINT-PLUGINS` |
| ESLint plugins run in oxlint unchanged, "most existing ESLint plugins should work out of the box" | kept with a limit: alpha, and no rule that needs types; 6 of the 18 functional rules fail here | `REF-OXLINT-JS-PLUGINS`, measures below |
| the project's own rules (`eslint/plugin`) may run as JS plugins | confirmed: all 10 run | measures below |
| oxlint reads no Markdown | confirmed | `REF-OXLINT` |
| `tsgolint` carries 59 of 61 type-aware rules, and `--type-check` can replace `tsc --noEmit` | kept | `REF-OXLINT-TYPE-AWARE` |
| oxfmt is about 30 times faster than Prettier and treats differences as bugs | kept; measured here: 0.24 s against 8.8 s | `REF-OXFMT`, measures below |
| typescript-go is about 10 times faster than `tsc` | kept, as Microsoft's figures | `REF-TS-NATIVE-PORT` |
| `vitest related` runs only the tests that import the changed files | kept | `REF-VITEST-RELATED` |
| ESLint's and Prettier's caches, to speed up the lint and format lanes | dropped: both tools leave with F19 | — |
| `check:all` at 170 to 217 s, lint and test 170 s each, docs build 107 s (RET-003) | measured again below: 123 s on 2026-10-16, test the slowest lane | measures below |
| the agent ran 2,172 `python3` calls, 162 pushes at 48 s median (RET-003) | kept as the project's own figures from `npm run metrics`; not a source | RET-003 |
| Rust 1.99.1 with cargo, clippy and rustfmt, and the `wasm32` target, are installed | confirmed: `cargo 1.99.1`, `rustc 1.99.1`, targets `aarch64-apple-darwin` and `wasm32-unknown-unknown` | measure |
| prompt cache prices, model per agent, context diet, `omitClaudeMd`, repository map | left to F20's spike | — |
| subagent limit, heavy lock, fragments, worktrees, flaky tests | read and recorded by SP-020 for F18 | note 0022 |

## Measures

### `check:all`, lane by lane

Each check run alone, then `check:all`, under the heavy lock, on 2026-10-16 from 22:56; the machine's load average stayed between 9 and 20 from the other agents' light work, so the figures are those of a working day, not of an idle machine.

| Check | Alone | In `check:all` | With Oxc (tried on the copy) |
| --- | --- | --- | --- |
| `test` (477 files, 3,269 tests, coverage) | 113.2 s; setup 36 %, import 28 %, tests 22 % | 123.2 s, the slowest lane | unchanged: Vitest stays |
| `lint` (ESLint) | 45.6 s | 66.6 s | oxlint: 0.1 s native rules, 1.6 s with the JS plugins, 2.8 s with type-aware rules and the type check |
| `docs:build` then `playground:build` | 29.1 s, then 0.6 s (load 7, after the other runs) | failed on the spike's own link, written before its note | unchanged |
| `format:check` (Prettier) | 8.8 s | 18.7 s | oxfmt: 0.24 s for the same 2,039 files |
| `typecheck` (`tsc --noEmit`) | 5.1 s | 7.4 s | folded into oxlint's `--type-check`, no error found, as `tsc` |
| `deps:outdated`, `deps:tools`, `lint:secrets`, `deps:unused`, `deps:audit` | 0.7 to 2.3 s each | 1.1 to 2.6 s each | unchanged |
| **`check:all`** | — | **123.4 s**, bound by `test` | bound by `test` and the docs build |

So F19 alone, with oxlint and oxfmt, removes the lint, format and type-check lanes from the critical path (about 93 s of work running beside the tests) but does not shorten `check:all` below its test lane: about 113 s on this machine, of which 64 % is the setup and import of each test file, not the tests.

### The ESLint rules on oxlint

ESLint applies 760 rules to a library module and 21 to a Markdown page (`eslint --print-config`). `@oxlint/migrate --type-aware --details` was run on the configuration of `eslint/`, loaded through `jiti` since `eslint.config.ts` imports a folder.

| Rules | Count | Carried how | Work left |
| --- | --- | --- | --- |
| built into oxlint, options kept | 432 | native, by `@oxlint/migrate` | one option dropped (`jsdoc/require-param`, `enableRestElementFixer`); 110 findings ESLint does not report, in 8 rules, from other defaults (`unicorn/prefer-number-properties` checks `NaN`, which the ESLint plugin skips by default; `id-length`, `preserve-caught-error`, `unicorn/no-useless-undefined`, `jsdoc/require-param` and `require-returns` on two files, `unicorn/prefer-global-this`, `unicorn/no-array-callback-reference`): an option or a decision per rule |
| in oxlint, unstable ("nursery") | 8 | native once stable | wait, or keep as JS plugins meanwhile |
| JSDoc, Unicorn, Import and typescript-eslint rules not ported yet | 246 of 249 | JS plugins aliased (`jsdoc-js`, `unicorn-js`, `import-x-js`, `ts-js`): each tried alone, all 246 run | the plugins stay as dev packages until oxlint ports them |
| typescript-eslint rules needing types, not in `tsgolint` | 3 | none: `naming-convention`, `prefer-destructuring`, `no-unsafe-enum-assignment` fail as JS plugins | rewrite without types, or drop |
| the project's own rules (`local/*`) | 10 | JS plugin, all 10 run in 0.8 s; their findings differ only where a scope of `eslint/scopes` relaxes them | the scopes translated to `overrides` |
| `unused-imports`, `@stylistic` | 7 | JS plugins, all run | none |
| `eslint-plugin-functional` | 12 of 18 | JS plugin | — |
| `eslint-plugin-functional`, failing | 6 | none: `immutable-data`, `prefer-immutable-types`, `type-declaration-immutability`, `no-mixed-types` and `no-conditional-statements` need types; `no-return-void` breaks on oxlint's tree (`returnType` is `null`) | rewrite, or drop: the five that need types cannot be rewritten on Oxc, which has no type checker |
| ESLint core rules missing | 4 | none: `no-restricted-syntax` (7 selectors: no `enum`, no `for…in`, no getters or setters, no default or optional parameter, ADR-0016), `require-atomic-updates`, `consistent-this`, `strict` | `no-restricted-syntax` rewritten as a project rule (the JS plugin API has selectors); `strict` is moot in modules; the other two rewritten or dropped |
| covered by another rule or by TypeScript, or refused by Oxc | 32 | replaced: `import-x/order` by oxfmt's import sorting, `no-invalid-this` by `noImplicitThis`, 22 Unicorn rules by `oxc/*` or type checking | check each replacement once |
| Markdown (`@eslint/markdown`) | 21 | none: oxlint reads no Markdown | rewritten in Rust (a Markdown parser crate), or another tool |

### oxfmt against Prettier

`oxfmt --migrate=prettier` translated `.prettierrc.json` and `.prettierignore`; `experimentalTernaries` is not supported (it is `false` here), and `printWidth` must be set to 100 by hand, since Prettier reads it from `.editorconfig`.

- On the 2,039 files of the format targets, Markdown included, oxfmt differs from Prettier on **3 files**: a union type after `=` that Prettier keeps on one line and oxfmt breaks one member per line (`TextCase`, `LeftOutReason`, `MessageIds`). No Markdown file differs: tables, front matter and lists come out the same.
- With `proseWrap: "never"` (EN-048, ADR-0065), Prettier rewrites 291 Markdown files; oxfmt with the same option accepts all 291 as formatted.
- Time: 0.24 s against 8.8 s for Prettier alone.

### Hooks and agent tools

Mean wall time of one call, 20 to 30 calls each, on the copy:

| Program | Time |
| --- | --- |
| a Rust binary reading the hook's input (release build, 434 KB) | 3.0 ms |
| `node -e ''`, Node.js starting alone | 32.3 ms |
| `guard-bash.ts`, run before every Bash call and waited for | 59.9 ms |
| `track-tool-call.ts`, before every call, not waited for | 94.2 ms |
| `status-line.ts`, every second | 135.7 ms |
| `backlog-status.ts`, `session-report.ts`, `activity.ts`, one call each | 0.11 s, 0.15 s, 0.28 s |

A hook or tool in Rust saves 30 to 130 ms per call, mostly Node.js's start: `guard-bash` runs before every Bash call and is waited for, so 10,000 Bash calls cost 10 minutes of waiting; the status line takes about one core-second in seven. No token is saved by the language: an agent pays in tokens for what a tool prints, not for how it runs; a token saving comes from shorter outputs and a context tool (F20).

### A rule in Rust on Oxc

A proof of concept on the `oxc` crate 0.153.0 parsed every TypeScript file of the repository (1,533), built each semantic model and counted each module's exports, the core of `local/one-export-per-file`: 0.06 s for all, on one thread. First build 25 s with the crate's dependencies, then 2 s; `target/` 274 MB. Clippy found nothing in it. Its 19 findings on `src/` are a type and a value sharing a name, which the real rule allows: a rewrite must carry each rule's exceptions, test by test.

## Answers to the spike's questions

| Question | Answer | Sources |
| --- | --- | --- |
| Which ESLint rules does oxlint carry, which must be rewritten? | 432 natively, 8 once stable, 246 as JS plugins of the current packages, the 10 project rules and 19 plugin rules as JS plugins; to rewrite: `no-restricted-syntax` and 2 core rules, the 21 Markdown rules, `functional/no-return-void`; 8 rules need types that Oxc has not (5 functional, 3 typescript-eslint): rewritten without types, or dropped (Product Owner) | `REF-OXLINT-MIGRATE`, `REF-OXLINT-JS-PLUGINS`, measures |
| Can the configurations be kept? | translated, not kept: `@oxlint/migrate` writes `.oxlintrc.json` from `eslint/` (one option dropped, 110 findings to decide, scopes by hand), `oxfmt --migrate` writes `.oxfmtrc.json` (`printWidth` by hand); the ESLint configuration's tests (`eslint/config.test.ts`) move to the new files | `REF-OXLINT-MIGRATE`, `REF-OXFMT-MIGRATE` |
| Does oxfmt format as Prettier does here? | yes on Markdown, tables and front matter included, with `proseWrap` `preserve` or `never`; 3 TypeScript files differ (a long union type) | `REF-OXFMT`, `REF-OXFMT-MD-DIVERGENCES`, measures |
| Which agent tools first in Rust, and what each saves? | the hooks first: `guard-bash` is waited for on every Bash call (60 ms to 3 ms), the status line runs every second (136 ms), `track-tool-call` and `log-tool-call` on every call (94 ms); then the 26 scripts, 5,855 lines with the hooks; each saves tens of milliseconds per call and no token by itself | measures |
| How fast can `check:all` get? | bound by the test lane, about 113 s; F19 removes lint, format and type check from the critical path and frees about 93 s of work beside the tests; going further is in the test lane (64 % setup and import) and in running the docs build only when the docs changed | measures, `REF-VITEST-RELATED` |
| How is Rust built, tested and checked here? | one Cargo workspace, below | `REF-CARGO-WORKSPACES`, `REF-CLIPPY-USAGE`, `REF-RUSTUP-TOOLCHAIN` |

## The Rust layout

Since 2026-10-16 every new script or tool is written in Rust, and the existing TypeScript tools are rewritten in F19; EN-047 builds the first crate, `tools/heavy-lock` (standard library only, its own `Cargo.lock`, `clippy::pedantic` at `warn`). The lasting layout proposed:

- **One virtual workspace at the root**: a `Cargo.toml` with `[workspace]`, `members = ["tools/*"]`, `resolver = "3"`, and no package; each tool a crate under `tools/` (`tools/heavy-lock`, `tools/guard-bash`…). One `Cargo.lock` at the root, tracked; one `target/` at the root, ignored by Git and by every check tool. `tools/heavy-lock/Cargo.lock` goes, its crate joins the workspace with `[lints] workspace = true`.
- **Shared settings**: `[workspace.package]` (edition 2024, licence Apache-2.0, `publish = false`), `[workspace.lints]` (`clippy::pedantic` as now, `unsafe_code = "forbid"`), `[workspace.dependencies]` with exact versions (`oxc = "=0.153.0"`) so that a crate added is reviewed as a dev package is: its row in `docs/tooling/dependencies.md`, its release notes read, at least 7 days old.
- **Toolchain pinned**: `rust-toolchain.toml` with `channel = "1.99.1"` and `components = ["rustfmt", "clippy"]`; `wasm32` is not needed by any tool found.
- **Warnings are errors** through `.cargo/config.toml` (`[build] warnings = "deny"`), which Clippy recommends over `-D warnings` because it keeps the build cache.
- **A `rust` lane in `check:all`**: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked`, `cargo test --workspace --locked`, with explicit targets as every check tool (EN-045). The pre-push and the CI on the pull request into `main` run it with the rest.
- **The binaries** are built by `cargo build --release` from `prepare` (after `npm install`) and from the lane; hooks and scripts call `target/release/<tool>`. A tool missing its binary fails with a message naming the build command.
- **The library stays dependency-free** (ADR-0025): crates are tools of the repository, never in `dependencies` nor in `dist`; `package.json`'s `files` is unchanged.
- **Not available yet**: no `cargo audit` nor `cargo deny` (a crate's advisories) and no coverage tool (`cargo-llvm-cov`) are installed; the 100 % coverage rule applies to `src/` only, so Rust tools are tested like today's scripts, without a coverage floor, unless the Product Owner wants one.

## The target of `check:all` (to set by the Product Owner)

| Option | `check:all` on this machine | What it takes |
| --- | --- | --- |
| A — F19 only | about 115 to 125 s: the test lane | oxlint, oxfmt, the type check in oxlint |
| B — A, and the docs build only when `docs/`, `src/` or the playground changed | the same on a code story, shorter on a tooling story | a path filter in `check-all.ts` |
| C — B, and the test lane's setup cut | estimated 50 to 70 s, from the 64 % of setup and import (no source gives a figure) | a story on Vitest's environments, imports and coverage: outside F19's open questions, inside E17's scope (the checks' speed) |

## F19's acceptance criteria and stories (draft)

Superseded on 2026-10-17 by the Product Owner's answers below and by [F19's feature file](../../backlog/features/E17-F19-oxc-and-rust-tools.md).

Criteria, to confirm at the refinement:

- Given `npm run lint`, when it runs on the whole repository, then oxlint reports what ESLint reported on 2026-10-16 (no finding on the same tree), every rule of `eslint/` carried, rewritten or dropped by a decision written in an ADR.
- Given `npm run format:check`, when it runs, then oxfmt checks the same targets as Prettier, Markdown included.
- Given `check:all`, when it runs, then it ends under the target set by the Product Owner, lane times printed as now.
- Given a hook or an agent tool, when it is called, then it is a Rust binary of the workspace, its output unchanged.
- Given `package.json`, when F19 ends, then no ESLint, Prettier or plugin package is left, and `dependencies` is still empty.

Stories, in order:

| Story | Type | Title |
| --- | --- | --- |
| EN | Enabler | The Cargo workspace, its toolchain file and the `rust` lane in `check:all`, `heavy-lock` moved in |
| EN | Enabler | oxfmt instead of Prettier: configuration, scripts, lint-staged, the 3 union types |
| EN | Enabler | oxlint with its native and type-aware rules, the type check in it, the scopes as `overrides`, the 110 findings decided |
| EN | Enabler | The project rules and the plugins oxlint lacks as JS plugins, their tests on oxlint's `RuleTester` |
| EN | Enabler | `no-restricted-syntax`, `require-atomic-updates`, `consistent-this` and `functional/no-return-void` rewritten (JS plugin or Rust: Product Owner) |
| EN | Enabler | The 21 Markdown rules in a Rust tool |
| EN | Enabler | The 8 rules that need types: rewritten without types, or dropped (Product Owner) |
| EN | Enabler | The hooks in Rust: `guard-bash`, `track-tool-call` and `log-tool-call`, the status line |
| EN | Enabler | The scripts in Rust, by group: metrics, library, backlog and story tools, `check:all` |
| EN | Enabler | ESLint, Prettier and their plugins removed; ADR on the Oxc toolchain, superseding ADR-0009 (toolchain) and ADR-0013 (Prettier next to ESLint), ADR-0010 (lint first) reread |
| CHK, AUD, VAL | — | as for every feature |

TypeScript stays at 6.0: TypeDoc 0.28.20, the latest, asks for `typescript` up to 6.0 (`npm view typedoc peerDependencies`), even once typescript-eslint has left; `tsgolint` brings its own typescript-go and read this `tsconfig.json` without an error.

## The Product Owner's answers (2026-10-17) and the research they asked

The Product Owner answered the spike's four questions on 2026-10-17 and widened its scope the same day. Each answer below is recorded as understood, with the research it asked for. Measures taken on 2026-10-17 from 08:57, under the heavy lock, on the same machine (load average 7 to 22 from other agents' work); tools tried in the spike's scratch folder, nothing added to `package.json`.

### Decisions

| Question | Decision of 2026-10-17 |
| --- | --- |
| Target of `check:all` | option B: Oxc instead of ESLint and Prettier, the docs build only when the docs change; about 70 to 90 s most of the time. Every tool moves to Oxc and Rust: no ESLint nor Prettier use is left, and every TypeScript script and hook of `scripts/` and `.claude/hooks/` is rewritten in Rust |
| The 8 rules that need types | none is dropped without a stated reason; the types come from an existing tool where one gives them (oxlint's type-aware mode); a rule no tool carries with types is rewritten without types |
| The project's own rules | JavaScript plugins if they are oxlint's usual way and cost no noticeable time; Rust if they are slower or not the usual way |
| Rust checks | a coverage floor for the Rust tools and an advisory check of their crates, both |
| Native TypeScript | asked: can `tsc` and the type-aware lint run on TypeScript 7, the native compiler |
| Scope | the library stays TypeScript, run as JavaScript; everything around it moves to faster technology where it is sound: Rust by default for the project's own scripts, hooks, agent tools and local MCP servers; for third-party tools the fastest maintained option, each with its status, its age, what is lost and its gain |
| Clean-up | at the end of E17, one story removes every TypeScript or JavaScript script, hook, configuration and dev package left unused, checked by knip and a search for their names, last before NAM-002 |
| Platforms | the library's tests (`src/`, `playground/`) never depend on macOS; the tools around it are written with Rust's portable standard calls where one exists, a goal and not a blocker: no three-system CI, at most one cheap check |

### The 8 rules that need types

oxlint gets its types from `tsgolint`, which runs typescript-eslint's type-aware rules on typescript-go: it is stable since 2026-07-22 and carries 60 of the 62 rules it targets (`REF-TSGOLINT`). JS plugins get no type: "Lint rules that rely on TypeScript type-awareness" are not supported (`REF-OXLINT-JS-PLUGINS`), "No custom type-aware rules" (`REF-OXLINT-JS-PLUGINS-ALPHA`), and the plugins' next milestone does not aim at it (`REF-OXLINT-JS-PLUGINS-M3`). `tsgolint` accepts no rule beyond typescript-eslint's, so no existing tool gives types to `eslint-plugin-functional`'s rules on Oxc.

| Rule | Why it needs types | With Oxc | Recommendation |
| --- | --- | --- | --- |
| `@typescript-eslint/prefer-destructuring` | only to tell an iterable from an object on `x = o[0]` (`REF-TSESLINT-PREFER-DESTRUCTURING`) | native `eslint/prefer-destructuring` with the same options, `enforceForDeclarationWithTypeAnnotation` included (`REF-OXLINT-PREFER-DESTRUCTURING`); not in `tsgolint` (PR 692) | carried natively: with `array: false` and `enforceForRenamedProperties: false` the type never changes a finding |
| `@typescript-eslint/naming-convention` | only for its `types` option (`REF-TSESLINT-NAMING-CONVENTION`): here a global `const` holding a function, and boolean prefixes | not in `tsgolint` yet: its pull request is an open draft (`REF-TSGOLINT-NAMING-PR`) | rewritten without types as a project rule, `types` read from the annotation or the initializer; replaced by `tsgolint`'s once released and 7 days old |
| `@typescript-eslint/no-unsafe-enum-assignment` | a number assigned to an enum (`REF-TSESLINT-NO-UNSAFE-ENUM-ASSIGNMENT`) | not among `tsgolint`'s targets | dropped, reason stated: `enum` is forbidden here (`no-restricted-syntax`, kept), so the rule can never report |
| `functional/immutable-data` | to tell a mutable from an immutable value (`REF-FUNCTIONAL-PLUGIN`) | no tool | rewritten without types: any assignment to a member, `delete`, and the mutating array methods by name |
| `functional/prefer-immutable-types` | to prove a parameter's type `ReadonlyDeep` | no tool | rewritten without types: each parameter's annotation written `readonly` or `Readonly<…>`, deep in the written type only |
| `functional/type-declaration-immutability` | to prove a declared type `ReadonlyDeep` | no tool | rewritten without types: every property `readonly`, every array `readonly`, in the declaration itself |
| `functional/no-mixed-types` | to tell a function-typed member from a value | no tool | rewritten without types, from the members' written annotations |
| `functional/no-conditional-statements` | `allowReturningBranches` checks that each branch returns | no tool | rewritten without types: a branch returns or throws, read from the tree |

What a rewrite without types loses: a type written elsewhere (an alias, an imported interface) is trusted by its name, not proven deep. The measured alternative that loses nothing: these 6 rules alone in a small ESLint pass with types took **9.0 s** (3 runs, 8.9 to 9.2 s), no finding on the tree, beside the test lane and off its critical path — but it keeps ESLint, typescript-eslint and the TypeScript 6 API, against the decision that every tool moves to Oxc. Kept as a question for the refinement: a rewrite without types (the decision's default) or that 9 s pass until an Oxc tool gives types.

`tsgolint` 7.0.2004, released 2026-10-10 (7 days old on 2026-10-17), brings only fixes (`no-unsafe-return`, `no-misused-promises`, `no-floating-promises`, `no-deprecated`, disable comments); the spike tried 7.0.2003. oxlint 1.88.0 (2026-10-13) is under 7 days: F19 starts on 1.87.0.

### The project's own rules

JS plugins are oxlint's documented way to write a project's own rules: "write your own custom rules, all within Oxlint", an API identical to ESLint's, a `RuleTester` (`REF-OXLINT-JS-PLUGINS-ALPHA`, `REF-OXLINT-WRITING-JS-PLUGINS`). No documented way exists to add a Rust rule to oxlint from outside the Oxc repository: a rule in Rust is a second linter of our own on the `oxc` crate.

| Run, whole repository (`src`, `scripts`, `eslint`, `playground`, `tests`) | Time, 6 runs |
| --- | --- |
| oxlint, one native rule (the floor: start, walk, parse) | 0.06 to 0.10 s |
| oxlint, the 10 project rules as a JS plugin | 1.00 to 1.17 s |
| oxlint, `local/one-export-per-file` alone as a JS plugin | 0.38 to 0.44 s |
| the Rust proof of concept, the same rule's core, on the `oxc` crate | 0.05 to 0.06 s |
| ESLint in `check:all` (2026-10-16), for scale | 66.6 s |

A rule is about 8 times slower as a JS plugin than in Rust, but the 10 rules together add about 1 s to a lint lane that runs beside the 113 s test lane: not noticeable in `check:all`, and less on the few files of a commit. **Decision that follows**: the project's rules stay JS plugins, written in the plugin API's faster form (`createOnce`, `REF-OXLINT-WRITING-JS-PLUGINS`) and tested on oxlint's `RuleTester`. Three core rules oxlint lacks (`no-restricted-syntax`, `require-atomic-updates`, `consistent-this`) need no rewrite: `oxlint-plugin-eslint` runs ESLint's own (`REF-OXLINT-JS-PLUGINS-ALPHA`; 1.87.0, 2026-10-05); `functional/no-return-void` becomes a project rule. Only the 21 Markdown rules go to Rust, since oxlint reads no Markdown (`REF-OXLINT`).

### TypeScript 7, the native compiler

TypeScript 7.0 is the native port, released on 2026-07-08 as `typescript` on npm (`REF-TS7-ANNOUNCEMENT`); `7.0.2` is the latest, 101 days old. It has no API until 7.1; tools that import `typescript` keep 6.0 through the alias `@typescript/typescript6` (6.0.2, 2026-07-06), its binary named `tsc6`. Here only TypeDoc 0.28.20 needs the 6.0 API (peer `typescript` up to 6.0.x); typescript-eslint leaves with F19.

| Check, on this repository | TypeScript 6.0.3 | TypeScript 7.0.2 |
| --- | --- | --- |
| `tsc --noEmit`, 5 runs | 4.10 to 4.32 s | 0.64 to 0.71 s (about 6 times faster) |
| errors on the clean tree | 0 | 0 |
| one type error planted in a copy | TS2322, line 21 | the same TS2322, same place (exit code 1, not 2) |
| declarations of `tsconfig.build.json` (`dist/types`) | 587 files | 587 files, byte for byte the same |

Nothing of `tsconfig.json` is refused by 7.0: `lib` with `esnext.temporal`, `moduleResolution: "bundler"`, `types` listed, `allowImportingTsExtensions`. **Recommendation: yes, now**: `typescript` 7.0.2 for `tsc` (the type check and the declarations of `build`), 6.0 kept under the alias for TypeDoc; oxlint's `--type-check` runs on `tsgolint`'s own typescript-go 7.0.2 and may replace the separate `typecheck` lane. A script reading `tsc`'s exit code must accept 1.

### Rust coverage and advisories

| Need | Tool | Version, age on 2026-10-17 | Time added |
| --- | --- | --- | --- |
| coverage floor | `cargo-llvm-cov`, `--fail-under-lines` and `--fail-under-functions` (`REF-CARGO-LLVM-COV`) | 0.9.1, 41 days | not measured: it needs rustup's `llvm-tools-preview` component, not installed; an instrumented second build of the workspace, to time in its story |
| advisories of the crates | `cargo-deny check advisories` (`REF-CARGO-DENY-ADVISORIES`) | 0.20.2, 100 days | 0.46 to 0.56 s offline on the 84 crates of the `oxc` proof of concept, 1.2 s with the database fetched; built in 74 s by `cargo install` |

cargo-deny is preferred to `cargo audit` (0.22.2, `REF-CARGO-AUDIT`): the same RustSec database, plus licences, duplicate versions and sources in one tool, as `docs/tooling/dependencies.md` asks of a dev package. The pre-push runs it `--offline`; the CI on the pull request into `main` fetches the database.

### Everything around the library

| Part | Today | Proposed | Status, age | What is lost | Gain |
| --- | --- | --- | --- | --- | --- |
| lint | ESLint 10 | oxlint 1.87.0 with `tsgolint` 7.0.2004 and JS plugins | stable; JS plugins alpha | type proofs of 5 functional rules (above) | 66.6 s to about 3 s |
| format | Prettier 3.9 | oxfmt 0.72.0 | measured on 2,039 files | 3 union types laid out differently | 8.8 s to 0.24 s |
| type check, declarations | TypeScript 6.0 | TypeScript 7.0.2, 6.0 aliased for TypeDoc | stable since 2026-07-08 | nothing measured | 4.2 s to 0.7 s |
| scripts and hooks (26 scripts, 14 hook files) | TypeScript on Node.js | Rust crates of the workspace | Rust 1.99.1 | nothing: outputs kept, tested | 30 to 130 ms per call (measures above) |
| the `library` MCP server | `scripts/library-mcp.ts`, Node.js, `node:sqlite` with FTS5 | Rust on `rmcp` (`REF-RMCP`), the official SDK, `stdio` transport, a SQLite crate with FTS5 chosen in its story | `rmcp` 3.5.1, 12 days | nothing if the store's format is kept | Node.js's start on each session; not measured |
| playground and library builds | Vite 8.3.4 | nothing to move: Vite 8 already bundles with Rolldown 1.2 (`REF-VOIDZERO`) | — | — | playground built in 0.6 s |
| docs build | TypeDoc 2.8 s, then VitePress 27.4 s | VitePress already runs on Vite 8; the gain is running it only when the docs change (option B) | — | — | 30 s skipped on a story without docs |
| tests | Vitest 5 | kept: VoidZero's runner on Vite (`REF-VOIDZERO`); no faster sound runner found for this suite; its cost is setup and import (64 %), option C | — | — | — |

Not moved, and why: `default-font-metrics.ts` runs the library itself to write `inter-metrics.json`, so it stays TypeScript (a Rust copy would duplicate the font reader); Vitest, Vite, VitePress and TypeDoc (above); secretlint, knip, npm-check-updates, `npm audit`, commitlint, lint-staged and husky, which take 0.7 to 2.6 s each beside the test lane and have no measured need; the husky hooks stay shell scripts calling the Rust binaries.

### Platforms

The library's tests already run on Linux in the CI (`ubuntu-latest`); 10 test files read macOS fonts and skip elsewhere, which EN-062 (Inter as a test font, F18) starts to close. The Rust crates of F18 hold Unix-only parts: `heavy-lock` calls `flock` through `extern "C"` and `std::os::unix`, `mutate` runs `kill` and Unix symlinks, `worktree` and its tests use Unix symlinks, `rule-docs`'s test sets Unix permissions. Rust's standard `File::lock` and `File::try_lock`, stable since **Rust 1.89.0**, map to `flock` on Unix and `LockFileEx` on Windows (`REF-RUST-FILE-LOCK`): `heavy-lock` can drop its `unsafe` block. One cheap check is proposed: `cargo check --workspace --target x86_64-pc-windows-msvc` in the `rust` lane, which compiles without linking, so a Unix-only call fails on the Mac; its cost, one rustup target installed and one more check pass, is to measure in its story.

## The Product Owner's second round (2026-10-17)

Decisions taken after the answers above, recorded as understood; they supersede, where they differ, the recommendations of the section above.

| Subject | Decision |
| --- | --- |
| Split | accepted: F19 holds the checks; the agent tools in Rust, the `library` MCP server and the clean-up of E17 form [F21](../../backlog/features/E17-F21-agent-tools-in-rust.md) |
| The 6 rules without types | rewritten in their strict form: `readonly` or `Readonly<…>` written in place, nothing trusted from a type defined elsewhere; nothing kept in ESLint (the 9 s pass is not adopted) |
| TypeScript | TypeScript 7 only; TypeScript 6 removed, so TypeDoc is replaced |
| Tools | every tool around the project in Rust or Go — the project's own in Rust, third-party ones rewritten natively; no JavaScript tool of the project's own; Vite, Vitest and VitePress stay |
| Project rules | in Rust, in a linter of the project on Oxc's crates — this supersedes the JS-plugin decision above |
| F20 | three stories added: a push notification at each stop, a progress page as a claude.ai Artifact, a `SubagentStop` hook in Rust |
| Agent setup | cloud agents are not used in this project; scheduled tasks are used only when the Product Owner asks (`docs/tooling/agent.md`) |
| Product | the library on GitHub only, never on npm (Q40); no accessibility layer nor other language in V1, maybe in the pro edition (Q41); font subsetting at export wanted later, for fixed texts only (Q42) |

### TypeDoc without TypeScript 6

TypeDoc writes the API part of the documentation site: 536 Markdown pages in `docs/api/` (396 functions, 101 type aliases, 39 constants), their index and `typedoc-sidebar.json`, which `docs/.vitepress/config.ts` turns into the API sidebar; the folder is generated, never tracked. Its configuration also checks the library: a warning is an error, and an export not documented, a link not resolved or an export not reachable fails the build.

| Option | Status on 2026-10-17 | Verdict |
| --- | --- | --- |
| TypeDoc on TypeScript 7 | its maintainer works on it in spare time, aims at TypeScript 7.1's release candidate and gives no timeline (`REF-TYPEDOC-TS7`); 7.1's release candidate is planned for 2026-11-10 (`REF-TS71-PLAN`) | not before mid-November at best, and still a JavaScript tool |
| `deno_doc`, a Rust crate | "generate documentation for JavaScript and TypeScript modules", not Deno-specific (`REF-DENO-DOC`); 0.209.0 of 2026-10-10; built on `deno_ast` (another parser than Oxc's) and `deno_graph`, writes HTML or JSON, not our Markdown pages | possible, but a second parser stack and a conversion of its output to the pages VitePress reads |
| A Rust generator of our own on Oxc | the library has one export per file with an austere TSDoc block (`@kind`, `@see`, parameters, returns); each page is a signature read from the written annotations, a description and fixed sections; links are exported names | **recommended**: the same crates as the project's linter, the three checks kept, the same pages and sidebar |

No Go tool generating API documentation for TypeScript was found.

### TypeScript 7 alone with Vite, Vitest and VitePress

Tried on 2026-10-17 in a copy of the branch (`git archive`) with `typescript` 7.0.2 as the only TypeScript and TypeDoc, typescript-eslint and its packages removed (631 packages installed in 20 s, none of TypeScript 6):

| Tool | Result |
| --- | --- |
| `vite build --mode playground`, `vite build` | built, 0.7 s and 0.6 s |
| `vitepress build docs` | built in 8.5 s (no API pages, TypeDoc being gone) |
| `vitest run`, 438 files | 3,019 of 3,020 tests passed in 49 s; the 4 files that failed read ESLint's configuration (which loads TypeScript's API), call Git (none in the copy), or list the removed packages — none because of TypeScript 7 |
| `tsc --noEmit` | 0.9 s, one error: `eslint/scopes/language.ts` imports `typescript-eslint`, which leaves with ESLint |

Vite, Vitest and VitePress transform TypeScript themselves and run with TypeScript 7 alone.

### The project's linter in Rust

The project's rules, the three core rules oxlint lacks, `no-return-void`, the 6 rules without types and the 21 Markdown rules go into one Rust linter of the workspace, on Oxc's crates `oxc_parser`, `oxc_ast`, `oxc_semantic` and `oxc_allocator` (dependencies from crates.io, each with its row in `docs/tooling/dependencies.md`): 0.153.0 of 2026-10-05, since 0.154.0 (2026-10-13) is under 7 days old. Measured cost, from the proof of concept on the same crates: one rule's core over the 1,533 TypeScript files in **0.05 to 0.06 s** (against 0.40 s as a JS plugin), a first build of 25 s and then 2 s, a `target/` of 274 MB. The Markdown rules need a Markdown parser crate, chosen in their story.

Left open by the decision that every tool is Rust or Go, a question for the refinement: the 265 rules that oxlint runs only as JS plugins of third-party packages (246 of JSDoc, Unicorn, Import and typescript-eslint not yet ported, 7 of `unused-imports` and `@stylistic`, 12 of `eslint-plugin-functional`) — each rewritten in the Rust linter, dropped with its reason, or waiting for oxlint to port it.

## The Product Owner's third round (2026-10-17)

- **One linter, oxlint.** `tsgolint` is used only if a rule kept truly needs types. It does: the configuration enables 120 typescript-eslint rules, of which **55** are type-aware rules that `tsgolint` runs — among them `no-floating-promises`, `no-misused-promises`, `strict-boolean-expressions`, `no-unnecessary-condition`, `switch-exhaustiveness-check`, `restrict-template-expressions` and the eight `no-unsafe-*` rules (counted on 2026-10-17 from the configuration ESLint prints and `tsgolint`'s rule folder, `REF-TSGOLINT`). Each reads a type the code does not write (a promise returned, a value possibly `any`), so none can be rewritten without a type checker. So `tsgolint` stays, for those 55 only; the 8 rules of the spike's question are handled without types as decided. The type check itself is TypeScript 7's compiler, not oxlint's `--type-check`.
- **Every rule oxlint lacks is rewritten in the project's Rust linter**: the 265 rules it runs today only as JS plugins of third-party packages — Unicorn 197, JSDoc 43, `eslint-plugin-functional` 12, `@stylistic` 5, Import 4, typescript-eslint without types 2, `unused-imports` 2 (counted from the spike's runs). The question of the second round is closed; how to cut this work into features is left to the refinement.
- **The dev packages**: each is rewritten in Rust when sensible, kept otherwise with its reason; the table and the stories are in [F21](../../backlog/features/E17-F21-agent-tools-in-rust.md). Rewritten: husky, lint-staged, commitlint, validate-branch-name, secretlint, npm-check-updates, knip. Kept: Vite, Vitest and its coverage, VitePress, TypeScript 7, oxlint, oxfmt and `tsgolint`, `happy-dom`, `fast-check`, `@types/node`, `mermaid`.

## The Product Owner's fourth round (2026-10-17)

- **`tsgolint` stays**, for the 55 rules that need types only; oxlint stays the one lint command.
- **Every rule oxlint lacks is rewritten in F19**, however long, and ESLint leaves at F19's end; a triage comes first, below, confirmed by F19's oxlint story. F19's rewrite stories are sized from the 232 rules kept: Unicorn in five stories of 29 to 45 rules, JSDoc in one (31), the other plugins in one (20).

### Triage of the rules oxlint lacks

Asked by the Product Owner on 2026-10-17: for each of the 265 rules oxlint runs today only as a JS plugin, what it guards; dropped with its reason when a project rule already forbids what it checks or when it guards nothing this code can contain; kept and rewritten in the Rust linter otherwise. Done from the configuration ESLint prints, each rule's own description (`meta.docs.description` of the installed plugins) and a search of `src/`, `playground/`, `tests/` and the root configuration files — what remains TypeScript once F21 moves the tools to Rust. F19's oxlint story confirms it rule by rule against the code.

| Plugin | Rules | Kept | Dropped |
| --- | --- | --- | --- |
| Unicorn | 197 | 173 | 24 |
| JSDoc | 43 | 31 | 12 |
| functional | 12 | 11 | 1 |
| @stylistic | 5 | 4 | 1 |
| Import | 4 | 2 | 2 |
| typescript-eslint, without types | 2 | 1 | 1 |
| unused-imports | 2 | 2 | 0 |
| **All** | **265** | **224** | **41** |

The 41 dropped, each with its reason (four added by EN-074, the last four by EN-078):

| Rule | Reason |
| --- | --- |
| `@typescript-eslint/no-unused-private-class-members` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `jsdoc/imports-as-dependencies` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/check-types` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/no-undefined-types` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/no-unnecessary-type-assertion` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/prefer-import-tag` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/reject-any-type` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/reject-function-type` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/ts-method-signature-style` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/ts-no-empty-object-type` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/ts-no-unnecessary-template-expression` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/ts-prefer-function-type` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `jsdoc/valid-types` | JSDoc types are forbidden (`jsdoc/no-types`, kept): a rule on the types written in JSDoc has nothing to read |
| `import-x/no-import-module-exports` | `import/no-commonjs` and `unicorn/prefer-module` (native) already forbid `module.exports` |
| `import-x/no-relative-packages` | the repository is one package: there is no other package to import by a relative path |
| `unicorn/consistent-export-decorator-position` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-exports-in-scripts` | only the tools used it (`node:sqlite`, Node.js scripts); they move to Rust in F21 |
| `unicorn/no-top-level-assignment-in-function` | `functional/no-let` forbids reassigning a variable, so no top-level variable can be assigned |
| `unicorn/no-unnecessary-polyfills` | `dependencies` stays empty: no polyfill package can be imported |
| `unicorn/class-reference-in-static-methods` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-incomplete-accessor-override` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-this-outside-of-class` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-useless-override` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/prefer-private-class-fields` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-unsafe-sqlite-interpolation` | only the tools used it (`node:sqlite`, Node.js scripts); they move to Rust in F21 |
| `unicorn/require-text-decoder-streaming` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `unicorn/no-invalid-integrity` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `unicorn/no-invalid-response-options` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `unicorn/no-unnecessary-fetch-options` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `unicorn/prefer-abort-signal-any` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `unicorn/prefer-abort-signal-timeout` | no network code: V1 is local only (Q5), and the code calls no `fetch`, `Response` nor `AbortSignal` |
| `functional/no-class-inheritance` | `functional/no-classes` already forbids every class, inheritance included |
| `@stylistic/lines-between-class-members` | no class can exist: `functional/no-classes` and `no-this-expressions` forbid classes and `this` |
| `unicorn/no-optional-chaining-on-undeclared-variable` | TypeScript refuses a name declared nowhere (ts(2304)), with or without `?.` |
| `unicorn/no-xor-as-exponentiation` | `no-bitwise` (native, kept) forbids `^` |
| `unicorn/no-useless-continue` | `no-continue` (native, kept) forbids every `continue` |
| `unicorn/no-leading-empty-lines` | oxfmt removes the blank lines at a file's start, and `npm run format:check` fails on them |
| `unicorn/no-array-splice` | it only reports `splice()` on a `let` or `var` array, and `functional/no-let` and `no-var` forbid both |
| `unicorn/no-array-concat-in-loop` | it only reports a `let` or `var` accumulator, and `functional/no-let` and `no-var` forbid both |
| `unicorn/prefer-array-from-range` | it reads `Array(n)` and `new Array(n)` only, which `new-for-builtins` and `no-new-array` (native, kept) refuse, as a probe file showed |
| `unicorn/prefer-get-or-insert-computed` | it reads `Map#getOrInsert()` only, which ES2025's library and Node.js 24 lack: `tsc` refuses the call (TS2550, tried on a probe file) |

Confirmed against the code by EN-074 on 2026-10-17: no class, `this`, accessor, `enum`, `for…in`, `let`, CommonJS export nor JSDoc type in `src/`, `playground/`, `tests/` and the root configuration files; `fetch` and `node:sqlite` only in `scripts/`; `dependencies` empty. Each of the four drops above was tried on a probe file. The rewrite stories EN-078, EN-079 and EN-080 lost the four rules (44, 36 and 37 rules left).

The 224 kept, by plugin and, for Unicorn, by the area of `eslint/rules/unicorn/`:

| Group | Kept | Rules |
| --- | --- | --- |
| @stylistic | 4 | `line-comment-position`, `multiline-comment-style`, `spaced-comment`, `padding-line-between-statements` |
| Import | 2 | `no-extraneous-dependencies`, `no-restricted-paths` |
| JSDoc | 31 | `check-param-names`, `check-syntax`, `check-template-names`, `escape-inline-tags`, `informative-docs`, `match-description`, `no-restricted-syntax`, `normalize-see-links`, `check-alignment`, `check-indentation`, `check-line-alignment`, `lines-before-block`, `multiline-blocks`, `no-bad-blocks`, `no-blank-block-descriptions`, `no-multi-asterisks`, `require-asterisk-prefix`, `require-hyphen-before-param-description`, `sort-tags`, `tag-lines`, `require-description`, `require-jsdoc`, `require-next-description`, `require-rejects`, `require-returns-check`, `require-template`, `require-template-description`, `require-throws`, `require-yields-check`, `no-types`, `ts-ban-ts-comment` |
| Unicorn, arrays | 12 | `no-array-from-fill`, `no-array-front-mutation`, `no-confusing-array-splice`, `no-impossible-length-comparison`, `no-negated-array-predicate`, `no-return-array-push`, `no-unnecessary-splice`, `prefer-array-last-methods`, `prefer-array-slice`, `prefer-flat-math-min-max`, `prefer-simple-sort-comparator`, `prefer-single-array-predicate` |
| Unicorn, collections | 12 | `consistent-conditional-object-spread`, `no-duplicate-set-values`, `no-mismatched-map-key`, `no-object-methods-with-collections`, `no-unnecessary-array-flat-map`, `no-useless-set-construction`, `prefer-array-from-map`, `prefer-group-by`, `prefer-has-check`, `prefer-includes-over-repeated-comparisons`, `prefer-map-from-entries`, `prefer-set-methods` |
| Unicorn, control-flow | 17 | `no-declarations-before-early-exit`, `no-duplicate-if-branches`, `no-duplicate-logical-operands`, `no-unnecessary-nested-ternary`, `no-useless-boolean-cast`, `no-useless-else`, `no-useless-logical-operand`, `prefer-boolean-return`, `prefer-combined-guards`, `prefer-early-return`, `prefer-else-if`, `prefer-hoisting-branch-code`, `prefer-minimal-ternary`, `prefer-simple-condition-first`, `prefer-simplified-conditions`, `prefer-smaller-scope`, `prefer-switch` |
| Unicorn, dom | 19 | `better-dom-traversing`, `dom-node-dataset`, `no-canvas-to-image`, `no-incorrect-query-selector`, `no-invalid-dom-token`, `no-invalid-style-set-property`, `no-late-current-target-access`, `no-late-event-control`, `no-prevent-default-in-passive-listener`, `no-selector-as-dom-name`, `no-unsafe-dom-html`, `prefer-add-event-listener-options`, `prefer-dom-node-html-methods`, `prefer-dom-node-replace-children`, `prefer-observer-apis`, `prefer-path2d`, `prefer-scoped-selector`, `prefer-toggle-attribute`, `require-passive-events` |
| Unicorn, errors | 4 | `no-error-property-assignment`, `prefer-aggregate-error`, `prefer-error-is-error`, `try-complexity` |
| Unicorn, functions | 9 | `consistent-function-style`, `isolated-functions`, `no-repeated-call-wrappers`, `no-uncalled-method`, `no-unnecessary-default-arguments`, `no-unnecessary-parameters`, `no-useless-recursion`, `prefer-block-statement-over-iife`, `prefer-short-arrow-method` |
| Unicorn, iteration | 16 | `iteration-fallback-style`, `no-array-sort-for-min-max`, `no-break-in-nested-loop`, `no-duplicate-loops`, `no-for-each`, `no-loop-iterable-mutation`, `no-unreadable-for-of-expression`, `no-unused-iterator-helper`, `prefer-array-iterable-methods`, `prefer-direct-iteration`, `prefer-iterable-in-constructor`, `prefer-iterator-helpers`, `prefer-iterator-to-array`, `prefer-iterator-to-array-at-end`, `prefer-object-iterable-methods`, `prefer-while-loop-condition` |
| Unicorn, misc | 4 | `consistent-optional-chaining`, `no-useless-compound-assignment`, `no-useless-delete-check`, `prefer-https` |
| Unicorn, modules | 8 | `default-export-style`, `no-global-object-property-assignment`, `no-top-level-side-effects`, `no-unnecessary-global-this`, `no-useless-re-export`, `prefer-global-number-constants`, `prefer-identifier-import-export-specifiers`, `prefer-json-import` |
| Unicorn, naming | 7 | `comment-content`, `consistent-compound-words`, `consistent-tuple-labels`, `name-replacements`, `no-keyword-prefix`, `prefer-type-literal-last`, `single-line-block-comment-style` |
| Unicorn, numbers | 6 | `no-negated-comparison`, `no-redundant-comparison`, `no-subtraction-comparison`, `prefer-math-abs`, `prefer-number-is-safe-integer`, `prefer-unary-minus` |
| Unicorn, objects | 12 | `consistent-destructuring`, `no-computed-property-existence-check`, `no-invalid-property-descriptor`, `no-invalid-well-known-symbol-methods`, `no-unreadable-new-expression`, `no-unreadable-object-destructuring`, `no-unsafe-property-key`, `no-unused-builtin-method-return`, `no-unused-properties`, `prefer-object-define-properties`, `prefer-object-destructuring-defaults`, `prefer-single-object-destructuring` |
| Unicorn, platform | 11 | `consistent-json-file-read`, `no-invalid-intl-options`, `no-invalid-temporal-arithmetic`, `no-unsafe-buffer-conversion`, `no-unsafe-json-serialization`, `no-using-resource-escape`, `prefer-dispose`, `prefer-temporal`, `prefer-temporal-conversion`, `prefer-uint8array-base64`, `prefer-uint8array-hex` |
| Unicorn, promises | 10 | `no-async-iterator-callback`, `no-async-promise-finally`, `no-multiple-promise-resolver-calls`, `prefer-array-from-async`, `prefer-await`, `prefer-promise-static-methods`, `prefer-promise-try`, `prefer-promise-with-resolvers`, `prefer-queue-microtask`, `prefer-then-catch` |
| Unicorn, strings | 14 | `no-incorrect-template-string-interpolation`, `no-unnecessary-string-trim`, `no-unsafe-string-replacement`, `prefer-escaped-irregular-whitespace`, `prefer-literal-ascii`, `prefer-regexp-escape`, `prefer-short-escape-sequences`, `prefer-single-replace`, `prefer-split-limit`, `prefer-string-match-all`, `prefer-string-pad-start-end`, `prefer-string-repeat`, `prefer-unicode-code-point-escapes`, `string-content` |
| Unicorn, web | 12 | `no-blob-to-file`, `no-conflicting-constraints`, `no-ineffective-csp-directives`, `no-invalid-boolean-attribute-value`, `no-invalid-file-input-accept`, `no-invalid-url-protocol-comparison`, `no-url-in-search-params`, `prefer-location-assign`, `prefer-url-can-parse`, `prefer-url-href`, `prefer-url-search-parameters`, `require-css-escape` |
| functional | 11 | `no-let`, `prefer-property-signatures`, `readonly-type`, `no-classes`, `no-this-expressions`, `functional-parameters`, `no-expression-statements`, `no-loop-statements`, `no-promise-reject`, `no-throw-statements`, `no-try-statements` |
| typescript-eslint, without types | 1 | `member-ordering` |
| unused-imports | 2 | `no-unused-imports`, `no-unused-vars` |

Rewritten by EN-078 on 2026-10-17, each rule checked again first (Product Owner's decisions of F19's run): of the 44 rules of the arrays, collections and iteration groups, 39 run in the project's linter (`tools/lint/src/rules/unicorn/`, the three rows above but `no-for-each`), each on the cases of its documentation, ESLint and the Rust linter giving the same 128 findings on those cases and none on the tree; `no-for-each` runs in oxlint as `unicorn/no-array-for-each`, its former name, with more findings on a `Map` or a `Set` and no fix; four are dropped (above). All 40 are `off` in ESLint.

Kept on purpose although close to a drop: the DOM and web rules (the playground uses the DOM, files and `URL`), the loop, `throw` and `try` rules (allowed in the playground and the tests), the generator rules (one module yields), `functional/no-classes` and `no-this-expressions` (they are the bans themselves), the JSDoc layout rules and the comment rules of `@stylistic` (no source says oxfmt rewrites comments).

### Triage of oxlint's native and type-aware rules (EN-074)

Decided by the Product Owner on 2026-10-17, before any rule was enabled or rewritten: every rule enabled today is triaged — oxlint's native rules, `tsgolint`'s type-aware ones and the 265 above — and a rule is dropped when what it guards is already forbidden by a rule kept, by TypeScript's compiler or by the project's settings, or when this code cannot contain what it checks; fewer rules make a faster lint, and a rule that guards nothing here is never rewritten. Done on 2026-10-17 from `@oxlint/migrate 1.87.0 --type-aware` run on `eslint/` (432 rules enabled, `@typescript-eslint/prefer-destructuring` aside), each claim of overlap tried on a probe file with oxlint 1.87.0 or `tsc` 7.0.2 and the project's `tsconfig.json`.

| Group | Enabled | Kept | Dropped |
| --- | --- | --- | --- |
| ESLint core | 163 | 127 | 36 |
| typescript-eslint, type-aware (`tsgolint`) | 53 | 46 | 7 |
| typescript-eslint, without types | 47 | 30 | 17 |
| Unicorn | 127 | 118 | 9 |
| Import | 24 | 17 | 7 |
| JSDoc | 18 | 12 | 6 |
| **All** | **432** | **350** | **82** |

The 82 dropped, by reason ("types": a type-aware rule of `tsgolint`):

| Rules dropped | Reason |
| --- | --- |
| `constructor-super` (ts(2335)), `getter-return` (ts(2378)), `no-class-assign` (ts(2629)), `no-const-assign` (ts(2588)), `no-dupe-class-members` (ts(2393)), `no-dupe-keys` (ts(1117)), `no-func-assign` (ts(2630)), `no-import-assign` (ts(2632)), `no-new-native-nonconstructor` (ts(7009)), `no-obj-calls` (ts(2349)), `no-setter-return` (ts(2408)), `no-this-before-super` (ts(2376)), `no-unsafe-negation` (ts(2322), ts(2365)), `no-with` (ts(1101)), `no-delete-var` (ts(1102)), `no-nonoctal-decimal-escape` (ts(1488)), `import/default` (ts(1192)), `import/namespace` (ts(2339)), `import/no-amd` (ts(2304)), `import/no-webpack-loader-syntax` (ts(2307)) | TypeScript's compiler refuses the code the rule reports, with the error in parentheses (`REF-TSESLINT-ESLINT-RECOMMENDED` for the core rules; the import rules tried) |
| `no-implicit-globals` | it checks scripts only; every file is a module (`moduleDetection: force`) |
| `class-methods-use-this`, `max-classes-per-file`, `no-constructor-return`, `no-empty-static-block`, `no-useless-constructor`, `typescript/class-literal-property-style`, `typescript/explicit-member-accessibility`, `typescript/no-extraneous-class`, `typescript/no-unnecessary-parameter-property-assignment`, `typescript/parameter-properties`, `typescript/prefer-readonly` (types), `typescript/prefer-return-this-type` (types), `unicorn/no-static-only-class`, `unicorn/prefer-class-fields`, `unicorn/custom-error-definition`, `unicorn/no-useless-error-capture-stack-trace` | no class can exist, error subclasses included: `functional/no-classes` forbids them |
| `typescript/no-misused-new`, `typescript/no-unsafe-declaration-merging` | no class can exist (`functional/no-classes`), nor an interface (`typescript/consistent-type-definitions: type`) |
| `jsdoc/implements-on-classes` | no class can exist, and `@implements` is refused by `jsdoc/check-tag-names` (`typed: true`) |
| `typescript/no-this-alias`, `unicorn/no-this-assignment` | `this` cannot appear: `functional/no-this-expressions` forbids it |
| `accessor-pairs`, `grouped-accessor-pairs`, `unicorn/no-accessor-recursion`, `typescript/related-getter-setter-pairs` (types) | getters and setters are forbidden (`no-restricted-syntax`) |
| `typescript/no-duplicate-enum-values`, `typescript/prefer-enum-initializers`, `typescript/prefer-literal-enum-member`, `typescript/no-mixed-enums` (types), `typescript/no-unsafe-enum-comparison` (types) | `enum` is forbidden (`no-restricted-syntax`) |
| `guard-for-in`, `typescript/no-for-in-array` (types) | `for…in` is forbidden (`no-restricted-syntax`) |
| `typescript/prefer-namespace-keyword`, `typescript/no-unnecessary-qualifier` (types) | namespaces are forbidden, declarations included (`typescript/no-namespace`), and enums too: no qualifier to check |
| `prefer-const`, `init-declarations`, `no-unassigned-vars`, `vars-on-top`, `block-scoped-var`, `import/no-mutable-exports` | only `const` can be declared, and a `const` always has its value: `no-var` and `functional/no-let` forbid `var` and `let` |
| `no-loop-func` | it reports a closure over a variable that a loop can change; only `const` can be declared and no parameter is reassigned (`no-param-reassign`) |
| `sort-vars` | `one-var: never` leaves one declarator per declaration: nothing to sort |
| `no-extra-label`, `no-label-var`, `no-unused-labels` | labels are forbidden by `no-labels` |
| `no-eq-null` | `eqeqeq: always` already reports `== null` |
| `typescript/no-confusing-non-null-assertion`, `typescript/no-extra-non-null-assertion`, `typescript/no-non-null-asserted-nullish-coalescing`, `typescript/no-non-null-asserted-optional-chain` | `!` is forbidden by `typescript/no-non-null-assertion` |
| `import/no-commonjs`, `import/no-dynamic-require`, `typescript/no-require-imports` | `unicorn/prefer-module` reports every `require` call, dynamic ones included, and TypeScript refuses `import x = require()` in a module (ts(1202)) |
| `unicorn/no-new-buffer` | `new Buffer()` is deprecated in Node.js's types: `typescript/no-deprecated` reports it |
| `unicorn/no-invalid-fetch-options`, `unicorn/prefer-response-static-json` | the library and the playground are local only (Q5) and call no `fetch`; only two tools of `scripts/` do, and they move to Rust in F21 |
| `jsdoc/check-access`, `jsdoc/check-property-names`, `jsdoc/require-property`, `jsdoc/require-property-description`, `jsdoc/require-property-name` | `@access`, `@typedef` and `@property` are refused by `jsdoc/check-tag-names` (`typed: true`) |

Kept on purpose although close to a drop: `no-unreachable` (the compiler only suggests it, `allowUnreachableCode` left unset), `no-caller` and `prefer-rest-params` (`arguments` is allowed in the tests and the tools, where `functional/functional-parameters` is off), `unbound-method` (a DOM method passed without its object loses it), `no-plusplus`, `operator-assignment` and `logical-assignment-operators` (members can still change in the shell), and the type-aware `no-unnecessary-type-assertion` and `no-unsafe-type-assertion` (they check the assertions the register of exceptions allows).

The 8 rules `@oxlint/migrate` skipped as unstable ("nursery") run in oxlint when named, so they are triaged too rather than left in ESLint:

| Rule | Verdict |
| --- | --- |
| `no-unreachable-loop`, `no-restricted-exports` (its five `restrictDefaultExports` options; off in the root configurations, as in ESLint), `typescript/no-unnecessary-condition` (types), `typescript/prefer-optional-chain` (types), `unicorn/no-useless-iterator-to-array` | carried, named in `.oxlintrc.json` with ESLint's options; `no-restricted-exports` also reports `export { default } from`, which `import/no-default-export` lets through |
| `import/export` | dropped: TypeScript refuses a name exported twice (ts(2323), ts(2528)) |
| `import/named` | dropped: TypeScript refuses an import of a name the module does not export (ts(2614)) |
| `no-useless-assignment` | dropped: only `const` can be declared and no parameter is reassigned, so no value can be assigned and overwritten unread |

The four ESLint core rules oxlint lacks, triaged the same way:

| Rule | Verdict |
| --- | --- |
| `no-restricted-syntax`, `require-atomic-updates` | kept, rewritten in the Rust linter (EN-075) |
| `consistent-this` | dropped: `this` cannot appear (`functional/no-this-expressions`), so no alias of it can |
| `strict` | dropped: every file is a module, so strict code, and `unicorn/prefer-module` (native, kept) reports any `"use strict"` directive |

## `tsc`'s two jobs and Oxc (EN-091, 2026-10-17)

Asked by the Product Owner during EN-091: which Oxc tool can take over each of `tsc`'s two jobs here, tried on this repository. The third round's line "the type check itself is TypeScript 7's compiler, not oxlint's `--type-check`" is replaced by the answer below (ADR-0071).

| Job of `tsc` | Oxc's tool | Tried here | Kept |
| --- | --- | --- | --- |
| the type check (`tsc --noEmit`, 0.67 to 0.70 s) | none of its own: Oxc lists no type checker (`REF-OXC-README`); oxlint's `--type-check` adds the diagnostics of typescript-go, run by `tsgolint`, to the lint (`REF-OXLINT-CLI`, "experimental"; `REF-OXLINT-TYPE-AWARE`, "can replace a separate `tsc --noEmit` step") | the same errors as `tsc` on the tree (none) and on planted ones: TS2307, TS2322, TS2375, TS18048 in one module, TS2322 in the playground, TS2403 across two files; on an invalid `tsconfig.json` it stops at the first invalid option, as `tsc` stops before the code; 1.1 s for the lint with it against 0.8 s without | oxlint with `--type-check` in every lint pass; `tsc` out of `check:all` |
| the declarations (`tsc -p tsconfig.build.json`, 0.63 s, 589 files) | `oxc_isolated_declarations` and `oxc_codegen` 0.153.0, for code under TypeScript's `isolatedDeclarations` (`REF-OXC-ISOLATED-DECLARATIONS`, `REF-TS-ISOLATED-DECLARATIONS`) | `src/` breaks the rule twice, both TS9010 on an exported constant computed from another (`DEGREES_PER_HALF_TURN`, `QUARTER_CIRCLE_HANDLE`), none on a function's return type nor a local: the whole tree would break it 87 times, 64 in `eslint/`. Annotated, Oxc writes the 589 files in 0.06 s: 103 byte for byte as `tsc`, the others differing only in indentation (tabs), a wrapped type parameter list and `.5` for `0.5`, the same in meaning | `tools/declarations`, a crate of the workspace; `tsconfig.build.json` removed |

What stays with `tsc` (TypeScript 7): `npm run typecheck` as a command of its own, the editors, and the test that TypeScript 7 is the project's compiler. Nothing in Oxc's documentation covers a type check apart from oxlint's.
