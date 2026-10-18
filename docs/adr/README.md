# Architecture Decision Records

Format: Status, Context, Decision, Consequences, References.
An accepted ADR is never rewritten: it is superseded by a new ADR (`Superseded by ADR-XXXX`).
All ADRs were translated from French to English on 2026-10-08 (project rule: English everywhere); on 2026-10-13 their Context sections were reworded so that they state the Product Owner's decisions without quoting them. The decisions are unchanged. Number 0036 is not used: it was taken on the F10 feature branch and never accepted (ADR-0037). Number 0053, reserved by ADR-0052 for F15's HTML layer, was written after 0056.

| ADR | Title | Status |
| --- | --- | --- |
| [0001](./0001-geometric-scope.md) | Geometric scope: segments and circular arcs | Partly superseded by ADR-0046 |
| [0002](./0002-everything-is-a-path.md) | Everything is a path, except text | Accepted |
| [0003](./0003-integer-model.md) | Integer model, derived geometry | Accepted, partly superseded by 0031, 0034 and 0035 |
| [0004](./0004-orthogonal-pipes.md) | Orthogonal pipes only | Accepted |
| [0005](./0005-parametric-pipeline.md) | Parametric pipeline: numbers → maths → SVG | Accepted |
| [0006](./0006-non-destructive-booleans.md) | Non-destructive boolean operations | Accepted |
| [0007](./0007-corner-radius-clamping.md) | Corner radius clamping: local proportional reduction | Accepted |
| [0008](./0008-quarter-turn-rotation.md) | Instance rotation by quarter turns | Accepted |
| [0009](./0009-toolchain.md) | Toolchain: TypeScript 6, Vite, Vitest, ESLint, Prettier, VitePress | Superseded by ADR-0075 |
| [0010](./0010-lint-first-strictness.md) | Strictness carried by ESLint first, TypeScript second | Superseded by ADR-0075, partly by 0071 before |
| [0011](./0011-function-kinds.md) | Function kinds: `@kind` tag and per-kind limits | Accepted |
| [0012](./0012-folder-barrels.md) | One `index.ts` per folder, canonical import paths | Accepted |
| [0013](./0013-prettier-alongside-eslint.md) | Prettier next to ESLint, not inside it | Superseded by ADR-0075 |
| [0014](./0014-functional-core-imperative-shell.md) | Functional core, imperative shell | Accepted, its DOM layers' scope moved to `tools/lint` by 0075 |
| [0015](./0015-one-export-per-file.md) | One export per file, named like the file | Superseded by ADR-0019 |
| [0016](./0016-explicit-inputs.md) | Explicit inputs: no default nor optional parameter in the library | Accepted |
| [0017](./0017-in-repository-backlog.md) | Agile backlog kept in the repository | Accepted |
| [0018](./0018-curves-in-static-drawings.md) | Bézier curves allowed in static drawings only | Partly superseded by ADR-0046 |
| [0019](./0019-kebab-case-file-names.md) | Kebab-case file names, one export per file | Accepted |
| [0020](./0020-feature-research-and-audit.md) | Every feature starts with a research spike and ends with an audit | Accepted, decision 4 amended by ADR-0072 |
| [0021](./0021-agent-working-environment.md) | Agent working environment: skills, rules, hooks and an independent auditor | Accepted, partly superseded by 0038 and 0040 |
| [0022](./0022-property-based-testing-and-link-checking.md) | Property-based tests, weekly link check, mutation tool deferred | Accepted, decision 3 superseded (`mutate`, EN-058) |
| [0023](./0023-inspection-cadence.md) | Inspection cadence: demo at each feature, review and retrospective at each epic | Accepted, extended by ADR-0049; its review and retrospective per epic superseded by 0086 (each review) |
| [0024](./0024-session-journal.md) | Session journal and transcript search instead of a database | Accepted, partly superseded by 0039 |
| [0025](./0025-portable-core.md) | A portable core: any JavaScript engine, transcribable to another language | Accepted |
| [0026](./0026-contributions-through-issues.md) | Contributions through issues only; the agent writes every change | Accepted |
| [0027](./0027-requirements-and-traceability.md) | Acceptance criteria as requirements, tests traced to them, a test report per release | Accepted |
| [0028](./0028-autonomous-runs.md) | Feature branches and autonomous runs: stories merge into their feature, the Product Owner merges the feature | Superseded by ADR-0072 |
| [0029](./0029-one-radius-per-vertex.md) | One requested corner radius per vertex in the model | Accepted, superseded by 0033 for free shapes |
| [0030](./0030-free-v1-then-pro-edition.md) | A finished free V1, then new features in a paid edition | Accepted |
| [0031](./0031-free-shape-keeps-fractional-vertices.md) | A free shape keeps the fractional vertices of a converted polygon | Accepted, partly superseded by 0034 |
| [0032](./0032-rectangle-is-a-free-shape.md) | The rectangle is a free shape, stretched by its width and height | Accepted, partly superseded by 0033 and 0035 |
| [0033](./0033-free-shape-of-corners.md) | A free shape of corners, fractions until edited, points typed in the SVG's coordinates | Accepted, partly superseded by 0035 |
| [0034](./0034-only-the-coordinate-typed-is-an-integer.md) | Only the coordinate typed is an integer; edits keep the fractions | Accepted, partly superseded by 0035 |
| [0035](./0035-angle-and-position-for-every-shape.md) | An angle and a position for every shape | Accepted |
| [0037](./0037-stretched-polygon.md) | A stretched option for the regular polygon | Accepted |
| [0038](./0038-develop-branch-and-local-checks.md) | A `develop` branch, checks run locally, one CI on the pull request into `main` | Superseded by ADR-0072 |
| [0039](./0039-local-source-library.md) | A local library of the sources read: SQLite full-text search behind an MCP server | Accepted, partly superseded by 0077 |
| [0040](./0040-agents-md-for-any-agent.md) | `AGENTS.md` holds the rules for any coding agent | Accepted |
| [0041](./0041-private-records-repository.md) | Private records in a sibling repository, saved by `npm run records:save` | Superseded by 0044, then 0081 |
| [0042](./0042-folders-by-theme.md) | Folders by theme, 15 modules as a signal, a shape's type in its folder | Superseded by ADR-0043 |
| [0043](./0043-folders-by-domain.md) | Folders by business domain, files and folders never side by side | Accepted |
| [0044](./0044-records-inside-the-project.md) | The private records inside the project folder, ignored by Git | Superseded by 0081 |
| [0045](./0045-run-is-the-go-ahead.md) | `/run` is the go-ahead; validated features merge by themselves; releases when the Product Owner asks | Superseded by ADR-0072 |
| [0046](./0046-cubic-sides-in-symbols.md) | Cubic Bézier sides in symbols, from points with handles | Accepted |
| [0047](./0047-fillet-beside-a-curve.md) | A corner fillet beside a cubic side: tangent arc, the curve cut, a radius too large reduced | Accepted |
| [0048](./0048-round-shape-as-cubic-sides.md) | The round shape drawn as four cubic sides, not elliptical arcs | Accepted |
| [0049](./0049-every-calculation-named.md) | Every calculation named, and a naming pass closes each epic | Accepted; its naming pass per epic superseded by 0086 (each review) |
| [0050](./0050-retrospective-e14-tools-and-merges.md) | The agent's tools, parallel checks, and validated epic stories that merge by themselves | Accepted, decision 1 superseded by ADR-0073, decisions 2 and 3 by ADR-0072 |
| [0051](./0051-main-thread-free-for-the-product-owner.md) | The main thread kept free: stories worked by subagents | Superseded by ADR-0063 |
| [0052](./0052-text-measured-and-drawn-as-positioned-lines.md) | Text measured from font metrics and drawn as positioned lines (`text`, `tspan`) | Accepted, decision 4 amended by ADR-0054 and ADR-0055, decision 1 by ADR-0055, decisions 7 and 8 settled by ADR-0053, decisions 6 and 8 amended by ADR-0060 |
| [0053](./0053-html-layer-and-embedded-fonts.md) | The HTML layer of a text and its embedded fonts (`switch`, `foreignObject`, `style`) | Accepted, decisions 1 and 3 partly superseded and decision 2 amended by ADR-0060 |
| [0054](./0054-text-font-list-and-spaces-kept.md) | A text's font list and its spaces kept (`white-space`, `xml:space`) | Accepted |
| [0055](./0055-text-fallback-metrics-and-turn-on-its-group.md) | A text measured with its fallback's metrics, and turned by its group | Accepted, decision 3 completed by ADR-0057 |
| [0056](./0056-color-stored-as-hexadecimal-hsl-and-hsb-as-views.md) | A color stored as hexadecimal and an opacity; HSL and HSB as views | Accepted, decision 4 superseded by ADR-0058 |
| [0057](./0057-a-turned-text-keeps-its-turned-corner-when-it-grows.md) | A turned text keeps its turned top-left corner when it grows | Accepted |
| [0058](./0058-hsl-and-hsb-shown-and-typed-as-integers.md) | HSL and HSB shown and typed as integers | Accepted, decision 2 amended by ADR-0059 |
| [0059](./0059-an-open-picker-rebuilds-against-the-color-it-read.md) | An open picker rebuilds each edit against the color it read | Accepted |
| [0060](./0060-no-html-layer-and-fonts-embedded-at-export.md) | No HTML layer; fonts embedded only when a file is written | Accepted, decision 4 refined by ADR-0061 |
| [0061](./0061-an-installed-font-without-the-latin-letters-is-not-listed.md) | An installed font without the basic Latin letters is not listed | Accepted, decision 2 refined by ADR-0062 |
| [0062](./0062-an-imported-font-follows-the-rules-of-an-installed-one.md) | An imported font follows the rules of an installed one | Accepted |
| [0063](./0063-runs-with-a-limited-number-of-agents.md) | Runs with a limited number of agents, enforced by the tools | Superseded by ADR-0072 |
| [0064](./0064-checks-by-level.md) | Checks by level: the story, the feature, the epic | Accepted, from E02 |
| [0065](./0065-parallel-branches-merged-without-conflicts.md) | Parallel branches merged without conflicts on shared files | Accepted, decision 1 superseded by ADR-0068 |
| [0066](./0066-a-register-of-exceptions.md) | A register of exceptions, and a byte buffer written once | Accepted |
| [0067](./0067-release-pull-request-on-develop.md) | The release pull request on `develop`, only the release on `main` | Accepted, confirmed at VAL-019; decisions 1 to 3 and 5 superseded by 0079 |
| [0068](./0068-tables-without-padding-moved-to-f19.md) | Tables without padding moved to F19's formatter | Accepted, confirmed at VAL-019 |
| [0069](./0069-tooling-fast-for-the-agent.md) | Tooling as fast as it can be, for the agent | Accepted |
| [0070](./0070-tests-without-over-testing.md) | Tests without over-testing: one claim, one place; slow preparation done once | Accepted |
| [0071](./0071-type-errors-with-the-lint-declarations-by-oxc.md) | Type errors with the lint, declarations by Oxc | Accepted |
| [0072](./0072-how-a-run-works.md) | How a run works: the go-ahead, the branches and merges, three agents and one heavy command | Accepted, its release lines superseded by 0079, its checkpoint per feature and its branch start in part by 0084 |
| [0073](./0073-every-new-tool-in-rust.md) | Every new script, hook or agent tool written in Rust | Accepted, decision 1 partly superseded by 0078 |
| [0074](./0074-the-agent-shapes-its-own-method-and-tools.md) | The agent shapes its own method and tools, the Product Owner decides | Accepted, amended by 0085 (the tool incident cards) |
| [0075](./0075-the-toolchain-on-oxc.md) | The toolchain on Oxc: oxlint, the project's Rust linter, oxfmt and TypeScript 7 | Accepted |
| [0076](./0076-ported-tooling-code-under-its-licence.md) | Code ported into the tools under its licence, never into the library | Accepted |
| [0077](./0077-the-library-server-in-rust-on-rmcp.md) | The library's server in Rust on rmcp, its file named by its own variable | Accepted |
| [0078](./0078-the-agents-tools-called-by-their-binary.md) | The hooks, the Git hooks and the agent call the Rust tools by their binary's path | Accepted, its `.husky/` superseded by 0080, its scripts of the agent's tools by 0082 |
| [0079](./0079-the-release-on-a-release-branch-by-our-own-tool.md) | The release prepared on a release branch from `develop`, published by our own tool | Superseded by 0083 |
| [0080](./0080-the-git-hooks-in-githooks-installed-by-our-own-tool.md) | The Git hooks in `.githooks/`, installed by `git-hooks install` | Accepted |
| [0081](./0081-the-private-records-in-a-local-folder-shared-by-projects.md) | The private records in a local folder shared by every project, never sent | Accepted |
| [0082](./0082-npm-scripts-kept-for-a-person-or-a-caller.md) | The npm scripts kept for a person or a caller, the agent's tools by their binary only | Accepted |
| [0083](./0083-the-release-made-by-the-merge-into-main-from-a-tag.md) | The release made by the merge into `main`: the version from the tags, no release branch nor version commit | Superseded by 0086 |
| [0084](./0084-small-features-tests-first-and-a-linear-history.md) | Small features, tests first, a linear history, and agents that work apart | Accepted, from E02; in decision 3 its rebases and its `develop` → `main` at the epic's end, decision 4 and its parallel researchers superseded by 0086 |
| [0085](./0085-tool-incident-cards-tools-changed-at-an-epics-end.md) | A tool that fails is a card; tools change only at an epic's end | Accepted; tools changed at each review since 0086 |
| [0086](./0086-one-story-at-a-time-into-develop-the-release-by-a-button.md) | One story at a time, a merge commit for every merge, a review at each return, the release by a button | Accepted, implemented by EN-122, EN-117 and EN-123 |
