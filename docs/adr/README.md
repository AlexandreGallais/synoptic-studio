# Architecture Decision Records

Format: Status, Context, Decision, Consequences, References.
An accepted ADR is never rewritten: it is superseded by a new ADR (`Superseded by ADR-XXXX`).
All ADRs were translated from French to English on 2026-10-08 (project rule: English everywhere); the decisions are unchanged.

| ADR                                                        | Title                                                                                                        | Status                 |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | ---------------------- |
| [0001](./0001-geometric-scope.md)                          | Geometric scope: segments and circular arcs                                                                  | Accepted               |
| [0002](./0002-everything-is-a-path.md)                     | Everything is a path, except text                                                                            | Accepted               |
| [0003](./0003-integer-model.md)                            | Integer model, derived geometry                                                                              | Accepted               |
| [0004](./0004-orthogonal-pipes.md)                         | Orthogonal pipes only                                                                                        | Accepted               |
| [0005](./0005-parametric-pipeline.md)                      | Parametric pipeline: numbers → maths → SVG                                                                   | Accepted               |
| [0006](./0006-non-destructive-booleans.md)                 | Non-destructive boolean operations                                                                           | Accepted               |
| [0007](./0007-corner-radius-clamping.md)                   | Corner radius clamping: local proportional reduction                                                         | Accepted               |
| [0008](./0008-quarter-turn-rotation.md)                    | Instance rotation by quarter turns                                                                           | Accepted               |
| [0009](./0009-toolchain.md)                                | Toolchain: TypeScript 6, Vite, Vitest, ESLint, Prettier, VitePress                                           | Accepted               |
| [0010](./0010-lint-first-strictness.md)                    | Strictness carried by ESLint first, TypeScript second                                                        | Accepted               |
| [0011](./0011-function-kinds.md)                           | Function kinds: `@kind` tag and per-kind limits                                                              | Accepted               |
| [0012](./0012-folder-barrels.md)                           | One `index.ts` per folder, canonical import paths                                                            | Accepted               |
| [0013](./0013-prettier-alongside-eslint.md)                | Prettier next to ESLint, not inside it                                                                       | Accepted               |
| [0014](./0014-functional-core-imperative-shell.md)         | Functional core, imperative shell                                                                            | Accepted               |
| [0015](./0015-one-export-per-file.md)                      | One export per file, named like the file                                                                     | Superseded by ADR-0019 |
| [0016](./0016-explicit-inputs.md)                          | Explicit inputs: no default nor optional parameter in the library                                            | Accepted               |
| [0017](./0017-in-repository-backlog.md)                    | Agile backlog kept in the repository                                                                         | Accepted               |
| [0018](./0018-curves-in-static-drawings.md)                | Bézier curves allowed in static drawings only                                                                | Accepted               |
| [0019](./0019-kebab-case-file-names.md)                    | Kebab-case file names, one export per file                                                                   | Accepted               |
| [0020](./0020-feature-research-and-audit.md)               | Every feature starts with a research spike and ends with an audit                                            | Accepted               |
| [0021](./0021-agent-working-environment.md)                | Agent working environment: skills, rules, hooks and an independent auditor                                   | Accepted               |
| [0022](./0022-property-based-testing-and-link-checking.md) | Property-based tests, weekly link check, mutation tool deferred                                              | Accepted               |
| [0023](./0023-inspection-cadence.md)                       | Inspection cadence: demo at each feature, review and retrospective at each epic                              | Accepted               |
| [0024](./0024-session-journal.md)                          | Session journal and transcript search instead of a database                                                  | Accepted               |
| [0025](./0025-portable-core.md)                            | A portable core: any JavaScript engine, transcribable to another language                                    | Accepted               |
| [0026](./0026-contributions-through-issues.md)             | Contributions through issues only; the agent writes every change                                             | Accepted               |
| [0027](./0027-requirements-and-traceability.md)            | Acceptance criteria as requirements, tests traced to them, a test report per release                         | Accepted               |
| [0028](./0028-autonomous-runs.md)                          | Feature branches and autonomous runs: stories merge into their feature, the Product Owner merges the feature | Accepted               |
| [0030](./0030-free-v1-then-pro-edition.md)                 | A finished free V1, then new features in a paid edition                                                      | Accepted               |
