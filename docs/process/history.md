# History before 0.1.0

The project restarted on 2026-10-13 from one consolidated commit, version 0.1.0. During its first epic the documentation grew as a working record — every exchange, every attempt, in great detail; at the end of E01 it was rewritten into the reference it is meant to be, and the repository was started again from that clean base. This page keeps the record of what came before: every commit's message and the files it touched, without their content.

| From | To | Commits | Releases |
| --- | --- | --- | --- |
| 2026-10-08 | 2026-10-13 | 549 | 0.1.0, 0.1.1, 0.2.0, 0.2.1, 0.2.2, 0.3.0, 0.4.0, 0.5.0, 0.6.0, 0.6.1, 0.6.2, 0.6.3, 0.6.4, 0.6.5, 0.6.6, 0.6.7, 0.6.8, 0.6.9, 0.7.0, 0.7.1, 0.7.2, 0.7.3, 0.7.4, 0.7.5, 0.7.6, 0.7.7, 0.7.8, 0.7.9, 0.7.10, 0.7.11, 0.8.0, 0.8.1, 0.8.2, 0.8.3, 0.8.4, 0.8.5, 0.8.6, 0.9.0, 0.9.1, 0.9.2, 0.9.3, 0.9.4, 0.9.5, 0.9.6, 0.9.7, 0.9.8, 0.9.9, 0.9.10, 0.10.0, 0.10.1, 0.10.2, 0.10.3, 0.10.4, 0.10.5, 0.11.0, 0.11.1, 0.11.2, 0.11.3, 0.12.0, 0.12.1, 0.12.2, 0.12.3, 0.12.4, 0.12.5, 0.13.0, 0.13.1, 0.13.2, 0.13.3, 0.13.4, 0.14.0, 0.14.1, 0.14.2 |

The work itself is described in the [E01 retrospective](./retrospectives/E01-retrospective.md) and the [journal](./journal.md).

## 2026-10-08

- **chore(tooling): bootstrap strict tooling, documentation site and backlog**
  Initial state of the project before any library code (src/ is an empty barrel). - ESLint 10: every rule of every plugin decided once, in small per-theme files, with a completeness audit test; custom rules for function kinds, one export per file, folder barrels, canonical import paths and verified @see references. - TypeScript 6 (strict), Prettier + EditorConfig, Vitest, Vite library build and playground; npm audit and latest-version checks block the pipeline. - Git workflow: Conventional Commits (commitlint), branch names, husky hooks. - Documentation in English: domain, ADR-0001 to ADR-0017, conventions, tooling, research protocol, bibliography; VitePress site with TypeDoc API pages. - Agile backlog in the repository (epic, feature, story, task, no iterations), with eight draft epics awaiting the Product Owner's review.
  Refs: ADR-0009, ADR-0010, ADR-0011, ADR-0012, ADR-0013, ADR-0014, ADR-0015, ADR-0016; Refs: ADR-0017, Q10, Q11
  Files: `.editorconfig`, `.github/workflows/ci.yml`, `.gitignore`, `.husky/commit-msg`, `.husky/pre-commit`, `.husky/pre-push`, `.idea/.gitignore`, `.idea/SVG editor.iml`, `.idea/inspectionProfiles/Project_Default.xml`, `.idea/jsLinters/eslint.xml`, `.idea/modules.xml`, `.idea/prettier.xml` and 215 more.
- **ci: add GitHub quality gate, security scans and automated releases**
  - CI on pull requests and main: check:all (format, lint, secrets, types, tests, audit, latest versions, docs build), build, commitlint on every commit, branch name, dependency review. - CodeQL security and quality analysis; secretlint before commit and in CI. - release-please for versions and CHANGELOG from Conventional Commits. - Dependabot weekly updates, pull request template. - Workflow: one branch per story, one commit per task, rebase merge.
    Refs: ADR-0009, ADR-0017
    Files: `.github/dependabot.yml`, `.github/pull_request_template.md`, `.github/workflows/ci.yml`, `.github/workflows/codeql.yml`, `.github/workflows/release.yml`, `.release-please-manifest.json`, `.secretlintignore`, `.secretlintrc.json`, `CLAUDE.md`, `commitlint.config.ts`, `docs/backlog/README.md`, `docs/backlog/_story.md` and 9 more.
- **Merge pull request #5 from AlexandreGallais/dependabot/github_actions/googleapis/release-please-action-5**
  ci: Bump googleapis/release-please-action from 4 to 5
- **Merge pull request #4 from AlexandreGallais/dependabot/github_actions/actions/setup-node-7**
  ci: Bump actions/setup-node from 4 to 7
- **Merge pull request #3 from AlexandreGallais/dependabot/github_actions/github/codeql-action-4**
  ci: Bump github/codeql-action from 3 to 4
- **Merge pull request #1 from AlexandreGallais/dependabot/github_actions/actions/dependency-review-action-5**
  ci: Bump actions/dependency-review-action from 4 to 5
- **Merge branch 'main' into dependabot/github_actions/actions/checkout-7**
- **Merge pull request #2 from AlexandreGallais/dependabot/github_actions/actions/checkout-7**
  ci: Bump actions/checkout from 4 to 7
- **build(eslint): lint Markdown structure with @eslint/markdown**
  - @eslint/markdown checks headings, links, code languages, bare URLs and tables of every page; its rules are decided in eslint/rules/markdown and audited. - markdownlint-cli2 and Stylelint are not installed: they depend on braces, vulnerable without fix (GHSA-vfj7-8cjw-p6xm); recorded in the docs. - katex forced to 0.19 (GHSA-238p-pmpm-9mq7); overrides checked by test.
    Refs: ADR-0009, ADR-0010
    Files: `docs/conventions/architecture.md`, `docs/conventions/files.md`, `docs/conventions/lint.md`, `docs/conventions/writing.md`, `docs/references.md`, `docs/research/requests/_template.md`, `docs/tooling/dependencies.md`, `docs/tooling/git-workflow.md`, `docs/tooling/versions-and-security.md`, `eslint/config.test.ts`, `eslint/config.ts`, `eslint/rules/index.ts` and 9 more.
- **docs(backlog): rewrite epics from the product vision and plan F01**
  - Domain: four tools (Symbol Editor, Configurator, View Editor, drawing tools), configuration trees, interfaces, property groups, business types and libraries, animations, static drawings, free shape rotation in symbols. - Settled Q5 (local server), Q10 (5 decimals, EPSILON 1e-9), Q11 (clockwise from the top-left vertex, cyclic contours); opened Q12, Q13, Q14. - Eleven draft epics in the Product Owner's order; E01 split into features; F01 split into eleven stories with tasks. - File names and titles carry the ancestry (E01-F01-US-001); backlog test checks prefixes and YAML quoting.
    Refs: ADR-0017, Q5, Q10, Q11, Q12, Q13, Q14
    Files: `CLAUDE.md`, `backlog.test.ts`, `docs/backlog/README.md`, `docs/backlog/_feature.md`, `docs/backlog/_story.md`, `E01`, `E01`, `E02`, `E02`, `E03`, `E03`, `E04` and 36 more.
- **docs(adr): allow Bézier curves in static drawings and settle Q12 to Q15**
  - ADR-0018: Figma-like Bézier pen in static drawings only; symbols keep ADR-0001. - Q13: animations remap a property range to their own range. - Q14: a configuration node brings its own and its ancestors' options; sub-trees of other trees can be added. - Q15: rectangle sizes are >= 0; US-001 and EN-001 criteria updated. - Stylelint postponed until SCSS starts (braces advisory); GitHub settings written step by step in docs/tooling/ci.md.
    Refs: ADR-0001, ADR-0018, Q12, Q13, Q14, Q15, US-001, EN-001
    Files: `CLAUDE.md`, `docs/adr/0001-geometric-scope.md`, `docs/adr/0018-curves-in-static-drawings.md`, `docs/adr/README.md`, `E10`, `EN-001`, `US-001`, `docs/domain/README.md`, `docs/domain/configuration.md`, `docs/domain/shapes.md`, `docs/domain/symbols-and-views.md`, `docs/tooling/ci.md` and 1 more.
- **docs(backlog): set EN-001 ready**
  Product Owner agreement given in the session of 2026-10-08.
  Refs: EN-001
  Files: `EN-001`, `docs/backlog/stories/README.md`.
- **feat(io): add SVG_DECIMALS output precision**
  Maximum number of decimals of SVG output numbers, 5 (Q10).
  Refs: EN-001.T2, Q10
  Files: `src/index.ts`, `src/io/SVG_DECIMALS.ts`, `src/io/index.ts`.
- **build(eslint): require kebab-case file names named after the export**
  - local/one-export-per-file now expects the kebab-case of the export (formatSvgNumber -> format-svg-number.ts); unicorn/filename-case applies kebab-case everywhere again; SVG_DECIMALS.ts renamed svg-decimals.ts. ADR-0019 supersedes ADR-0015. - jsdoc/informative-docs ignores @kind, a classification that may repeat the function name (format on formatSvgNumber).
    Refs: EN-001, ADR-0019
    Files: `CLAUDE.md`, `docs/adr/0015-one-export-per-file.md`, `docs/adr/0019-kebab-case-file-names.md`, `docs/adr/README.md`, `docs/conventions/files.md`, `docs/conventions/lint.md`, `eslint/plugin/rules/one-export-per-file.test.ts`, `eslint/plugin/rules/one-export-per-file.ts`, `eslint/rules/jsdoc/content.ts`, `eslint/scopes/all.ts`, `eslint/scopes/modules.ts`, `eslint/settings/files.ts` and 2 more.
- **feat(io): add formatSvgNumber for fixed-precision output**
  Rounds to SVG_DECIMALS decimals and writes the shortest form: no trailing zeros, no negative zero. Tests were written first (cases computed by hand) and committed with the implementation, as the pre-commit hook requires a compiling tree.
  Refs: EN-001.T1, EN-001.T3, Q10
  Files: `src/io/format-svg-number.test.ts`, `src/io/format-svg-number.ts`.
- **docs(backlog): record EN-001 output rules and tick its tasks**
  The shortest-form rules (no trailing zeros, no negative zero) accepted by the Product Owner join Q10 in docs/domain/shapes.md. EN-001 stays in-progress until the Product Owner accepts it by merging the pull request.
  Refs: EN-001.T4, Q10; Closes: EN-001
  Files: `EN-001`, `docs/backlog/stories/README.md`, `docs/domain/shapes.md`.
- **build(deps): bump vite from 8.3.3 to 8.3.4**
  Refs: ADR-0009
  Files: `package-lock.json`, `package.json`.
- **ci: fix the release pull request and publish the docs site on release**
  - CHANGELOG.md is generated by release-please: excluded from Prettier and secretlint (it failed format:check on the release pull request). - First version is 0.1.0 (initial-version) instead of release-please's 1.0.0. - After a release, the docs job builds VitePress + TypeDoc and deploys it to GitHub Pages (also runnable by hand); the site base path comes from DOCS_BASE.
    Refs: ADR-0009
    Files: `.github/workflows/release.yml`, `.prettierignore`, `.secretlintignore`, `docs/.vitepress/config.ts`, `docs/tooling/ci.md`, `docs/tooling/releases.md`, `release-please-config.json`.
- **docs(backlog): set EN-001 done**
  Accepted by the Product Owner by merging pull request #6.
  Refs: EN-001
  Files: `EN-001`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.1.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(tooling): require full coverage and add standard repository files**
  - Vitest coverage (v8) with 100 % thresholds on src/, run by npm test. - Node version pinned in .nvmrc (CI reads it) and enforced by engine-strict. - SECURITY.md with private vulnerability reporting (enabled on GitHub). - README: CI, CodeQL and release badges, link to the documentation site. - Docs and CLAUDE.md: post-merge routine, release pull request CI, lessons learned from EN-001.
    Refs: ADR-0009, EN-001
    Files: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `.npmrc`, `.nvmrc`, `CLAUDE.md`, `README.md`, `SECURITY.md`, `docs/conventions/testing.md`, `docs/tooling/commands.md`, `docs/tooling/dependencies.md`, `docs/tooling/git-workflow.md`, `docs/tooling/releases.md` and 3 more.
- **docs(tooling): list the tools considered for later**
  knip, size-limit, api-extractor and Stryker, with what they would bring and when.
  Refs: ADR-0009
  Files: `docs/tooling/dependencies.md`, `package.test.ts`.
- **chore: license the project under Apache-2.0**
  Official text from apache.org; NOTICE holds the copyright line. package.json declares the license, repository and homepage; LICENSE and NOTICE ship with the package.
  Files: `LICENSE`, `NOTICE`, `README.md`, `package.json`.
- **ci: run release-please with a token and rename to synoptic-studio**
  - release-please uses RELEASE_PLEASE_TOKEN, so the CI runs on its pull requests (token expires 2026-12-31). - The repository is now github.com/AlexandreGallais/synoptic-studio: links, README, CLAUDE.md and docs updated; the Pages base path follows the name.
    Refs: ADR-0009
    Files: `.github/workflows/release.yml`, `CLAUDE.md`, `docs/.vitepress/config.ts`, `docs/tooling/releases.md`.
- **chore(main): release 0.1.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set US-001 ready**
  Product Owner agreement given in the session of 2026-10-08.
  Refs: US-001
  Files: `US-001`, `docs/backlog/stories/README.md`.
- **feat(math): add the Point type**
  Point of the plane in SVG user units; integer in the model, float in derived geometry (ADR-0003).
  Refs: US-001.T2, US-001.T4
  Files: `src/index.ts`, `src/math/index.ts`, `src/math/point.ts`.
- **feat(model): add the rectangle contour and its validation**
  - Rectangle: integer width and height, >= 0. - rectangleContour: four vertices clockwise on screen from the top-left one (Q11), integer coordinates (ADR-0003), degenerate sizes allowed (Q15). - isValidRectangle: rejects negative or fractional sizes (Q15, ADR-0003). Tests were written first (cases computed by hand) and committed with the implementation, as the pre-commit hook requires a compiling tree.
    Refs: US-001.T1, US-001.T3, US-001.T4, Q11, Q15
    Files: `src/index.ts`, `src/model/index.ts`, `src/model/is-valid-rectangle.test.ts`, `src/model/is-valid-rectangle.ts`, `src/model/rectangle-contour.test.ts`, `src/model/rectangle-contour.ts`, `src/model/rectangle.ts`.
- **docs(backlog): tick US-001 tasks and record the empty-barrel pitfall**
  US-001 stays in-progress until the Product Owner accepts it by merging the pull request.
  Refs: US-001.T4; Closes: US-001
  Files: `CLAUDE.md`, `US-001`, `docs/backlog/stories/README.md`.
- **fix(model): document type members and catch it before the docs build**
  - Point and Rectangle properties are documented: TypeDoc failed the docs build of the pull request on undocumented properties. - jsdoc/require-jsdoc now covers type members in src/, so lint reports it. - The pre-push hook runs check:all (docs build included).
    Refs: US-001
    Files: `.husky/pre-push`, `CLAUDE.md`, `docs/conventions/documentation.md`, `docs/tooling/git-workflow.md`, `eslint/scopes/modules.ts`, `eslint/settings/formatting.ts`, `src/math/point.ts`, `src/model/rectangle.ts`.
- **chore(main): release 0.2.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set US-001 done**
  Accepted by the Product Owner by merging pull request #9.
  Refs: US-001
  Files: `US-001`, `docs/backlog/stories/README.md`.
- **ci: merge release pull requests automatically once green**
  release-please's pull request now enables GitHub auto-merge (rebase) with the release token: every merged story produces a release, the changelog and the docs site stay up to date without a manual step.
  Refs: ADR-0009
  Files: `.github/workflows/release.yml`, `CLAUDE.md`, `docs/tooling/releases.md`.
- **chore(main): release 0.2.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set stories done in their pull request**
  Merging a story's pull request is the Product Owner's acceptance, so the story is set to done in the pull request's last commit: main never needs a status commit after a merge. A test now checks that the folder indexes show the status written in each item file.
  Refs: ADR-0017
  Files: `CLAUDE.md`, `backlog.test.ts`, `docs/backlog/README.md`.
- **chore(main): release 0.2.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **test(backlog): check the epics index and require every item to be listed**
  The status check skipped the epics index: its rows start with an order column the pattern did not allow, so no epic was compared. The pattern now finds the item link anywhere in the row, and a new test requires every item to appear in its folder index, so a row can no longer be skipped silently. Each index was mutation-checked (a wrong status makes the test fail).
  Refs: ADR-0017
  Files: `backlog.test.ts`.
- **docs(backlog): set EN-002 ready**
  Product Owner agreement given in the session of 2026-10-08.
  Refs: EN-002
  Files: `EN-002`, `docs/backlog/stories/README.md`.
- **feat(io): write the path data of a sharp contour**
  contourToPathData writes M for the first vertex, L for the others and Z, numbers through formatSvgNumber (EN-001); an empty contour gives an empty d. Tests were written first (cases computed by hand) and committed with the implementation, as the pre-commit hook requires a compiling tree.
  Refs: EN-002.T1, EN-002.T2
  Files: `src/io/contour-to-path-data.test.ts`, `src/io/contour-to-path-data.ts`, `src/io/index.ts`.
- **docs(backlog): set EN-002 done**
  Done on merge: merging this pull request is the Product Owner's acceptance.
  Refs: EN-002; Closes: EN-002
  Files: `EN-002`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.3.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **ci: publish the playground with the docs site**
  - vite build --mode playground builds the playground into the docs site (docs/.vitepress/dist/playground); the default mode still builds the library. - The release workflow publishes it on GitHub Pages next to the docs, with a Playground link in the docs menu; check:all builds it so a broken playground fails the CI.
    Refs: ADR-0009
    Files: `.github/workflows/release.yml`, `docs/.vitepress/config.ts`, `docs/tooling/commands.md`, `docs/tooling/documentation-site.md`, `docs/tooling/releases.md`, `package.json`, `vite.config.ts`.
- **docs(backlog): set US-002 ready**
  Product Owner agreement given in the session of 2026-10-08.
  Refs: US-002
  Files: `US-002`, `docs/backlog/stories/README.md`.
- **feat(render): create SVG canvas and path elements**
  - createSvgElement: \<svg> in the SVG namespace with a viewBox written at fixed precision; createPathElement: \<path> with its d attribute. - ViewBox type and SVG_NAMESPACE constant (REF-MDN-CREATEELEMENTNS). - happy-dom runs the render tests, keeping 100 % coverage of src/.
    Refs: US-002.T1
    Files: `docs/conventions/testing.md`, `docs/tooling/dependencies.md`, `package-lock.json`, `package.json`, `src/index.ts`, `src/render/create-path-element.test.ts`, `src/render/create-path-element.ts`, `src/render/create-svg-element.test.ts`, `src/render/create-svg-element.ts`, `src/render/index.ts`, `src/render/svg-namespace.ts`, `src/render/view-box.ts`.
- **feat(playground): draw a sharp rectangle from typed width and height**
  The playground draws the rectangle as one \<path> through the public API (model, contour, path data, render) and shows each pipeline stage. Sizes that are not integers >= 0 are refused: inputs outlined, message shown, drawing unchanged. "read" joins the procedure verbs. Checked by a throwaway happy-dom run of the real page: 120 x 80 drawn, 10.5 refused, 200 redrawn, a single \<path>.
  Refs: US-002.T2
  Files: `eslint/settings/verbs.ts`, `playground/index.html`, `playground/index.ts`.
- **docs(backlog): set US-002 done**
  Done on merge: the demonstration (T3) is the review of this pull request.
  Refs: US-002.T3; Closes: US-002
  Files: `US-002`, `docs/backlog/stories/README.md`.
- **docs(backlog): add US-004, fixed scale in the playground**
  Requested by the Product Owner after US-002: the automatic framing hid size changes. Set ready on request.
  Refs: US-004
  Files: `F01`, `US-004`, `docs/backlog/stories/README.md`.
- **feat(playground): draw shapes at a fixed scale**
  The view box takes the size of the drawing area in CSS pixels, so one user unit is one pixel: a rectangle twice as large is drawn twice as large. The canvas fills the page height and cuts shapes larger than itself.
  Refs: US-004.T1
  Files: `playground/index.html`, `playground/index.ts`.
- **feat(playground): keep the scale when the window is resized**
  The playground redraws on window resize: the view box follows the new size of the drawing area, shapes keep their size on screen. Checked by a throwaway happy-dom run with a simulated 800 x 600 then 400 x 300 canvas.
  Refs: US-004.T2
  Files: `playground/index.ts`.
- **docs(backlog): set US-004 done**
  Done on merge: the demonstration (T3) is the review of this pull request.
  Refs: US-004.T3; Closes: US-004
  Files: `F01`, `US-004`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.4.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set EN-003 ready**
  Product Owner agreement given in the session of 2026-10-08.
  Refs: EN-003
  Files: `F01`, `EN-003`, `docs/backlog/stories/README.md`.
- **docs(geometry): derive the turning angle at a contour vertex**
  DERIV-turning-angle: tau = atan2(perp dot, dot) of the incoming and outgoing edges, cyclic indices (Q11), sign convention in the SVG frame, check table and limits. Sources verified online: MathWorld (vector addition and difference, dot and perp dot products) and MDN (Math.atan2).
  Refs: EN-003.T1, Q11
  Files: `docs/derivations/README.md`, `docs/derivations/turning-angle.md`, `docs/references.md`.
- **feat(math): add vector subtraction, dot and perp dot products**
  - Vector type; subtract (REF-MATHWORLD-VECTOR-DIFFERENCE), dot (REF-MATHWORLD-DOT) and perpDot, the 2D cross product (REF-MATHWORLD-PERP-DOT), one file each. - The root barrel re-exports math as values now that it holds functions. Tests were written first (values computed by hand) and committed with the implementation, as the pre-commit hook requires a compiling tree.
    Refs: EN-003.T2, EN-003.T3
    Files: `src/index.ts`, `src/math/dot.test.ts`, `src/math/dot.ts`, `src/math/index.ts`, `src/math/perp-dot.test.ts`, `src/math/perp-dot.ts`, `src/math/subtract.test.ts`, `src/math/subtract.ts`, `src/math/vector.ts`.
- **test(tooling): fail when a barrel re-exports values as types only**
  The barrel autofix writes `export type *` for a type-only folder and never switches back once the folder holds values: the root barrel hid the math functions. barrels.test.ts loads every module re-exported as types only and fails if it exports a runtime value (mutation-checked on the old barrel).
  Refs: EN-003
  Files: `CLAUDE.md`, `barrels.test.ts`.
- **feat(geometry): compute the turning angle at each contour vertex**
  - turningAngle: tau = atan2(perp dot, dot) of the incoming and outgoing edges (DERIV-turning-angle); positive for a clockwise turn on screen. - turningAngles: one angle per vertex of a cyclic contour (Q11). - cyclicVertex: vertex at index mod n, with an explicit fallback for an empty contour (a reachable branch, so coverage stays at 100 %). Tests follow the derivation's check table and were written first.
    Refs: EN-003.T2, EN-003.T4, Q11
    Files: `src/geometry/cyclic-vertex.test.ts`, `src/geometry/cyclic-vertex.ts`, `src/geometry/index.ts`, `src/geometry/turning-angle.test.ts`, `src/geometry/turning-angle.ts`, `src/geometry/turning-angles.test.ts`, `src/geometry/turning-angles.ts`, `src/index.ts`.
- **docs(backlog): set EN-003 done**
  Done on merge: merging this pull request is the Product Owner's acceptance.
  Refs: EN-003; Closes: EN-003
  Files: `F01`, `EN-003`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.5.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set EN-004 ready**
  Product Owner agreement given in the session of 2026-10-08. Criteria completed from the derivation's check table.
  Refs: EN-004
  Files: `F01`, `EN-004`, `docs/backlog/stories/README.md`.
- **docs(geometry): derive the fillet setback from Euclid**
  REF-GG-FILLET could not be consulted online. DERIV-fillet-setback derives s = r tan(|tau|/2) with verified sources: Euclid I.32, III.16, III.18, IV.4 (D. E. Joyce's edition) and MathWorld (tangent), cross-checked with the road-design tangent length T = R tan(Delta/2). The clamp derivation, the domain and the docs now cite it.
  Refs: EN-004.T1
  Files: `docs/conventions/documentation.md`, `docs/derivations/README.md`, `docs/derivations/fillet-setback.md`, `docs/derivations/local-radius-clamp.md`, `docs/domain/shapes.md`, `docs/references.md`, `docs/research/README.md`.
- **feat(geometry): compute the fillet setback at a corner**
  filletSetback: s = r tan(|tau|/2) (DERIV-fillet-setback); 0 for aligned vertices or a radius of 0; the direction of the turn does not matter. Tests follow the derivation's check table and were written first.
  Refs: EN-004.T2, EN-004.T3
  Files: `src/geometry/fillet-setback.test.ts`, `src/geometry/fillet-setback.ts`, `src/geometry/index.ts`.
- **docs(backlog): set EN-004 done**
  Done on merge: merging this pull request is the Product Owner's acceptance.
  Refs: EN-004; Closes: EN-004
  Files: `F01`, `EN-004`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.6.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): open every feature with research and close it with an audit**
  ADR-0020: a research spike (SP) sources all the feature's stories before any code; an audit (AUD) re-derives the mathematics, recomputes expected values, mutation-checks each function, re-verifies the sources, checks provenance, duplicates and consistency; then the Product Owner validates (VAL). Regression and end-to-end tests come with the applications.
  Refs: ADR-0017, ADR-0020
  Files: `docs/adr/0020-feature-research-and-audit.md`, `docs/adr/README.md`, `docs/conventions/README.md`, `docs/conventions/audit.md`, `docs/conventions/testing.md`, `docs/research/README.md`.
- **docs(backlog): frame features with research, audit and validation**
  Feature frame and AUD story type in the method, spike and audit templates, F01 receives SP-001 (research for its remaining stories) and AUD-001. The backlog test fails when a feature lacks its spike, audit or validation (mutation-checked).
  Refs: ADR-0020, SP-001, AUD-001
  Files: `backlog.test.ts`, `docs/backlog/README.md`, `docs/backlog/_audit.md`, `docs/backlog/_spike.md`, `F01`, `AUD-001`, `SP-001`, `docs/backlog/stories/README.md`.
- **test(docs): check the bibliography for duplicates and malformed statuses**
  references.test.ts: unique REF identifiers, no URL under two identifiers, well-formed status cells (mutation-checked). Supports the audit's duplicate and source checks.
  Refs: ADR-0020
  Files: `references.test.ts`.
- **docs: reorganize CLAUDE.md and fix stale statements**
  CLAUDE.md follows the order of use (where to read, rules, feature and story lifecycles, definition of done, guardrails, pitfalls), states the current state instead of "src/ is empty", and links to the domain for settled questions. Stale statements fixed in conventions, commands, lint zones; the docs home links to the backlog and the playground.
  Refs: ADR-0020
  Files: `CLAUDE.md`, `docs/conventions/architecture.md`, `docs/conventions/lint.md`, `docs/index.md`, `docs/tooling/commands.md`.
- **chore(main): release 0.6.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: record agent and verification practices**
  Research note 0002: Claude Code documentation (best practices, memory, skills, hooks, subagents), context engineering, spec-driven development critiques, QuickCheck and fast-check, mutation testing at Google and the StrykerJS trial (blocked by stryker-js issue 6210 with Vitest 5), lychee trial. Thirteen references recorded, each read on 2026-10-08.
  Refs: ADR-0021, ADR-0022
  Files: `docs/references.md`, `docs/research/notes/0002-agent-and-verification-practices.md`, `docs/research/notes/README.md`.
- **docs(adr): adopt an agent working environment and property-based tests**
  ADR-0021: rules true everywhere stay in CLAUDE.md; procedures become skills, area pitfalls become path-scoped rules, an independent read-only auditor reviews each feature, hooks guard merges and Git hook bypasses and reload the project state at session start. ADR-0022: fast-check properties next to the hand-computed examples, weekly lychee link check, StrykerJS deferred. ADR-0020: date corrected to 2026-10-08.
  Refs: ADR-0020, ADR-0021, ADR-0022
  Files: `docs/adr/0020-feature-research-and-audit.md`, `docs/adr/0021-agent-working-environment.md`, `docs/adr/0022-property-based-testing-and-link-checking.md`, `docs/adr/README.md`.
- **build(deps): add fast-check for property-based tests**
  fast-check 4.10.2 and @fast-check/vitest 0.5.0, pinned, audit clean. Testing conventions describe properties (record form, seeds, Object.is and -0). StrykerJS stays under "considered for later" until stryker-js issue 6210 is fixed.
  Refs: ADR-0022
  Files: `docs/conventions/testing.md`, `docs/tooling/dependencies.md`, `package-lock.json`, `package.json`.
- **ci: check every Markdown link weekly with lychee**
  links.yml runs lychee-action v2 every Monday and on demand, over all Markdown including .claude/, templates excluded; local trial: 225 links OK.
  Refs: ADR-0022
  Files: `.github/workflows/links.yml`, `.lycheeignore`, `docs/tooling/ci.md`.
- **chore(tooling): configure the agent with skills, rules, auditor and hooks**
  Skills story, spike, derivation, audit; path-scoped rules for the library, Markdown and tooling; read-only auditor subagent; settings with routine permissions, a PreToolUse guard (merge of pull requests, Git hook bypass, force push without lease) and a SessionStart state report. Hooks are TypeScript, linted and tested (mutation-checked). CLAUDE.md shrinks from 151 to 118 lines; no rule lost (keyword comparison).
  Refs: ADR-0021
  Files: `.claude/agents/auditor.md`, `.claude/hooks/guard-bash.test.ts`, `.claude/hooks/guard-bash.ts`, `.claude/hooks/index.ts`, `.claude/hooks/session-state.test.ts`, `.claude/hooks/session-state.ts`, `.claude/rules/docs.md`, `.claude/rules/library.md`, `.claude/rules/tooling.md`, `.claude/settings.json`, `.claude/skills/audit/SKILL.md`, `.claude/skills/derivation/SKILL.md` and 10 more.
- **test(backlog): check every item in the table of its parent**
  The F01 story table still showed EN-001, US-001, EN-002 and US-002 as draft: the test only compared folder indexes. It now also compares the stories table of each feature and the features table of each epic; the four rows are fixed.
  Refs: ADR-0020
  Files: `backlog.test.ts`, `F01`.
- **docs(backlog): interview the Product Owner and use the auditor in audits**
  Refinement starts with an interview of the Product Owner; the audit template and AUD-001 use the auditor subagent and add the missing properties of the F01 functions merged before ADR-0022.
  Refs: ADR-0021, ADR-0022, AUD-001
  Files: `docs/backlog/README.md`, `docs/backlog/_audit.md`, `AUD-001`.
- **chore(main): release 0.6.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): update eslint-plugin-jsdoc to 65.2.1**
  Patch release flagged by the latest-version check.
  Files: `package-lock.json`, `package.json`.
- **docs(adr): inspect each feature with a demo and each epic with a review**
  ADR-0023: plain-language demo page and guided test at each feature validation; epic review (report, user test) and retrospective (evolvability review, playbook) at each epic; improvement log fed continuously. Markdown stays the backlog source of truth, GitHub Projects not adopted. Three references read on 2026-10-08 (Kanban guide, fitness functions, GitHub Projects).
  Refs: ADR-0023
  Files: `docs/adr/0023-inspection-cadence.md`, `docs/adr/README.md`, `docs/references.md`.
- **docs: add the process, guide and playbook sections**
  Process: cadences, evolvability review, improvement log, retrospective template. Guide: plain-language pages for the Product Owner, with demo and epic report templates. Playbook: principles, setup checklist, process, agent use and lessons, reusable for future projects. Sections added to the docs site.
  Refs: ADR-0023
  Files: `docs/.vitepress/config.ts`, `docs/guide/README.md`, `docs/guide/_epic-report.md`, `docs/guide/_feature-demo.md`, `docs/index.md`, `docs/playbook/README.md`, `docs/playbook/agent.md`, `docs/playbook/lessons.md`, `docs/playbook/principles.md`, `docs/playbook/process.md`, `docs/playbook/setup.md`, `docs/process/README.md` and 3 more.
- **test(backlog): close every started epic with a review and a retrospective**
  REV and RET stories belong to their epic and are listed in its Closing table; the test checks their presence and statuses (mutation-checked). E01 receives REV-001 and RET-001; VAL-001 gains its demo page task.
  Refs: ADR-0023, REV-001, RET-001, VAL-001
  Files: `backlog.test.ts`, `docs/backlog/README.md`, `docs/backlog/_retro.md`, `docs/backlog/_review.md`, `E01`, `VAL-001`, `RET-001`, `REV-001`, `docs/backlog/stories/README.md`.
- **chore(tooling): add the review and retro skills**
  /review writes feature demo pages and epic reports, /retro runs the retrospective and updates the playbook; /audit and the audit checklist gain a light evolvability check; CLAUDE.md lists the epic lifecycle and the new sections.
  Refs: ADR-0023
  Files: `.claude/rules/docs.md`, `.claude/skills/audit/SKILL.md`, `.claude/skills/retro/SKILL.md`, `.claude/skills/review/SKILL.md`, `CLAUDE.md`, `docs/conventions/audit.md`, `docs/tooling/agent.md`.
- **chore(main): release 0.6.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set F01 and E01 in progress**
  The Product Owner agreed on 2026-10-08. Rule written in the method: an epic or a feature is in progress from its first story, the agent proposes, the Product Owner agrees.
  Refs: F01, E01
  Files: `docs/backlog/README.md`, `E01`, `docs/backlog/epics/README.md`, `F01`, `docs/backlog/features/README.md`, `docs/process/improvements.md`.
- **docs: write the feedback of the Product Owner on their behalf**
  The Product Owner dictates by voice and never edits the repository: guide templates, process cadences, the review skill and the playbook say that the agent records their feedback, quoting them.
  Refs: ADR-0023
  Files: `.claude/skills/review/SKILL.md`, `docs/guide/README.md`, `docs/guide/_epic-report.md`, `docs/guide/_feature-demo.md`, `docs/playbook/agent.md`, `docs/process/README.md`.
- **docs(adr): keep a session journal instead of a database**
  ADR-0024: docs/process/journal.md records each session (requests, decisions, where recorded, #tags for grep), backfilled since 2026-10-07; the local transcripts are the full archive, searched with grep and jq; no database.
  Refs: ADR-0024
  Files: `docs/adr/0024-session-journal.md`, `docs/adr/README.md`, `docs/process/journal.md`, `docs/tooling/agent.md`.
- **docs: extend the writing rules**
  Voice (Microsoft and Google guides), word list including the French false friend contour, page skeletons, and a fixed shape for end-of-task messages to the Product Owner. Five references read on 2026-10-08.
  Refs: ADR-0023
  Files: `CLAUDE.md`, `docs/conventions/writing.md`, `docs/references.md`.
- **docs: add contributing guide, code of conduct and issue forms**
  Contributor Covenant 2.0 from the GitHub API, contact through the owner's profile; bug and idea issue forms, blank issues off; README with links to the guide, playground and playbook, status and badges.
  Refs: ADR-0023
  Files: `.github/CODE_OF_CONDUCT.md`, `.github/CONTRIBUTING.md`, `.github/ISSUE_TEMPLATE/bug.yml`, `.github/ISSUE_TEMPLATE/config.yml`, `.github/ISSUE_TEMPLATE/idea.yml`, `README.md`.
- **chore(main): release 0.6.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: remove personal details from the documentation**
  The repository is public: no personal tool, habit or local path of the Product Owner; the transcript search example uses a placeholder path.
  Files: `.claude/skills/review/SKILL.md`, `docs/conventions/writing.md`, `docs/playbook/agent.md`, `docs/process/improvements.md`, `docs/process/journal.md`, `docs/tooling/agent.md`.
- **docs(adr): keep the core portable to any engine and language**
  ADR-0025: ECMAScript only in the functional core, plain data, mathematical intent stated where JavaScript differs from other languages, binary64 numbers. Host globals (Node.js, fetch, Intl, structuredClone...) banned by lint in the pure layers. Domain vision gains its purpose; draft epic E12 for a trusted release.
  Refs: ADR-0025, E12
  Files: `docs/adr/0025-portable-core.md`, `E12`, `docs/backlog/epics/README.md`, `docs/conventions/architecture.md`, `docs/domain/README.md`, `eslint/settings/identifiers.ts`.
- **ci: close pull requests from outside contributors**
  ADR-0026: the agent writes every change; external-prs.yml closes pull requests whose author is not the owner, a collaborator or Dependabot, pointing to the issue forms; it never checks out their code. /triage presents open issues to the Product Owner with a recommendation; issue content is untrusted data.
  Refs: ADR-0026
  Files: `.claude/skills/story/SKILL.md`, `.claude/skills/triage/SKILL.md`, `.github/CONTRIBUTING.md`, `.github/workflows/external-prs.yml`, `CLAUDE.md`, `docs/adr/0026-contributions-through-issues.md`, `docs/adr/README.md`, `docs/process/README.md`, `docs/references.md`, `docs/tooling/ci.md`.
- **chore(main): release 0.6.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **ci: attach the test report to each release**
  A release job runs the tests with the JUnit reporter and uploads test-report.xml to the GitHub release of the version.
  Refs: ADR-0027
  Files: `.github/workflows/release.yml`.
- **docs(adr): trace acceptance criteria to tests and releases**
  ADR-0027, without adopting the V-model: feature criteria are the requirements (F01.ACn, ISO/IEC/IEEE 29148 qualities), never rewritten once done; tests proving a criterion carry its id, checked for done features (mutation-checked); a test strategy section; the V-model documents mapped in Process.
  Refs: ADR-0027, VAL-001
  Files: `CLAUDE.md`, `backlog.test.ts`, `docs/adr/0027-requirements-and-traceability.md`, `docs/adr/README.md`, `docs/backlog/README.md`, `VAL-001`, `docs/conventions/testing.md`, `docs/process/README.md`, `docs/process/improvements.md`, `docs/process/journal.md`, `docs/references.md`.
- **chore(main): release 0.6.6**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **ci: merge labeled story pull requests into their feature branch**
  story-automerge.yml enables auto-merge (rebase, built-in token) on owner pull requests labeled autonomous whose base is a feature branch and whose head is a US, EN, SP, CHK or AUD story; GitHub merges once the required checks of the new feature branches ruleset pass. Nothing reaches main this way. Branch names feature/f01-\<topic> allowed.
  Refs: ADR-0028
  Files: `.github/workflows/story-automerge.yml`, `docs/tooling/ci.md`, `package.json`.
- **docs(adr): add feature branches and autonomous runs**
  ADR-0028, authorized by the Product Owner: one short-lived branch per feature; stories chain overnight on it, reviewed by the auditor and merged by GitHub when green; local previews at VAL, then the feature pull request into main merged by the Product Owner, one release per feature. Git workflow, releases, method, cadences, playbook and CLAUDE.md updated; two GitHub references read on 2026-10-08.
  Refs: ADR-0028
  Files: `CLAUDE.md`, `docs/adr/0028-autonomous-runs.md`, `docs/adr/README.md`, `docs/backlog/README.md`, `docs/playbook/process.md`, `docs/process/README.md`, `docs/process/improvements.md`, `docs/process/journal.md`, `docs/references.md`, `docs/tooling/git-workflow.md`, `docs/tooling/releases.md`.
- **test(backlog): add the checkpoint story to the feature frame**
  CHK stories (docs and methods brought up to date mid-feature) are required in every feature, CHK-001 added to F01; user stories carry a Product Owner test card section.
  Refs: ADR-0028, CHK-001
  Files: `backlog.test.ts`, `docs/backlog/_checkpoint.md`, `docs/backlog/_story.md`, `F01`, `CHK-001`, `docs/backlog/stories/README.md`.
- **chore(tooling): add the run skill for autonomous runs**
  /run chains the stories of an authorized scope on the feature branch, with the auditor before each pull request and local previews at validation; /story targets the feature branch and writes test cards; /review compares the account of the Product Owner with the demo; /retro adds fresh research and the starter kit.
  Refs: ADR-0028
  Files: `.claude/skills/retro/SKILL.md`, `.claude/skills/review/SKILL.md`, `.claude/skills/run/SKILL.md`, `.claude/skills/story/SKILL.md`, `docs/tooling/agent.md`.
- **chore(main): release 0.6.7**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: record the start of the autonomous run of F01**
  Refs: ADR-0028, F01
  Files: `docs/process/journal.md`.
- **chore(main): release 0.6.8**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set SP-001 ready**
  Refs: SP-001
  Files: `F01`, `SP-001`, `docs/backlog/stories/README.md`.
- **docs(backlog): inventory the needs of the remaining F01 stories**
  Refs: SP-001.T1
  Files: `SP-001`.
- **docs(geometry): derive the tangent points, center and flags of a fillet**
  DERIV-fillet-arc: tangent points at the setback, center on the perpendicular at the tangent point (Euclid III.18) on the turning side, arc angle |tau| so large-arc-flag 0, sweep-flag 1 when tau > 0 (SVG positive-angle direction, y down). Check table computed by hand and cross-checked numerically. REF-SVG2-PATHS and REF-SVG2-IMPLNOTE verified; SVG coordinates, vector norm, unit vector and rotation matrix recorded.
  Refs: SP-001.T2
  Files: `docs/derivations/README.md`, `docs/derivations/fillet-arc.md`, `docs/references.md`.
- **docs(backlog): source the remaining F01 stories**
  Research note 0003; EN-005, EN-006 and EN-007 cite their sources, with exact expected values and path data from DERIV-fillet-arc; EN-007 is no longer blocked; EN-006 adds EPSILON and the vector helpers (5 points); US-003 gets its test card section and one question for the Product Owner.
  Refs: SP-001.T3
  Files: `EN-005`, `EN-006`, `EN-007`, `US-003`, `docs/research/notes/0003-f01-remaining-stories.md`, `docs/research/notes/README.md`.
- **docs(backlog): apply the auditor findings on SP-001**
  Per-edge factor written as in the derivation (S \<= L tested before dividing, no NaN on zero-length edges); EPSILON moved to EN-005, which compares 12.5 within it; the starting point of a rounded path stated in DERIV-fillet-arc step 7; research note number, VAL-001 inventory row and concave vertex fixed.
  Refs: SP-001.T2, SP-001.T3
  Files: `EN-005`, `EN-006`, `SP-001`, `docs/derivations/fillet-arc.md`, `docs/research/notes/0003-f01-remaining-stories.md`.
- **docs(backlog): set SP-001 done**
  Closes: SP-001
  Files: `F01`, `SP-001`, `docs/backlog/stories/README.md`.
- **docs(backlog): set EN-005 ready**
  Tasks split: EPSILON and norm, generic cyclic access, factor per edge, factor per vertex.
  Refs: EN-005
  Files: `F01`, `EN-005`, `docs/backlog/stories/README.md`.
- **feat(math): add EPSILON and the length of a vector**
  EPSILON = 1e-9 (Q10); norm(v) = sqrt(vx^2 + vy^2) through Math.hypot (REF-MATHWORLD-VECTOR-NORM), with hand-computed examples and properties (direction, triangle inequality).
  Refs: EN-005.T1
  Files: `src/math/epsilon.test.ts`, `src/math/epsilon.ts`, `src/math/index.ts`, `src/math/norm.test.ts`, `src/math/norm.ts`.

## 2026-10-07

- **ci: Bump actions/dependency-review-action from 4 to 5**
  Bumps \[actions/dependency-review-action](`https://github.com/actions/dependency-review-action`) from 4 to 5. - \[Release notes](`https://github.com/actions/dependency-review-action/releases`) - \[Commits](`https://github.com/actions/dependency-review-action/compare/v4...v5`) --- updated-dependencies: - dependency-name: actions/dependency-review-action dependency-version: '5' dependency-type: direct:production update-type: version-update:semver-major ...
  Files: `.github/workflows/ci.yml`.
- **ci: Bump github/codeql-action from 3 to 4**
  Bumps \[github/codeql-action](`https://github.com/github/codeql-action`) from 3 to 4. - \[Release notes](`https://github.com/github/codeql-action/releases`) - \[Changelog](`https://github.com/github/codeql-action/blob/main/CHANGELOG.md`) - \[Commits](`https://github.com/github/codeql-action/compare/v3...v4`) --- updated-dependencies: - dependency-name: github/codeql-action dependency-version: '4' dependency-type: direct:production update-type: version-update:semver-major ...
  Files: `.github/workflows/codeql.yml`.
- **ci: Bump actions/setup-node from 4 to 7**
  Bumps \[actions/setup-node](`https://github.com/actions/setup-node`) from 4 to 7. - \[Release notes](`https://github.com/actions/setup-node/releases`) - \[Commits](`https://github.com/actions/setup-node/compare/v4...v7`) --- updated-dependencies: - dependency-name: actions/setup-node dependency-version: '7' dependency-type: direct:production update-type: version-update:semver-major ...
  Files: `.github/workflows/ci.yml`.
- **ci: Bump googleapis/release-please-action from 4 to 5**
  Bumps \[googleapis/release-please-action](`https://github.com/googleapis/release-please-action`) from 4 to 5. - \[Release notes](`https://github.com/googleapis/release-please-action/releases`) - \[Changelog](`https://github.com/googleapis/release-please-action/blob/main/CHANGELOG.md`) - \[Commits](`https://github.com/googleapis/release-please-action/compare/v4...v5`) --- updated-dependencies: - dependency-name: googleapis/release-please-action dependency-version: '5' dependency-type: direct:production update-type: version-update:semver-major ...
  Files: `.github/workflows/release.yml`.
- **ci: Bump actions/checkout from 4 to 7**
  Bumps \[actions/checkout](`https://github.com/actions/checkout`) from 4 to 7. - \[Release notes](`https://github.com/actions/checkout/releases`) - \[Changelog](`https://github.com/actions/checkout/blob/main/CHANGELOG.md`) - \[Commits](`https://github.com/actions/checkout/compare/v4...v7`) --- updated-dependencies: - dependency-name: actions/checkout dependency-version: '7' dependency-type: direct:production update-type: version-update:semver-major ...
  Files: `.github/workflows/ci.yml`, `.github/workflows/codeql.yml`.

## 2026-10-09

- **build(deps): update happy-dom to 20.14.6**
  Refs: AUD-001
  Files: `package-lock.json`, `package.json`.
- **chore(tooling): wait for CI by reading its statuses**
  Product Owner request: poll the pull request state and checks every 15 s, stop at the first failing check and read its log at once, give up after 12 minutes; a patch published meanwhile is updated on the branch and on main. Lesson added to the playbook.
  Refs: ADR-0028
  Files: `.claude/skills/run/SKILL.md`, `.claude/skills/story/SKILL.md`, `docs/playbook/lessons.md`.
- **docs: record the stop of the F01 autonomous run**
  Refs: F01, VAL-001
  Files: `docs/process/journal.md`.
- **chore(main): release 0.6.9**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **refactor(geometry): generalize cyclicVertex into cyclicItem**
  Cyclic access serves vertices and per-vertex values (radii, setbacks, factors). The index is reduced with a non-negative modulo stated in the TSDoc rather than relying on the sign of the JavaScript remainder (ADR-0025). turningAngles, rules and notes updated.
  Refs: EN-005.T2
  Files: `.claude/rules/library.md`, `docs/process/improvements.md`, `docs/research/notes/0003-f01-remaining-stories.md`, `src/geometry/cyclic-item.test.ts`, `src/geometry/cyclic-item.ts`, `src/geometry/cyclic-vertex.test.ts`, `src/geometry/cyclic-vertex.ts`, `src/geometry/index.ts`, `src/geometry/turning-angles.ts`.
- **feat(geometry): compute setbacks, edge lengths and edge factors**
  Corner pairs a vertex with its requested radius. cornerSetbacks applies DERIV-local-radius-clamp step 1; edgeLengths, edgeFactor and edgeFactors step 2, testing S \<= L before dividing so that a zero-length edge gives 1. Tests from the check table, a zero-size rectangle and an aligned vertex; mutation of the comparison caught.
  Refs: EN-005.T3
  Files: `src/geometry/corner-setbacks.test.ts`, `src/geometry/corner-setbacks.ts`, `src/geometry/corner.ts`, `src/geometry/edge-factor.test.ts`, `src/geometry/edge-factor.ts`, `src/geometry/edge-factors.test.ts`, `src/geometry/edge-factors.ts`, `src/geometry/edge-lengths.test.ts`, `src/geometry/edge-lengths.ts`, `src/geometry/index.ts`.
- **feat(geometry): clamp corner radii locally and proportionally**
  vertexFactors takes the smaller factor of the two edges of a vertex; effectiveRadii composes the steps of DERIV-local-radius-clamp (ADR-0007). Criterion tests \[F01.AC3] (1000 on 100, 12.5 on an edge of 25, circle), zero size without NaN, and two properties on random rectangles: radii never increase nor become negative, setbacks of every edge fit its length.
  Refs: EN-005.T4
  Files: `src/geometry/effective-radii.test.ts`, `src/geometry/effective-radii.ts`, `src/geometry/index.ts`, `src/geometry/vertex-factors.test.ts`, `src/geometry/vertex-factors.ts`.
- **test(geometry): import Corner in the setback tests**
  The import was dropped by the unused-imports autofix before the signature used it; Vitest strips types, tsc caught it.
  Refs: EN-005.T3
  Files: `src/geometry/corner-setbacks.test.ts`.
- **fix(geometry): return no turn on a zero-length edge whatever the zeros**
  turningAngle gave pi when the outgoing edge had zero length after an incoming vector with negative components: atan2(+0, -0) = pi in IEEE 754. A zero-length edge has no direction, so the angle is 0 explicitly (ADR-0025), stated in the derivation and tested. Auditor findings also applied: factor ranges \[0, 1], independent setbacks in the overlap property (mutation-checked), AUD-001 list updated, question on the effective radius without arc recorded for the Product Owner in US-003.
  Refs: EN-005.T5
  Files: `AUD-001`, `EN-005`, `US-003`, `docs/derivations/turning-angle.md`, `src/geometry/edge-factor.ts`, `src/geometry/edge-factors.ts`, `src/geometry/effective-radii.test.ts`, `src/geometry/turning-angle.test.ts`, `src/geometry/turning-angle.ts`, `src/geometry/vertex-factors.ts`.
- **docs(backlog): set EN-005 done**
  Closes: EN-005
  Files: `F01`, `EN-005`, `docs/backlog/stories/README.md`.
- **docs(backlog): set EN-006 ready**
  Refs: EN-006
  Files: `F01`, `EN-006`, `docs/backlog/stories/README.md`.
- **feat(math): add, scale, normalize and turn vectors**
  add (REF-MATHWORLD-VECTOR-ADDITION), scale (Strang and Herman, OpenStax Calculus Volume 3, section 2.1, recorded as REF-OPENSTAX-CALC3-VECTORS), unit with (0, 0) for the zero vector (REF-MATHWORLD-UNIT-VECTOR) and quarterTurn (x, y) -> (-y, x) (REF-MATHWORLD-ROTATION-MATRIX). Examples and properties (difference, length, perpendicularity, direction).
  Refs: EN-006.T1
  Files: `docs/references.md`, `src/math/add.test.ts`, `src/math/add.ts`, `src/math/index.ts`, `src/math/quarter-turn.test.ts`, `src/math/quarter-turn.ts`, `src/math/scale.test.ts`, `src/math/scale.ts`, `src/math/unit.test.ts`, `src/math/unit.ts`.
- **feat(geometry): define segments and arcs of an evaluated contour**
  Segment and Arc (start, end, center, radius, direction), told apart by kind in ContourPiece (ADR-0005 stage 3, DERIV-fillet-arc).
  Refs: EN-006.T2
  Files: `src/geometry/arc.ts`, `src/geometry/contour-piece.ts`, `src/geometry/index.ts`, `src/geometry/segment.ts`.
- **feat(geometry): compute the fillet arc of a corner**
  filletArc gives the tangent points, center and direction of the fillet at a corner (DERIV-fillet-arc), degenerate at the vertex when no arc exists. CornerPoints groups the previous, corner and next vertices. Tests from the check table (rectangle, concave corner, aligned vertex, zero-length edge, a turn of pi/3); two mutations caught.
  Refs: EN-006.T3
  Files: `src/geometry/corner-points.ts`, `src/geometry/fillet-arc.test.ts`, `src/geometry/fillet-arc.ts`, `src/geometry/index.ts`.
- **feat(geometry): evaluate a rounded contour into segments and arcs**
  roundedContour clamps the radii, rounds every corner with its fillet arc and links the arcs with segments, starting after the first corner (DERIV-fillet-arc step 7); hasLength drops pieces shorter than EPSILON. Tests: rectangle with radius 10, circle \[F01.AC3], sharp contour, zero size, empty; property: a closed chain on random rectangles. Mutation of hasLength caught.
  Refs: EN-006.T4
  Files: `src/geometry/has-length.ts`, `src/geometry/index.ts`, `src/geometry/rounded-contour.test.ts`, `src/geometry/rounded-contour.ts`.
- **docs(geometry): state the limits of rounded contours found by the audit**
  A spike (turning angle +-pi) is consumed by its fillet: documented as a limit of DERIV-fillet-arc and raised as open question Q16 for the Product Owner. The Arc radius contract allows degenerate arcs from filletArc; pieces meet within EPSILON and the drop rule is stated in the derivation; citations of unit and add and the references aligned.
  Refs: EN-006.T5
  Files: `EN-006`, `docs/derivations/fillet-arc.md`, `docs/domain/README.md`, `docs/references.md`, `src/geometry/arc.ts`, `src/geometry/fillet-arc.ts`, `src/geometry/rounded-contour.ts`, `src/math/add.ts`, `src/math/unit.ts`.
- **docs(backlog): set EN-006 done**
  Closes: EN-006
  Files: `F01`, `EN-006`, `docs/backlog/stories/README.md`.
- **docs(backlog): set CHK-001 ready**
  Refs: CHK-001
  Files: `F01`, `CHK-001`, `docs/backlog/stories/README.md`.
- **docs: bring rules, glossary and lessons up to date mid-F01**
  Glossary: tangent point, effective radius, spike. Library rules: tsc before each commit, signed zeros and zero vectors, parameter objects. Playbook lessons from EN-005 and EN-006 audits.
  Refs: CHK-001.T1
  Files: `.claude/rules/library.md`, `docs/domain/README.md`, `docs/playbook/lessons.md`.
- **docs(backlog): record the light evolvability check of F01**
  Six answers with figures in CHK-001; three items for the retrospective in the improvement log (shared test fixtures, CornerPoints for turningAngle, a fitness function for property tests).
  Refs: CHK-001.T2
  Files: `CHK-001`, `docs/process/improvements.md`.
- **docs: apply the auditor findings on CHK-001**
  Test \[F01.AC3] for radius 1000 on every corner of a square of 100 (the same circle), plan updated; knip trigger reached and logged; exact figures in the evolvability check; DERIV-fillet-arc step 1 aligned with the explicit zero-edge rule; hasLength TSDoc, testing conventions (EPSILON) and the spike question Q16 in shapes.md made consistent.
  Refs: CHK-001.T1, CHK-001.T2
  Files: `F01`, `CHK-001`, `docs/conventions/testing.md`, `docs/derivations/fillet-arc.md`, `docs/domain/shapes.md`, `docs/process/improvements.md`, `src/geometry/effective-radii.test.ts`, `src/geometry/has-length.ts`.
- **docs(backlog): set CHK-001 done**
  Closes: CHK-001
  Files: `F01`, `CHK-001`, `docs/backlog/stories/README.md`.
- **docs(backlog): set EN-007 ready**
  Refs: EN-007
  Files: `F01`, `EN-007`, `docs/backlog/stories/README.md`.
- **feat(io): write the arc command of a fillet**
  formatCoordinatePair is shared (moved out of contourToPathData); formatArcCommand writes A r r 0 0 sweep x y (REF-SVG2-PATHS 9.3.8): no rotation, small arc, sweep 1 in the positive direction (DERIV-fillet-arc steps 4-5), numbers at fixed precision. Swapping the sweep flags is caught.
  Refs: EN-007.T1
  Files: `src/io/contour-to-path-data.ts`, `src/io/format-arc-command.test.ts`, `src/io/format-arc-command.ts`, `src/io/format-coordinate-pair.test.ts`, `src/io/format-coordinate-pair.ts`, `src/io/index.ts`.
- **feat(io): write the path data of a rounded contour**
  contourPiecesToPathData writes M, then L for segments and A for arcs, then Z; a final segment is left to closepath, so radius 0 gives exactly the sharp path data of EN-002 (property on random rectangles). Criterion strings of EN-007 for the 100 x 50 rectangle with radius 10 and the circle, plus a mixed contour. Rule stated in DERIV-fillet-arc; mutation caught.
  Refs: EN-007.T2
  Files: `docs/derivations/fillet-arc.md`, `src/io/contour-pieces-to-path-data.test.ts`, `src/io/contour-pieces-to-path-data.ts`, `src/io/index.ts`.
- **docs(geometry): qualify the closepath equality for zero-length edges**
  Auditor finding: with radius 0 the rounded path data equals the sharp one only for edges of positive length; a 0 x 50 rectangle is written M0 0 L0 50 Z. Stated in DERIV-fillet-arc and tested.
  Refs: EN-007.T2
  Files: `docs/derivations/fillet-arc.md`, `src/io/contour-pieces-to-path-data.test.ts`.
- **docs(backlog): set EN-007 done**
  Closes: EN-007
  Files: `F01`, `EN-007`, `docs/backlog/stories/README.md`.
- **docs(backlog): set US-003 ready**
  Refs: US-003
  Files: `F01`, `US-003`, `docs/backlog/stories/README.md`.
- **feat(model): give the rectangle a global corner radius**
  The requested radius is stored (integer >= 0, validated, a radius larger than the rectangle stays valid); rectangleCorners pairs each vertex with it; effectiveCornerRadius derives the radius actually drawn (Q8, ADR-0007). Tests \[F01.AC2]: radius 10 on 100 x 50, 12.5 for 100 on 100 x 25, rounding following the size; property: never above the requested radius, never shrinking when the rectangle widens. The playground keeps radius 0 until T2.
  Refs: US-003.T1
  Files: `playground/index.ts`, `src/model/effective-corner-radius.test.ts`, `src/model/effective-corner-radius.ts`, `src/model/index.ts`, `src/model/is-valid-rectangle.test.ts`, `src/model/is-valid-rectangle.ts`, `src/model/rectangle-contour.test.ts`, `src/model/rectangle-corners.test.ts`, `src/model/rectangle-corners.ts`, `src/model/rectangle.ts`.
- **feat(playground): round the rectangle corners with a radius input**
  Radius input next to width and height; the effective radius is shown and said to be reduced to fit when it differs from the requested one (Q8); the path is the rounded contour; the panel shows the evaluated segments and arcs. Checked with a smoke run in happy-dom (120 x 80 radius 10, 100 x 25 radius 100 -> 12.5, invalid radius), which caught a radius never read; logged as a future permanent test.
  Refs: US-003.T2
  Files: `docs/process/improvements.md`, `playground/index.html`, `playground/index.ts`.
- **docs(backlog): write the Product Owner test card of US-003**
  Nine steps in plain words, from the rounded rectangle to the clamped pill and the circle, with the words radius, requested and effective radius explained; run at VAL-001.
  Refs: US-003.T3
  Files: `US-003`.
- **fix(playground): clear the effective radius of a refused input**
  Auditor findings on US-003: the effective radius of the last valid rectangle stayed displayed next to an invalid input (cleared now, smoke-checked); step 3 of the test card typed the wrong radius (radius 100 now); the size-0 question becomes open question Q17 of the domain; shapes.md describes the global radius of the rectangle and its validation; glossary gains the requested radius.
  Refs: US-003.T2, US-003.T3
  Files: `US-003`, `docs/domain/README.md`, `docs/domain/shapes.md`, `playground/index.ts`.
- **docs(backlog): set US-003 done**
  Closes: US-003
  Files: `F01`, `US-003`, `docs/backlog/stories/README.md`.
- **docs(backlog): set AUD-001 ready**
  Refs: AUD-001
  Files: `F01`, `AUD-001`, `docs/backlog/stories/README.md`.
- **test: add properties to the F01 functions merged before ADR-0022**
  subtract, dot, perpDot (symmetry, antisymmetry, squared length), turningAngle (range, translation), filletSetback (sign, parity, linearity), cyclicItem (whole turn), edgeLengths (perimeter), edgeFactor (range, fit), vertexFactors (bounded by both edges).
  Refs: AUD-001.T1
  Files: `src/geometry/cyclic-item.test.ts`, `src/geometry/edge-factor.test.ts`, `src/geometry/edge-lengths.test.ts`, `src/geometry/fillet-setback.test.ts`, `src/geometry/turning-angle.test.ts`, `src/geometry/vertex-factors.test.ts`, `src/math/dot.test.ts`, `src/math/perp-dot.test.ts`, `src/math/subtract.test.ts`.
- **test: close the test gaps found by the F01 audit**
  Tags \[F01.AC4] on precision and orientation; properties for formatSvgNumber (5 decimals, half a unit), rectangleContour (positive signed area, clockwise on screen) and filletArc (center at the radius); a rounding-up example; a test file for hasLength at the EPSILON boundary (mutation >= to > now caught); the chain property tightened to EPSILON. Mutation spot-checks: 22 of 24 caught, 2 equivalent in JavaScript.
  Refs: AUD-001.T1
  Files: `src/geometry/fillet-arc.test.ts`, `src/geometry/has-length.test.ts`, `src/io/format-svg-number.test.ts`, `src/model/rectangle-contour.test.ts`.
- **docs(geometry): cite the sources of every derivation step**
  Audit B of F01: the components of the perp vector come from the rotation matrix, not from the perp-dot page; scalar multiplication in the tangent points cites OpenStax; REF-FIGMA-CR re-verified and dated. All sources cited by src/ re-opened on 2026-10-09; 86 links of the bibliography answer.
  Refs: AUD-001.T2
  Files: `docs/derivations/fillet-arc.md`, `docs/derivations/turning-angle.md`, `docs/references.md`.
- **docs: align the domain and the feature plan with what F01 built**
  Audit E of F01: shapes.md states where the rounded path starts, open question Q17 and the concave limit of the clamp; formatSvgNumber states its rounding of ties (portability); the feature plan maps each criterion to the stories and tagged tests of today. Duplicates D1, D2 and undated sources B2 logged for the retrospective.
  Refs: AUD-001.T3
  Files: `F01`, `docs/domain/shapes.md`, `docs/process/improvements.md`, `src/io/format-svg-number.ts`.
- **docs(backlog): record the findings of the F01 audit**
  Findings table (A to F with evidence and actions), sources and questions for F02, light evolvability check.
  Refs: AUD-001.T4
  Files: `AUD-001`.
- **docs(backlog): set AUD-001 done**
  Closes: AUD-001
  Files: `F01`, `AUD-001`, `docs/backlog/stories/README.md`.
- **docs(backlog): set VAL-001 ready**
  Tasks: criterion tests with a playground test, guided test of the nine steps in the playground (Product Owner request), demo page, validation.
  Refs: VAL-001
  Files: `F01`, `VAL-001`, `docs/backlog/stories/README.md`.
- **test(playground): test the playground page in happy-dom**
  The logic moves to mount-playground.ts (index.ts only mounts it), so that the test loads the page as served (index.html?raw, vite/client types), mounts it, types values and reads the outputs: \[F01.AC1] one path, \[F01.AC2] effective radius shown as reduced or as requested, refused input clears it.
  Refs: VAL-001.T1
  Files: `docs/conventions/testing.md`, `docs/process/improvements.md`, `playground/index.ts`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`, `tsconfig.json`, `vite.config.ts`.
- **feat(playground): guide the Product Owner through the nine test steps**
  A guided test panel shows one step of the US-003 card at a time, what happens in plain words and what to look at; Previous and Next type the step values; a Words box explains radius, requested and effective. Tested: the nine steps show what they announce.
  Refs: VAL-001.T2
  Files: `playground/guided-step.ts`, `playground/guided-steps.ts`, `playground/index.html`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`.
- **docs: explain F01 in plain language in the guide**
  Demo page: what you can do, fillet, tangent point and setback, the radius reduced to fit, the circle as a rounded square, a word list, how to run the guided test, limits. Four pictures drawn by the library itself.
  Refs: VAL-001.T3
  Files: `docs/guide/README.md`, `docs/guide/f01-rounded-rectangle.md`, `docs/guide/images/f01-circle.svg`, `docs/guide/images/f01-corner.svg`, `docs/guide/images/f01-pill.svg`, `docs/guide/images/f01-radius-10.svg`.
- **docs: settle Q17, keep the radius where nothing is rounded**
  Product Owner, 2026-10-09: the requested radius stays on the vertex and is shown as requested; it applies as soon as the shape grows.
  Refs: VAL-001, US-003
  Files: `US-003`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **docs: settle Q16, a spike is consumed by its fillet**
  Product Owner, 2026-10-09: rounding may eat length, so the current behavior is the intended one. Domain, glossary, derivation, TSDoc and guide aligned.
  Refs: VAL-001
  Files: `docs/derivations/fillet-arc.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/guide/f01-rounded-rectangle.md`, `src/geometry/fillet-arc.ts`, `src/geometry/rounded-contour.ts`.
- **docs(backlog): record the Product Owner validation of F01**
  Nine steps of the guided test run and understood as intended, quoted; no gap. Remarks routed: radius per vertex by clicking (F03), browser SVG measurements for end-to-end tests (E12) and specialized expert agents (E01 retrospective), in the improvement log.
  Refs: VAL-001.T4
  Files: `F03`, `VAL-001`, `docs/guide/f01-rounded-rectangle.md`, `docs/process/improvements.md`.
- **docs(backlog): set VAL-001 and F01 done**
  The Product Owner validated F01 on the guided test; every criterion is traced to a tagged test (backlog test). Merging the feature pull request is the acceptance.
  Closes: VAL-001; Closes: F01
  Files: `E01`, `F01`, `docs/backlog/features/README.md`, `VAL-001`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.7.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **chore(tooling): keep what the validation of F01 taught**
  /run without argument takes the feature in progress; the validation prepares criterion tests, a guided test inside the playground and the demo page, lets the Product Owner launch it, opens one feature pull request from the VAL branch and deletes every branch through the API. /review, rules, Git workflow and CLAUDE.md state updated; journal and playbook lessons recorded.
  Refs: ADR-0028, VAL-001
  Files: `.claude/rules/tooling.md`, `.claude/skills/review/SKILL.md`, `.claude/skills/run/SKILL.md`, `CLAUDE.md`, `docs/playbook/lessons.md`, `docs/process/journal.md`, `docs/tooling/git-workflow.md`.
- **build(deps): eslint-plugin-jsdoc 65.2.2**
  Files: `package-lock.json`, `package.json`.
- **chore(tooling): write the feature branch pattern for any feature**
  The skills, CLAUDE.md and the Git workflow named feature/f01-\<topic>; they now name feature/f\<nn>-\<topic>. Journal: start of the F02 run.
  Files: `.claude/skills/run/SKILL.md`, `.claude/skills/story/SKILL.md`, `CLAUDE.md`, `docs/process/journal.md`, `docs/tooling/git-workflow.md`.
- **chore(main): release 0.7.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): refine F02 with the Product Owner**
  Q18 settled (natural starting vertex), Q19 open (largest number of corners, researched in SP-002); maximal radius accepted; shape selector in the playground. F02 criteria rewritten, eight draft stories, plan.
  Files: `F02`, `AUD-002`, `CHK-002`, `EN-008`, `SP-002`, `US-005`, `US-006`, `US-007`, `VAL-002`, `docs/backlog/stories/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md` and 1 more.
- **chore(main): release 0.7.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set F02 in progress and its stories ready**
  The Product Owner relaunched /run F02 after being asked to agree.
  Refs: F02
  Files: `E01`, `F02`, `docs/backlog/features/README.md`, `AUD-002`, `CHK-002`, `EN-008`, `SP-002`, `US-005`, `US-006`, `US-007`, `VAL-002`, `docs/backlog/stories/README.md` and 1 more.
- **chore(main): release 0.7.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the stop of the F02 run at Q19**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.7.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): @types/node 26.6.5**
  Files: `package-lock.json`, `package.json`.
- **chore(main): release 0.7.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(tooling): check that GitHub Actions and Node.js are up-to-date**
  npm run deps:tools, in check:all: fails when a workflow action has a newer major version or .nvmrc is not the latest Node.js LTS major. The Product Owner asked for every tool, not only npm packages.
  Files: `.github/workflows/ci.yml`, `CLAUDE.md`, `docs/process/journal.md`, `docs/tooling/commands.md`, `docs/tooling/versions-and-security.md`, `eslint/settings/files.ts`, `package.json`, `scripts/index.ts`, `scripts/tool-versions.test.ts`, `scripts/tool-versions.ts`, `tsconfig.json`, `vite.config.ts`.
- **chore(main): release 0.7.6**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: log the stop of the F02 run at VAL-002**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.7.7**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: record the Product Owner's view on Q20**
  Files: `docs/process/improvements.md`, `docs/process/journal.md`.
- **chore(main): release 0.7.8**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): draft F06, a polygon stretched to fill its box**
  The Product Owner's idea during F02, kept apart from the regular polygon (Q1, uniform mode).
  Files: `E01`, `F06`, `docs/backlog/features/README.md`, `docs/process/journal.md`.
- **chore(main): release 0.7.9**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): record the Product Owner's idea for SVG import (E11)**
  Files: `E11`, `docs/process/improvements.md`, `docs/process/journal.md`.
- **chore(main): release 0.7.10**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): keep fixed and stretched polygons as two kinds (F06)**
  Files: `F06`.
- **chore(main): release 0.7.11**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): inventory what the stories of F02 need**
  Refs: SP-002.T1
  Files: `F02`, `SP-002`, `docs/backlog/stories/README.md`.
- **docs(geometry): source the regular polygon fit and its maximal rounding**
  Vertices from the roots of unity, drawing order and starting vertex (Q11, Q18), placement formulas, and the proof that the maximal rounding is the incircle. Inkscape's tool read; MathWorld, OpenStax, ISA rows.
  Refs: SP-002.T2
  Files: `SP-002`, `docs/derivations/regular-polygon-fit.md`, `docs/references.md`.
- **docs(docs): ask which polygons process symbols use (Q19)**
  The symbol standards are paid; what could be read is recorded, and the precise question is ready for a research in claude.ai, if wanted.
  Refs: SP-002.T3
  Files: `SP-002`, `docs/domain/README.md`, `docs/research/requests/0004-polygons-in-process-symbols.md`.
- **docs(docs): write the research note of F02 and update its stories**
  Refs: SP-002.T4
  Files: `EN-008`, `SP-002`, `US-005`, `US-006`, `docs/research/notes/0004-f02-regular-polygon.md`, `docs/research/notes/README.md`.
- **docs(geometry): apply the auditor's review of SP-002**
  The incircle's center is on the bisector by Euclid I.47 and I.8 (IV.4 proves the converse); the ISA diamond's geometry is not stated; path data measured at 50 to 60 characters per corner; one more checkable case in US-006.
  Refs: SP-002.T2
  Files: `US-006`, `docs/derivations/regular-polygon-fit.md`, `docs/references.md`, `docs/research/notes/0004-f02-regular-polygon.md`, `docs/research/requests/0004-polygons-in-process-symbols.md`.
- **docs(docs): settle Q19 at twelve corners after research 0004**
  The Product Owner ran research 0004 in claude.ai and chose 12. Symbols use up to 8 sides; other orientations come from rotation (F04).
  Refs: SP-002.T3
  Files: `F02`, `F04`, `US-005`, `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/references.md`, `docs/research/notes/0004-f02-regular-polygon.md`, `docs/research/requests/0004-polygons-in-process-symbols.md`.
- **docs(backlog): set SP-002 done**
  Closes: SP-002
  Files: `F02`, `SP-002`, `docs/backlog/stories/README.md`.
- **feat(geometry): unit regular polygon with a flat base**
  unitRegularPolygon(n): n vertices on the unit circle, clockwise on screen from the topmost one, the leftmost on a tie (Q11, Q18), chosen by index. Hand-computed triangle, square, hexagon; +0 rather than -0.
  Refs: EN-008.T1
  Files: `EN-008`, `src/geometry/index.ts`, `src/geometry/unit-regular-polygon.test.ts`, `src/geometry/unit-regular-polygon.ts`.
- **test(geometry): properties of the unit regular polygon**
  For n from 3 to 64: n vertices on the unit circle a chord 2 sin(pi/n) apart, clockwise (shoelace), topmost then leftmost start, flat base at cos(pi/n). Mutations of the start, the order and the y flip are caught.
  Refs: EN-008.T2
  Files: `EN-008`, `src/geometry/unit-regular-polygon.test.ts`.
- **docs(geometry): apply the auditor's review of EN-008**
  The derivation says the unit polygon is handed over already flipped to the SVG frame and gives step 5 in flipped ordinates; the signed-zero intent covers both coordinates; the regularity property traces F02.AC1.
  Refs: EN-008.T2
  Files: `EN-008`, `docs/derivations/regular-polygon-fit.md`, `src/geometry/unit-regular-polygon.test.ts`, `src/geometry/unit-regular-polygon.ts`.
- **docs(backlog): set EN-008 done**
  Closes: EN-008
  Files: `F02`, `EN-008`, `docs/backlog/stories/README.md`.
- **feat(geometry): bounding box and uniform fit in a box**
  boundingBox(points) and fitInBox(points, width, height): the largest uniform scale fitting the box, centered (DERIV-regular-polygon-fit steps 3-5); a box of size 0 collapses to its center. Mutations caught.
  Refs: US-005.T1
  Files: `US-005`, `src/geometry/bounding-box.test.ts`, `src/geometry/bounding-box.ts`, `src/geometry/fit-in-box.test.ts`, `src/geometry/fit-in-box.ts`, `src/geometry/index.ts`.
- **feat(model): regular polygon fitted in its box**
  RegularPolygon (corners, width, height, radius), corners from 3 to 12 (Q19), isValidRegularPolygon, regularPolygonContour = fitInBox of the unit polygon. Triangle, square, hexagon, octagon checked by hand.
  Refs: US-005.T2
  Files: `US-005`, `src/model/index.ts`, `src/model/is-valid-regular-polygon.test.ts`, `src/model/is-valid-regular-polygon.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-contour.ts`, `src/model/regular-polygon-corner-range.ts`, `src/model/regular-polygon.ts`.
- **docs(geometry): name the code of the polygon fit**
  Refs: US-005.T3
  Files: `US-005`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/shapes.md`.
- **refactor(model): one rule for model sizes; apply the US-005 review**
  isModelSize (integer >= 0) shared by the rectangle and the polygon validators; polygon parameters and Q15 stated for every shape in the domain; height-0 polygon tested; fitInBox tolerance stated honestly.
  Refs: US-005.T3
  Files: `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/process/improvements.md`, `src/geometry/fit-in-box.ts`, `src/model/index.ts`, `src/model/is-model-size.test.ts`, `src/model/is-model-size.ts`, `src/model/is-valid-rectangle.ts`, `src/model/is-valid-regular-polygon.ts`, `src/model/regular-polygon-contour.test.ts`.
- **docs(backlog): set US-005 done**
  Closes: US-005
  Files: `F02`, `US-005`, `docs/backlog/stories/README.md`.
- **docs: bring code, domain and agent configuration up to date (CHK-002)**
  Derived vertices may be fractional (Corner, shapes, glossary); CLAUDE.md state; tooling rule names deps:tools.
  Refs: CHK-002.T1
  Files: `.claude/rules/tooling.md`, `CLAUDE.md`, `F02`, `CHK-002`, `US-006`, `docs/backlog/stories/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `src/geometry/corner.ts`.
- **docs: light evolvability check of F02; review before the done commit**
  Refs: CHK-002.T2
  Files: `.claude/skills/run/SKILL.md`, `CHK-002`, `docs/process/improvements.md`.
- **docs: apply the auditor's review of CHK-002**
  Settled questions up to Q19 in CLAUDE.md; /run orders the review before the done commit in step 2 too; evolvability facts corrected.
  Refs: CHK-002.T2
  Files: `.claude/skills/run/SKILL.md`, `CLAUDE.md`, `CHK-002`.
- **docs(backlog): set CHK-002 done**
  Closes: CHK-002
  Files: `F02`, `CHK-002`, `docs/backlog/stories/README.md`.
- **feat(model): round the corners of a regular polygon**
  regularPolygonCorners gives every vertex the requested radius; the F01 clamp does the rest. effectiveCornerRadius now takes corners, shared by the rectangle and the polygon. At the maximal radius the arcs share one center: the incircle (hexagon 43.30127, triangle 28.86751).
  Refs: US-006.T1
  Files: `US-006`, `playground/mount-playground.ts`, `src/model/effective-corner-radius.test.ts`, `src/model/effective-corner-radius.ts`, `src/model/index.ts`, `src/model/regular-polygon-corners.test.ts`, `src/model/regular-polygon-corners.ts`.
- **docs(geometry): the polygon is rounded by the clamp of F01**
  Refs: US-006.T2
  Files: `US-006`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/shapes.md`.
- **test(model): count the arcs of the rounded triangle; apply the US-006 review**
  The triangle test asserted centers in a loop that passes when empty. Large-box rounding residue logged for the Product Owner.
  Refs: US-006.T2
  Files: `US-006`, `docs/process/improvements.md`, `src/model/regular-polygon-corners.test.ts`.
- **docs(backlog): set US-006 done**
  Closes: US-006
  Files: `F02`, `US-006`, `docs/backlog/stories/README.md`.
- **feat(playground): choose rectangle or polygon**
  A shape selector; the Corners field shows for the polygon only; width, height and radius are shared; refused values explained (Q15, Q19); the F01 guided test sets the rectangle back. Tested in happy-dom.
  Refs: US-007.T1
  Files: `US-007`, `playground/index.html`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`.
- **docs(backlog): write the Product Owner test card of US-007**
  Refs: US-007.T2
  Files: `US-007`.
- **fix(playground): hide the Corners field for real; apply the US-007 review**
  label { display: block } overrode the hidden attribute: the field never disappeared in a browser. A \[hidden] rule fixes it; the tests load the page head and check the computed display (mutation caught). Opening, rounded path data and refusals of 2 and empty corners tested.
  Refs: US-007.T1
  Files: `.claude/rules/library.md`, `docs/process/improvements.md`, `playground/index.html`, `playground/mount-playground.test.ts`.
- **docs(backlog): set US-007 done**
  Closes: US-007
  Files: `F02`, `US-007`, `docs/backlog/stories/README.md`.
- **docs: audit of F02, breadth and next feature; links refused to robots**
  Standards publishers (IEC, ANSI, ISO) answer 403 to the link checker as to every automated client: excluded in .lycheeignore with the reason.
  Refs: AUD-002.T4
  Files: `.lycheeignore`, `F02`, `AUD-002`, `docs/backlog/stories/README.md`.
- **test(model): audit of F02, mathematics and missing cases**
  Pentagon (odd n above 3), 12 corners, rounded polygon of size 0, equal effective radii on every corner (property), output decimals \[F02.AC3]; the fit's rounding error stated relative to the box.
  Refs: AUD-002.T1
  Files: `docs/derivations/regular-polygon-fit.md`, `playground/mount-playground.test.ts`, `src/geometry/fit-in-box.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **docs: audit of F02, sources**
  What could not be re-read (IEC, ANSI, ISO refuse automated clients) is stated consistently; the note's pre-research line is marked superseded.
  Refs: AUD-002.T2
  Files: `.lycheeignore`, `docs/research/notes/0004-f02-regular-polygon.md`.
- **docs: audit of F02, consistency**
  Playground help no longer says the curve starts r before every corner (true on a rectangle only); feature plan and testing convention list the F02 tests.
  Refs: AUD-002.T3
  Files: `F02`, `docs/conventions/testing.md`, `playground/index.html`.
- **docs(backlog): findings of the audit of F02; Q20 asked**
  Refs: AUD-002.T4
  Files: `AUD-002`, `docs/domain/README.md`.
- **docs(backlog): set AUD-002 done**
  Closes: AUD-002
  Files: `F02`, `AUD-002`, `docs/backlog/stories/README.md`.
- **docs(backlog): add US-008, cap the playground sizes at 100 000**
  Asked by the Product Owner when settling Q20; criterion F02.AC5.
  Refs: US-008
  Files: `F02`, `US-008`, `docs/backlog/stories/README.md`.
- **feat(playground): cap width, height and radius at 100 000**
  A value typed above 100 000 becomes 100 000 in the input; smaller values are kept exactly. The library keeps no upper limit (Q20).
  Refs: US-008.T1
  Files: `US-008`, `playground/index.html`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`.
- **docs: settle Q20, no upper limit in the calculations**
  One unit is one screen pixel; the interface caps at 100 000; EPSILON stays absolute (Product Owner, 2026-10-09).
  Refs: US-008.T2
  Files: `US-008`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **docs: apply the auditor's review of US-008**
  Rounding error stated as about 1e-16 times the size; safe integers; height cap and library without limit tested; delivery order; VAL-002 covers F02.AC5; settled questions up to Q20.
  Refs: US-008.T2
  Files: `CLAUDE.md`, `F02`, `VAL-002`, `docs/backlog/stories/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/process/improvements.md`, `playground/mount-playground.test.ts`, `src/model/is-valid-rectangle.test.ts`.
- **docs(backlog): set US-008 done**
  Closes: US-008
  Files: `F02`, `US-008`, `docs/backlog/stories/README.md`.
- **docs: merge the improvement log of main and F02 once each**
  The rebase of the feature branch on main had repeated rows.
  Files: `docs/process/improvements.md`.
- **docs(backlog): list the tests of each F02 criterion**
  Refs: VAL-002.T1
  Files: `F02`, `VAL-002`, `docs/backlog/stories/README.md`.
- **feat(playground): guided test of F02**
  Ten steps of the US-007 card and the cap of US-008 after the nine of F01; each step sets the shape and the number of corners.
  Refs: VAL-002.T2
  Files: `VAL-002`, `playground/guided-step.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`.
- **docs(docs): demo page of F02, the regular polygon**
  The polygon fitted in its box, the inscribed circle, sizes (Q20), the guided test; pictures drawn by the library.
  Refs: VAL-002.T3
  Files: `docs/guide/README.md`, `docs/guide/f02-regular-polygon.md`, `docs/guide/images/f02-hexagon-incircle.svg`, `docs/guide/images/f02-hexagon.svg`, `docs/guide/images/f02-octagon.svg`, `docs/guide/images/f02-square-in-100x50.svg`, `docs/guide/images/f02-triangle-incircle.svg`, `docs/guide/images/f02-triangle.svg`.
- **docs(backlog): set VAL-002 and F02 done**
  The Product Owner ran the playground and validated F02; account and remarks recorded in VAL-002.
  Closes: VAL-002; Closes: F02
  Files: `CLAUDE.md`, `E01`, `F02`, `docs/backlog/features/README.md`, `VAL-002`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.8.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs: log the release of F02; state of the project**
  Files: `CLAUDE.md`, `docs/process/journal.md`.
- **chore(main): release 0.8.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.

## 2026-10-10

- **docs(backlog): draft F07, a polygon aligned in its box**
  Idea of the Product Owner after F02: the flat base touches the bottom of the box by default, with a vertical (top, center, bottom) and a horizontal (left, center, right) alignment. Placed right after F02 in E01's order.
  Files: `E01`, `F07`, `docs/backlog/features/README.md`.
- **docs(backlog): refine F07 with the Product Owner; open Q21, the layout box**
  F07: anchor on a 3 x 3 grid, default center, stored as an intention, placing the sharp-cornered polygon, rotation after. The "always touch" option is left to the spike. Q21 records the idea of a rectangular layout box.
  Files: `F07`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **docs(backlog): add E13, the shape tree and the layout box; settle Q21**
  The Product Owner wants an epic between shapes and symbols for the tree and a layout box: anchor or row/column stacking, a child may override the anchor, no hug. Placed before E03, whose booleans are tree nodes; the shape tree leaves E05.
  Files: `E05`, `E13`, `docs/backlog/epics/README.md`, `F07`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/domain/shapes.md`.
- **docs(backlog): settle stacks in the layout box; confirm E13's order**
  A child with its own anchor stays in the box and leaves the stack; stacks are a drawing aid, fixed once the symbol is built. The Product Owner confirms E13 before E03 and could not find the epic order: logged.
  Files: `E13`, `docs/backlog/epics/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/process/improvements.md`.
- **build(deps): update eslint-plugin-jsdoc to 65.2.4 and prettier to 3.9.10**
  Files: `package-lock.json`, `package.json`.
- **chore(tooling): order guard in /run; state and journal after F07 and E13**
  A run only takes the next feature in the Product Owner's order and refuses any other scope. CLAUDE.md states F07 as next; session logged.
  Files: `.claude/skills/run/SKILL.md`, `CLAUDE.md`, `docs/process/journal.md`.
- **chore(main): release 0.8.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.

## 2026-10-11

- **docs(backlog): refine F07 and set its stories ready**
  Five criteria agreed with the Product Owner, no "always touch" option, seven stories from SP-003 to VAL-003; run authorized with /run; the rotation-after rule recorded in F04.
  Refs: F07
  Files: `E01`, `F04`, `F07`, `docs/backlog/features/README.md`, `AUD-003`, `CHK-003`, `EN-009`, `SP-003`, `US-009`, `US-010`, `VAL-003`, `docs/backlog/stories/README.md` and 1 more.
- **build(deps): update npm-check-updates to 23.1.1**
  Files: `package-lock.json`, `package.json`.
- **chore(main): release 0.8.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the F07 run, stopped at VAL-003**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.8.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the F07 validation in progress**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.8.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(tooling): clean up and ask the Product Owner before a feature pull request**
  Files: `.claude/skills/run/SKILL.md`, `docs/process/journal.md`.
- **chore(main): release 0.8.6**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(geometry): place the fitted polygon by nine alignments**
  DERIV-regular-polygon-fit step 5 follows SVG's preserveAspectRatio (meet): min, mid or max per axis, the max form written to keep the edge exact. Check rows for anchored cases. Sources: SVG 2 coordinate systems, Figma.
  Refs: SP-003.T1
  Files: `docs/derivations/regular-polygon-fit.md`, `docs/references.md`.
- **docs(backlog): research note 0005; F07 stories cite their sources**
  Refs: SP-003.T2
  Files: `EN-009`, `US-009`, `US-010`, `docs/research/notes/0005-f07-polygon-anchor.md`, `docs/research/notes/README.md`.
- **docs(geometry): anchored edges within rounding; one SVG reference**
  Review of SP-003: the two vertices of one edge may differ in their last bit, so the anchored side holds within rounding and is written exactly (5 decimals); REF-SVG2-COORDS now covers §8.2 and §8.7 (one URL, one ID); keyboard of the anchor grid sourced (W3C radio group pattern).
  Refs: SP-003.T2
  Files: `F07`, `EN-009`, `SP-003`, `US-009`, `US-010`, `docs/derivations/regular-polygon-fit.md`, `docs/references.md`, `docs/research/notes/0005-f07-polygon-anchor.md`, `docs/research/notes/README.md`.
- **docs(backlog): set SP-003 done**
  Closes: SP-003
  Files: `F07`, `SP-003`, `docs/backlog/stories/README.md`.
- **feat(geometry): fit a shape in its box at one of nine anchors**
  fitInBox takes the box size and an anchor (horizontal and vertical alignment, min / mid / max) and puts 0, half or all of the room before the shape, as SVG's preserveAspectRatio with meet. alignmentFactor, Alignment, Anchor, Size. The polygon stays centered until US-009.
  Refs: EN-009.T1
  Files: `src/geometry/alignment-factor.test.ts`, `src/geometry/alignment-factor.ts`, `src/geometry/alignment.ts`, `src/geometry/anchor.ts`, `src/geometry/fit-in-box.test.ts`, `src/geometry/fit-in-box.ts`, `src/geometry/index.ts`, `src/geometry/size.ts`, `src/model/regular-polygon-contour.ts`.
- **docs(geometry): name alignmentFactor in step 5; anchor in the glossary**
  Refs: EN-009.T2
  Files: `EN-009`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/README.md`.
- **test(geometry): anchored sides of real polygons land on the box edge**
  Review of EN-009: the property now runs on unitRegularPolygon (cos and sin rounding), every vertex near an anchored edge within rounding.
  Refs: EN-009.T1
  Files: `src/geometry/fit-in-box.test.ts`.
- **docs(geometry): same written drawing on the filled axis, but at a boundary**
  Refs: EN-009.T2
  Files: `docs/derivations/regular-polygon-fit.md`.
- **docs(backlog): set EN-009 done**
  Closes: EN-009
  Files: `F07`, `EN-009`, `docs/backlog/stories/README.md`.
- **feat(model): anchor a regular polygon in its box**
  RegularPolygon stores an anchor (min / mid / max per axis); the contour is placed by it; isValidRegularPolygon refuses any other alignment (isAnchor, isRegularPolygonCornerCount split out to stay within the domain limits). The playground keeps a centered anchor until US-010.
  Refs: US-009.T1
  Files: `playground/mount-playground.ts`, `src/geometry/alignments.ts`, `src/geometry/index.ts`, `src/model/index.ts`, `src/model/is-anchor.test.ts`, `src/model/is-anchor.ts`, `src/model/is-regular-polygon-corner-count.test.ts`, `src/model/is-regular-polygon-corner-count.ts`, `src/model/is-valid-regular-polygon.test.ts`, `src/model/is-valid-regular-polygon.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-contour.ts` and 2 more.
- **docs(model): the anchor of a regular polygon in the domain**
  Refs: US-009.T2
  Files: `US-009`, `docs/domain/shapes.md`.
- **test(model): direct checks of the anchored faces; one list of alignments**
  Review of US-009: AC4 asserts the right face from (100, 10) to (100, 40) and the triangle's top at y = 10; the vacuous "anchor stored" assertion is dropped; Alignment is derived from ALIGNMENTS and the factors are typed by it, so the three lists cannot drift.
  Refs: US-009.T1
  Files: `src/geometry/alignment-factor.ts`, `src/geometry/alignment.ts`, `src/geometry/alignments.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **docs(backlog): set US-009 done**
  Closes: US-009
  Files: `F07`, `US-009`, `docs/backlog/stories/README.md`, `docs/process/improvements.md`.
- **chore(tooling): pitfalls met in F07; state of the run**
  Cast for an out-of-union test value, mutation restore by content, closed commit scopes; CLAUDE.md states the F07 run.
  Refs: CHK-003.T1
  Files: `.claude/rules/library.md`, `.claude/rules/tooling.md`, `CLAUDE.md`.
- **docs(geometry): a box of size 0 sends the polygon to its anchored place**
  Review of CHK-003: step 4 still said the center; Size is the box a shape is placed in, not its bounding box.
  Refs: CHK-003.T1
  Files: `docs/derivations/regular-polygon-fit.md`, `src/geometry/size.ts`.
- **docs(backlog): checkpoint of F07; set CHK-003 done**
  Light evolvability check; no test card yet (US-010 carries it).
  Refs: CHK-003.T2; Closes: CHK-003
  Files: `F07`, `CHK-003`, `docs/backlog/stories/README.md`.
- **feat(playground): choose the polygon's anchor on a 3 × 3 grid**
  Nine radio buttons (one group, arrow keys by the browser) shown for the polygon only, center checked; the box is drawn in grey behind the polygon so the anchor shows; guided steps carry an anchor (center for F01, F02). Guided steps split per feature, anchor helpers in their own files.
  Refs: US-010.T1
  Files: `playground/create-box-elements.ts`, `playground/guided-step.ts`, `playground/guided-steps-f01.ts`, `playground/guided-steps-f02.ts`, `playground/guided-steps.ts`, `playground/index.html`, `playground/listen-to-anchor.ts`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`, `playground/read-anchor.ts`, `playground/write-anchor-value.ts`.
- **docs(backlog): test card of US-010 for the Product Owner; happy-dom pitfall**
  Refs: US-010.T2
  Files: `.claude/rules/library.md`, `US-010`.
- **test(playground): base kept between its fillets; grid in reading order**
  Review of US-010: the AC4 test matched the radius-0 path too; the order of the nine cells was not checked.
  Refs: US-010.T1
  Files: `playground/mount-playground.test.ts`.
- **docs(backlog): set US-010 done**
  Closes: US-010
  Files: `F07`, `US-010`, `docs/backlog/stories/README.md`.
- **test(model): the flat base stays on the box at any radius; audit fixes**
  AUD-003: property for F07.AC4 (n = 3…12, radius up to 2000); the bound near the largest safe integer is a few units (2 measured), not 0.5; text anchor told apart from a shape's anchor; "default" wording; feature plan counts the playground tests.
  Refs: AUD-003.T1
  Files: `F07`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `src/model/regular-polygon-corners.test.ts`, `src/model/regular-polygon.ts`.
- **docs(backlog): audit of F07; set AUD-003 done**
  Findings table, sources and questions for F03, light evolvability check.
  Refs: AUD-003.T2; Closes: AUD-003
  Files: `F07`, `AUD-003`, `docs/backlog/stories/README.md`.
- **docs(backlog): tests per criterion of F07**
  Refs: VAL-003.T1
  Files: `VAL-003`.
- **feat(playground): guided test of F07, the corner button in a wide box**
  The nine steps of US-010's card, with one more showing a corner button acting as left in a wide box and as bottom in a square one.
  Refs: VAL-003.T2
  Files: `VAL-003`, `playground/guided-step.ts`, `playground/guided-steps-f07.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`.
- **docs(docs): demo page of F07, the polygon anchored in its box**
  Images computed by the library; the F02 page says the center is now a default.
  Refs: VAL-003.T3
  Files: `docs/guide/README.md`, `docs/guide/f02-regular-polygon.md`, `docs/guide/f07-polygon-anchor.md`, `docs/guide/images/f07-square-bottom-left-wide.svg`, `docs/guide/images/f07-square-right.svg`, `docs/guide/images/f07-triangle-bottom.svg`, `docs/guide/images/f07-triangle-top-radius-10.svg`, `docs/guide/images/f07-triangle-top.svg`.
- **docs(model): keep the nine anchor places, a corner holds both choices**
  The Product Owner's account of the validation so far, quoted.
  Refs: VAL-003.T4
  Files: `F07`, `VAL-003`, `docs/domain/shapes.md`.
- **fix(playground): keep Next in place from one guided step to the next**
  Refs: VAL-003.T4
  Files: `playground/index.html`.
- **docs(model): no option to push a rounded polygon onto its box, and why**
  Refs: VAL-003.T4
  Files: `F07`, `docs/domain/shapes.md`.
- **feat(playground): open the guided test on the step the address asks**
  A link such as ?step=20 opens that step; the address follows the step shown, so that it can be copied.
  Refs: VAL-003.T4
  Files: `docs/guide/f07-polygon-anchor.md`, `playground/guided-step-address.test.ts`, `playground/mount-playground.ts`, `playground/read-step-from-address.ts`, `playground/write-step-to-address.ts`.
- **fix(playground): large guide buttons at the top of the panel**
  The step title wraps on one or two lines: only the top of the panel keeps Next in place.
  Refs: VAL-003.T4
  Files: `playground/index.html`.
- **docs(backlog): validation of F07 by the Product Owner; set VAL-003 and F07 done**
  Closes: VAL-003; Closes: F07
  Files: `CLAUDE.md`, `E01`, `F07`, `docs/backlog/features/README.md`, `VAL-003`, `docs/backlog/stories/README.md`, `docs/guide/f07-polygon-anchor.md`.
- **chore(main): release 0.9.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): feedback log of the Product Owner's critiques, sheet at each retrospective**
  Files: `.claude/skills/retro/SKILL.md`, `docs/playbook/agent.md`, `docs/process/README.md`, `docs/process/feedback.md`, `docs/process/journal.md`.
- **chore(main): release 0.9.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): split F03, per-vertex radius first, node editing as F08**
  Refinement of F03 with the Product Owner: explicit points mode, minimal tool bar in the playground, free contour by a voluntary action.
  Files: `E01`, `F03`, `F03`, `F08`, `docs/backlog/features/README.md`, `docs/domain/interaction.md`, `docs/process/feedback.md`.
- **chore(main): release 0.9.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): refinement of F03, per-vertex radius, points mode and list**
  Selection rules for the drawing and the list, Mixed radius, radii kept by vertex number; stories proposed.
  Files: `F03`, `F08`, `docs/domain/interaction.md`, `docs/domain/shapes.md`.
- **docs(docs): feedback on the scope of refinement questions**
  Files: `docs/process/feedback.md`.
- **chore(main): release 0.9.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): start of F03, stories ready and run authorized**
  Files: `CLAUDE.md`, `E01`, `F03`, `docs/backlog/features/README.md`, `AUD-004`, `CHK-004`, `EN-010`, `SP-004`, `US-011`, `US-012`, `US-013`, `VAL-004` and 3 more.
- **chore(main): release 0.9.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): a finished free V1, local only, then a paid pro edition**
  ADR-0030 sets where the project stops. The free V1 runs locally: a working folder, files saved one by one, no server at all (Q5 replaced, E11 renamed). Pro ideas kept out of V1 in docs/domain/editions.md; open question Q22 (which epics make V1).
  Files: `CLAUDE.md`, `docs/adr/0030-free-v1-then-pro-edition.md`, `docs/adr/README.md`, `E11`, `docs/backlog/epics/README.md`, `docs/domain/README.md`, `docs/domain/configuration.md`, `docs/domain/editions.md`, `docs/domain/symbols-and-views.md`.
- **chore(tooling): parallel sessions; the run picks up main first**
  Files: `.claude/skills/run/SKILL.md`, `docs/playbook/agent.md`, `docs/playbook/lessons.md`, `docs/process/feedback.md`, `docs/process/journal.md`, `docs/tooling/agent.md`.
- **docs(backlog): every epic makes the free V1; application shell open**
  Q22 settled by the Product Owner: E11 as files in the working folder, E07 local only; design system epic later. Q23 open: web view in a desktop app. Pro ideas stay in the public repository.
  Files: `docs/adr/0030-free-v1-then-pro-edition.md`, `E07`, `docs/backlog/epics/README.md`, `docs/domain/README.md`, `docs/domain/editions.md`, `docs/process/improvements.md`, `docs/process/journal.md`.
- **chore(main): release 0.9.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): agenda for the E01 retrospective; Q24 foreignObject**
  Files: `docs/domain/README.md`, `docs/process/improvements.md`, `docs/process/journal.md`.
- **chore(main): release 0.9.6**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): pro ideas, Rust engine without limits and a drawing interface**
  Files: `docs/domain/editions.md`, `docs/process/journal.md`.
- **chore(main): release 0.9.7**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): more topics for the E01 retrospective**
  Files: `docs/process/improvements.md`, `docs/process/journal.md`.
- **chore(main): release 0.9.8**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the F03 run, stopped at VAL-004**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.9.9**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the validation of F03**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.9.10**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): sources of F03, unequal radii checks, one radius per vertex**
  ADR-0029: radii in contour order replace the single radius; the shape's radius is derived, Mixed when they differ.
  Refs: SP-004.T1
  Files: `docs/adr/0029-one-radius-per-vertex.md`, `docs/adr/README.md`, `docs/derivations/local-radius-clamp.md`, `docs/references.md`.
- **docs(backlog): research note of F03; stories cite their sources**
  Refs: SP-004.T2
  Files: `EN-010`, `US-011`, `US-012`, `US-013`, `docs/research/notes/0006-f03-radius-per-vertex.md`, `docs/research/notes/README.md`.
- **docs(backlog): review of SP-004, vertex 1 and 2 of the contour, range start**
  Refs: SP-004.T2
  Files: `docs/adr/0029-one-radius-per-vertex.md`, `F03`, `EN-010`, `US-011`, `US-012`, `US-013`, `docs/derivations/local-radius-clamp.md`, `docs/domain/interaction.md`, `docs/references.md`, `docs/research/notes/0006-f03-radius-per-vertex.md`.
- **docs(backlog): set SP-004 done**
  Closes: SP-004
  Files: `F03`, `SP-004`, `docs/backlog/stories/README.md`.
- **feat(model): one requested corner radius per vertex**
  radii replaces radius in Rectangle and RegularPolygon (ADR-0029), checked by areValidRadii and given to the vertices by cornersWithRadii. The playground gives the radius typed to every corner, its page code split into smaller files to stay within the limits. vertex, instead of radius.
  BREAKING CHANGE: Rectangle and RegularPolygon take radii, one radius per; Refs: EN-010.T1
  Files: `playground/create-box-elements.ts`, `playground/mount-playground.test.ts`, `playground/mount-playground.ts`, `playground/read-guided-step.ts`, `playground/read-number.ts`, `playground/read-shape.ts`, `playground/select-element.ts`, `playground/shape-reading.ts`, `playground/show-guided-step.ts`, `playground/show-shape.ts`, `playground/write-capped-sizes.ts`, `playground/write-input-value.ts` and 21 more.
- **docs(model): domain of the radius per vertex; glossary says arrondi**
  Refs: EN-010.T2
  Files: `docs/domain/README.md`, `docs/domain/shapes.md`.
- **test(model): unequal radii on the polygon, all F03 clamp rows; review fixes**
  A vertex without radius is sharp, as ADR-0029 says; the playground marks every number input from one place; the domain states the count rule.
  Refs: EN-010.T1
  Files: `docs/domain/shapes.md`, `playground/write-validity.ts`, `src/model/corners-with-radii.test.ts`, `src/model/corners-with-radii.ts`, `src/model/is-valid-regular-polygon.ts`, `src/model/rectangle-corners.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **docs(backlog): set EN-010 done**
  Closes: EN-010
  Files: `F03`, `EN-010`, `docs/backlog/stories/README.md`.
- **feat(model): radii kept by vertex number; the common radius, or Mixed**
  resizeRadii keeps each vertex's radius when the number of corners changes, new vertices sharp; commonRadius gives the radius shared by every vertex, none when they differ (REF-FIGMA-CR).
  Refs: US-011.T1
  Files: `src/model/common-radius.test.ts`, `src/model/common-radius.ts`, `src/model/index.ts`, `src/model/resize-radii.test.ts`, `src/model/resize-radii.ts`.
- **feat(playground): the radius field shows Mixed when the corners differ**
  The page keeps different radii on the radius input, resized by vertex number; a radius typed goes to every corner. Product Owner test card.
  Refs: US-011.T2
  Files: `US-011`, `playground/index.html`, `playground/listen-to-inputs.ts`, `playground/mount-playground.ts`, `playground/radius-field.test.ts`, `playground/read-radii.ts`, `playground/read-shape.ts`, `playground/show-guided-step.ts`, `playground/show-shape.ts`, `playground/write-radius-field.ts`.
- **fix(playground): a removed vertex never gets its radius back; review fixes**
  The resized radii are kept even while the shape is refused; Mixed stops showing once the field is typed in or a guided step types a radius.
  Refs: US-011.T2
  Files: `docs/research/notes/0006-f03-radius-per-vertex.md`, `playground/listen-to-inputs.ts`, `playground/radius-field.test.ts`, `playground/read-radii.ts`, `playground/read-shape.ts`, `playground/show-guided-step.ts`, `playground/update-kept-radii.ts`, `src/model/common-radius.ts`, `src/model/resize-radii.ts`.
- **docs(backlog): set US-011 done**
  Closes: US-011
  Files: `F03`, `US-011`, `docs/backlog/stories/README.md`.
- **docs(tooling): three library pitfalls met in F03**
  Refs: CHK-004.T1
  Files: `.claude/rules/library.md`.
- **docs(backlog): light evolvability check in the middle of F03**
  Refs: CHK-004.T2
  Files: `CHK-004`, `docs/process/improvements.md`.
- **docs(model): concave contours come with F08; Shape and Points modes named**
  Review of CHK-004.
  Refs: CHK-004.T1
  Files: `CHK-004`, `docs/domain/interaction.md`, `docs/domain/shapes.md`.
- **docs(backlog): set CHK-004 done**
  Closes: CHK-004
  Files: `F03`, `CHK-004`, `docs/backlog/stories/README.md`.
- **feat(playground): a points mode, a handle on each vertex, first red, second green**
  A tool bar of two large buttons, Shape and Points (REF-INKSCAPE-NODE-TOOL as functional reference); vertex 1 is drawn last, on top.
  Refs: US-012.T1
  Files: `playground/create-handle-elements.ts`, `playground/index.html`, `playground/listen-to-inputs.ts`, `playground/listen-to-modes.ts`, `playground/points-mode.test.ts`, `playground/read-mode.ts`, `playground/show-shape.ts`.
- **docs(backlog): test card of US-012 for the Product Owner; help text**
  Refs: US-012.T2
  Files: `US-012`, `playground/index.html`.
- **fix(playground): one red and one green handle tested; card of a 0 x 50 rectangle**
  Review of US-012: the card said a line is drawn, but a shape without area is not; the domain states which handle is on top.
  Refs: US-012.T2
  Files: `US-012`, `docs/domain/interaction.md`, `playground/create-handle-elements.ts`, `playground/points-mode.test.ts`, `playground/read-mode.ts`.
- **docs(backlog): set US-012 done**
  Closes: US-012
  Files: `F03`, `US-012`, `docs/backlog/stories/README.md`.
- **feat(interaction): select vertices by click, toggle and range; radius on the selection**
  A first module of the interaction layer: selectionAfterClick, -Toggle and -Range (REF-APPLE-SELECT, REF-WAI-APG-LISTBOX, note 0006); radiiWithSelection gives the radius typed to the selected vertices only.
  Refs: US-013.T1
  Files: `src/index.ts`, `src/interaction/index.ts`, `src/interaction/no-vertex-selection.ts`, `src/interaction/selection-after-click.ts`, `src/interaction/selection-after-range.ts`, `src/interaction/selection-after-toggle.ts`, `src/interaction/selection.test.ts`, `src/interaction/vertex-selection.ts`, `src/model/index.ts`, `src/model/radii-with-selection.test.ts`, `src/model/radii-with-selection.ts`.
- **feat(playground): list of points, selection on handles and rows, radius per point**
  Clicks on a handle or a row select vertices (selectionAfterHandleClick and selectionAfterRowClick: Shift and Ctrl or Cmd as the Product Owner chose); a radius typed in Points mode goes to the selected vertices only; the field reads Mixed, or Select points when none is selected.
  Refs: US-013.T2
  Files: `playground/create-handle-elements.ts`, `playground/index.html`, `playground/listen-to-inputs.ts`, `playground/listen-to-modes.ts`, `playground/listen-to-points.ts`, `playground/listen-to-radius.ts`, `playground/points-list.test.ts`, `playground/radius-field.test.ts`, `playground/read-selection.ts`, `playground/show-guided-step.ts`, `playground/show-points-list.ts`, `playground/show-points.ts` and 8 more.
- **docs(backlog): test card of US-013 for the Product Owner; help text**
  Refs: US-013.T3
  Files: `US-013`, `playground/index.html`.
- **fix(playground): radii kept while a polygon is refused; Shift keeps the range start**
  Review of US-013: a refused polygon cut its radii to four; Shift + click on a handle moved the start of the next range, against note 0006; a refused shape no longer shows the list and handles of the last valid one. The selection functions move to the model: pure decisions belong to the core.
  Refs: US-013.T2
  Files: `docs/conventions/architecture.md`, `docs/conventions/testing.md`, `playground/mount-playground.ts`, `playground/points-list.test.ts`, `playground/read-shape.ts`, `playground/show-refused-shape.ts`, `src/index.ts`, `src/interaction/index.ts`, `src/model/click-keys.ts`, `src/model/index.ts`, `src/model/no-vertex-selection.ts`, `src/model/selection-after-click.ts` and 6 more.
- **docs(backlog): set US-013 done**
  Closes: US-013
  Files: `F03`, `US-013`, `docs/backlog/stories/README.md`.
- **test(model): audit of F03, mutations and properties of the radius per vertex**
  Every F03 model function mutated once; the one survivor (radii reversed on a square) gets a test that tells vertex 1 from vertex 4. Properties for radiiWithSelection, commonRadius and the flat base under any radii.
  Refs: AUD-004.T1
  Files: `src/model/common-radius.test.ts`, `src/model/radii-with-selection.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **fix(playground): a radius typed in Points mode is capped at 100 000; audit fixes**
  AUD-004 A–E: the cap of Q20 applied before the radius goes to the selection; TSDoc of the toggle and handle clicks; cornersWithRadii reuses resizeRadii; the handles' path data uses the io formatters; properties for unequal radii (closed outline), cornersWithRadii and handle clicks; end-to-end tests of radii kept by vertex number; Q25 opened for E13.
  Refs: AUD-004.T1
  Files: `docs/conventions/testing.md`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/process/improvements.md`, `docs/references.md`, `playground/create-handle-elements.ts`, `playground/listen-to-radius.ts`, `playground/points-list.test.ts`, `src/model/corners-with-radii.test.ts`, `src/model/corners-with-radii.ts`, `src/model/regular-polygon-corners.test.ts`, `src/model/selection-after-handle-click.ts` and 2 more.
- **ci(ci): gitlab merge request pages refuse the link checker**
  Refs: AUD-004.T2
  Files: `.lycheeignore`.
- **docs(backlog): findings of the F03 audit, sources for F08, evolvability**
  Refs: AUD-004.T2
  Files: `AUD-004`.
- **docs(backlog): set AUD-004 done**
  Closes: AUD-004
  Files: `F03`, `AUD-004`, `docs/backlog/stories/README.md`.
- **docs(backlog): tests per criterion of F03**
  Refs: VAL-004.T1
  Files: `VAL-004`.
- **feat(playground): guided test of F03, the points of a shape and their radii**
  Steps 29 to 33, from the US-012 and US-013 cards; each starts afresh.
  Refs: VAL-004.T2
  Files: `playground/guided-step.ts`, `playground/guided-steps-f03.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`.
- **docs(docs): demo page of F03, a rounding for each corner**
  Images computed by the library.
  Refs: VAL-004.T3
  Files: `docs/guide/README.md`, `docs/guide/f03-radius-per-vertex.md`, `docs/guide/images/f03-hexagon-points.svg`, `docs/guide/images/f03-hexagon-two-corners.svg`, `docs/guide/images/f03-one-corner.svg`, `docs/guide/images/f03-reduced.svg`.
- **docs(backlog): the Product Owner's remarks on F03, Q26 and the free contour of F08**
  Refs: VAL-004.T4
  Files: `F08`, `docs/domain/README.md`, `docs/guide/f03-radius-per-vertex.md`, `docs/process/improvements.md`.
- **docs(backlog): idea of an agile workspace for working with the agent, for later**
  Refs: VAL-004.T4
  Files: `docs/process/improvements.md`.
- **docs(backlog): validation of F03 by the Product Owner; set VAL-004 and F03 done**
  Closes: VAL-004; Closes: F03
  Files: `CLAUDE.md`, `E01`, `F03`, `docs/backlog/features/README.md`, `VAL-004`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.10.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): update eslint-plugin-jsdoc to 65.3.0**
  Published minutes before the CI of release 0.10.0 ran, which failed on the version check.
  Files: `package-lock.json`, `package.json`.
- **docs(backlog): refinement of F08, node editing; later ideas to E02 and E03**
  Files: `E02`, `E03`, `F08`, `docs/domain/interaction.md`.
- **chore(main): release 0.10.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): start of F08, stories ready and run authorized**
  Files: `CLAUDE.md`, `E01`, `F08`, `docs/backlog/features/README.md`, `AUD-005`, `CHK-005`, `EN-011`, `SP-005`, `US-014`, `US-015`, `US-016`, `VAL-005` and 3 more.
- **chore(main): release 0.10.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): stretchable free shape (F09) before F08, which pauses after its spike**
  Files: `CLAUDE.md`, `E01`, `F08`, `F09`, `docs/backlog/features/README.md`, `AUD-005`, `CHK-005`, `EN-011`, `US-014`, `US-015`, `US-016`, `VAL-005` and 10 more.
- **chore(main): release 0.10.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the F09 run, stopped at VAL-006**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.10.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): log the validation of F09**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.10.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): stretching a free shape without loss; the rectangle is a free shape**
  DERIV-free-shape-stretch from CSS Transforms (a scale about the top-left of the box); ADR-0032.
  Refs: SP-006.T1
  Files: `docs/adr/0032-every-shape-is-a-free-shape.md`, `docs/adr/README.md`, `docs/derivations/README.md`, `docs/derivations/free-shape-stretch.md`, `docs/references.md`.
- **docs(backlog): research note of F09; stories cite their sources**
  Refs: SP-006.T2
  Files: `EN-012`, `SP-006`, `docs/research/notes/0008-f09-stretchable-free-shape.md`, `docs/research/notes/README.md`.
- **docs(adr): review of SP-006, exact far edge, one box-to-box map, migration listed**
  Refs: SP-006.T2
  Files: `docs/adr/0032-rectangle-is-a-free-shape.md`, `docs/adr/README.md`, `EN-012`, `US-017`, `docs/derivations/free-shape-stretch.md`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/references.md`, `docs/research/notes/0008-f09-stretchable-free-shape.md`.
- **docs(backlog): set SP-006 done**
  Closes: SP-006
  Files: `F09`, `SP-006`, `docs/backlog/stories/README.md`.
- **feat(model): a free shape stretched by its width and height; the rectangle tool**
  stretchToSize scales each axis from the top-left of the box, multiplying first so that the far edge is exact (DERIV-free-shape-stretch); FreeShape, its validity, contour and corners; rectangleAsFreeShape stores the unit square, and rectangleContour and rectangleCorners now draw through it, so the F01 and F03 tests prove the drawing unchanged (ADR-0032).
  Refs: EN-012.T1
  Files: `src/geometry/index.ts`, `src/geometry/stretch-to-size.test.ts`, `src/geometry/stretch-to-size.ts`, `src/model/are-free-shape-points.ts`, `src/model/free-shape-contour.ts`, `src/model/free-shape-corners.test.ts`, `src/model/free-shape-corners.ts`, `src/model/free-shape.ts`, `src/model/index.ts`, `src/model/is-valid-free-shape.test.ts`, `src/model/is-valid-free-shape.ts`, `src/model/min-free-shape-points.ts` and 3 more.
- **docs(model): the free shape in the domain**
  Refs: EN-012.T2
  Files: `docs/domain/shapes.md`.
- **test(model): review of EN-012, height 0, a stored rectangle stretched, exactness bound**
  Refs: EN-012.T1
  Files: `EN-012`, `docs/derivations/free-shape-stretch.md`, `docs/references.md`, `docs/research/notes/0008-f09-stretchable-free-shape.md`, `src/geometry/stretch-to-size.test.ts`, `src/geometry/stretch-to-size.ts`, `src/model/free-shape-corners.test.ts`.
- **docs(backlog): set EN-012 done**
  Closes: EN-012
  Files: `F09`, `EN-012`, `docs/backlog/stories/README.md`.
- **docs(tooling): two library pitfalls met in F09**
  Refs: CHK-006.T1
  Files: `.claude/rules/library.md`.
- **docs(backlog): light evolvability check in the middle of F09**
  Refs: CHK-006.T2
  Files: `CHK-006`, `docs/process/improvements.md`.
- **docs(model): review of CHK-006, F08 rules marked to come, paused ADR and note indexed**
  Refs: CHK-006.T1
  Files: `docs/adr/README.md`, `docs/domain/interaction.md`, `docs/domain/shapes.md`, `docs/research/notes/README.md`, `src/model/rectangle.ts`.
- **docs(backlog): set CHK-006 done**
  Closes: CHK-006
  Files: `F09`, `CHK-006`, `docs/backlog/stories/README.md`.
- **feat(playground): the rectangle is read and shown as a free shape**
  Refs: US-017.T1
  Files: `playground/create-box-elements.ts`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`, `playground/shape-reading.ts`.
- **docs(backlog): test card of US-017 for the Product Owner; help text**
  Refs: US-017.T2
  Files: `US-017`, `playground/index.html`.
- **test(playground): review of US-017, stored points kept at width 0, drawn points listed**
  Refs: US-017.T1
  Files: `US-017`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`.
- **docs(backlog): set US-017 done**
  Closes: US-017
  Files: `F09`, `US-017`, `docs/backlog/stories/README.md`.
- **test(model): audit of F09, properties of the stretch, a fold for the bounding box**
  AUD-006 A–E: mutations 10 of 10 caught; properties for the round trip, the order of coordinates (a rectangle stays one) and the validity of the rectangle tool; boundingBox no longer spreads its points into Math.max; the exactness bound stated; Q27 opened for F08; stale guide and note wording fixed.
  Refs: AUD-006.T1
  Files: `F08`, `docs/derivations/free-shape-stretch.md`, `docs/domain/README.md`, `docs/guide/f01-rounded-rectangle.md`, `docs/guide/f03-radius-per-vertex.md`, `docs/references.md`, `docs/research/notes/0008-f09-stretchable-free-shape.md`, `playground/read-shape.ts`, `src/geometry/bounding-box.ts`, `src/geometry/stretch-to-size.test.ts`, `src/model/is-valid-free-shape.test.ts`.
- **docs(backlog): findings of the F09 audit, next features, evolvability**
  Refs: AUD-006.T2
  Files: `AUD-006`.
- **docs(backlog): set AUD-006 done**
  Closes: AUD-006
  Files: `F09`, `AUD-006`, `docs/backlog/stories/README.md`.
- **docs(backlog): tests per criterion of F09**
  Refs: VAL-006.T1
  Files: `VAL-006`.
- **feat(playground): guided test of F09, the rectangle as a free shape**
  Steps 34 to 38, from the US-017 card.
  Refs: VAL-006.T2
  Files: `playground/guided-step.ts`, `playground/guided-steps-f09.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`.
- **docs(docs): demo page of F09, the rectangle as a free shape**
  Refs: VAL-006.T3
  Files: `docs/guide/README.md`, `docs/guide/f09-stretchable-free-shape.md`, `docs/guide/images/f09-rectangle.svg`, `docs/guide/images/f09-triangle-100x40.svg`, `docs/guide/images/f09-triangle-60x80.svg`.
- **docs(backlog): the Product Owner's remarks on F09, corners with F08**
  Refs: VAL-006.T4
  Files: `F08`, `docs/domain/README.md`, `docs/guide/f09-stretchable-free-shape.md`, `docs/process/improvements.md`.
- **docs(backlog): validation of F09 by the Product Owner; set VAL-006 and F09 done**
  Closes: VAL-006; Closes: F09
  Files: `CLAUDE.md`, `E01`, `F09`, `docs/backlog/features/README.md`, `VAL-006`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.11.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): node editing in progress again, run authorized**
  Files: `CLAUDE.md`, `E01`, `F08`, `docs/backlog/features/README.md`, `docs/process/journal.md`.
- **chore(main): release 0.11.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the F08 run, stopped at VAL-005**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.11.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the F08 validation**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.11.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(adr): sources of F08; a free shape keeps fractional vertices**
  ADR-0031: converting a polygon does not move it; typed and moved vertices are integers.
  Refs: SP-005.T1
  Files: `docs/adr/0031-free-shape-keeps-fractional-vertices.md`, `docs/adr/README.md`, `docs/references.md`.
- **docs(backlog): research note of F08; stories cite their sources**
  Refs: SP-005.T2
  Files: `EN-011`, `US-015`, `US-016`, `docs/process/feedback.md`, `docs/process/improvements.md`, `docs/research/notes/0007-f08-node-editing.md`, `docs/research/notes/README.md`.
- **docs(adr): review of SP-005, ADR-0003 partly superseded, criteria made testable**
  Refs: SP-005.T2
  Files: `CLAUDE.md`, `docs/adr/0003-integer-model.md`, `docs/adr/0031-free-shape-keeps-fractional-vertices.md`, `docs/adr/README.md`, `US-014`, `US-015`, `US-016`, `docs/domain/README.md`, `docs/domain/shapes.md`, `docs/references.md`, `docs/research/notes/0007-f08-node-editing.md`.
- **docs(backlog): the Product Owner's answers to the review of SP-005**
  Refs: SP-005.T2
  Files: `US-015`, `docs/domain/interaction.md`, `docs/research/notes/0007-f08-node-editing.md`.
- **docs(backlog): set SP-005 done**
  Closes: SP-005
  Files: `F08`, `SP-005`, `docs/backlog/stories/README.md`.
- **docs(backlog): node editing resumed on top of F09, its spike's decisions kept**
  Files: `docs/adr/README.md`, `F08`, `docs/backlog/stories/README.md`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/domain/shapes.md`.
- **docs(backlog): node editing refined after Q27, corners and polygon numbering; run authorized**
  Files: `F08`, `AUD-005`, `CHK-005`, `EN-011`, `EN-013`, `US-014`, `US-015`, `US-016`, `VAL-005`, `docs/backlog/stories/README.md`, `docs/domain/README.md`.
- **feat(geometry): a regular polygon numbered from the right end of its base**
  Point 1 and point 2 are the two ends of the flat base at any number of corners, so radii kept by vertex number stay on the base (Q18 amended, F08). The polygon tests, the F03 and F07 playground tests follow the new order; the shapes do not change.
  Refs: EN-013.T1
  Files: `playground/mount-playground.test.ts`, `playground/points-list.test.ts`, `playground/points-mode.test.ts`, `src/geometry/fit-in-box.test.ts`, `src/geometry/unit-regular-polygon.test.ts`, `src/geometry/unit-regular-polygon.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **docs(geometry): the polygon's start at the right end of its base in the docs**
  Derivation step 2 and check tables, domain, F02 feature note, the F03 guided step, guide page and image.
  Refs: EN-013.T2
  Files: `F02`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/shapes.md`, `docs/guide/f03-radius-per-vertex.md`, `docs/guide/images/f03-hexagon-points.svg`, `playground/guided-steps-f03.ts`.
- **docs(geometry): review of EN-013, the polygon exception in Q11, CLAUDE.md and E01**
  Refs: EN-013.T2
  Files: `CLAUDE.md`, `E01`, `US-012`, `US-013`, `docs/derivations/regular-polygon-fit.md`, `docs/domain/README.md`, `docs/domain/shapes.md`, `src/geometry/unit-regular-polygon.test.ts`, `src/model/regular-polygon-corners.test.ts`.
- **docs(backlog): set EN-013 done**
  Closes: EN-013
  Files: `F08`, `EN-013`, `docs/backlog/stories/README.md`.
- **docs(adr): a free shape of corners, fractions until edited, typed in the SVG frame**
  ADR-0033, from Q27 and the remark of VAL-006.
  Refs: EN-011.T1
  Files: `docs/adr/0029-one-radius-per-vertex.md`, `docs/adr/0033-free-shape-of-corners.md`, `docs/adr/README.md`.
- **feat(model): a free shape of corners; a polygon converted unmoved; a point typed**
  FreeShape keeps each point with its radius (ADR-0033); its points and size may be fractional, as regularPolygonAsFreeShape leaves them; freeShapeWithPoint places a point typed in the SVG's coordinates and fits the box around the points. The playground reads the radii from the reading.
  Refs: EN-011.T2
  Files: `playground/mount-playground.test.ts`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`, `playground/shape-reading.ts`, `playground/show-points.ts`, `playground/show-shape.ts`, `src/model/are-free-shape-points.ts`, `src/model/free-shape-contour.ts`, `src/model/free-shape-corners.test.ts`, `src/model/free-shape-corners.ts`, `src/model/free-shape-radii.ts`, `src/model/free-shape-with-point.test.ts` and 8 more.
- **docs(model): the free shape of corners in the domain**
  Refs: EN-011.T3
  Files: `docs/domain/README.md`, `docs/domain/shapes.md`.
- **fix(playground): a fractional width still refused for the rectangle; review of EN-011**
  The rectangle tool's sizes are checked as integers again; a free shape at its own size is drawn bit for bit, so a converted polygon does not move by an ulp; one function fits a free shape around drawn corners; degenerate cases tested; ADR-0032 marked partly superseded; domain, guide and the F09 guided step speak of corners.
  Refs: EN-011.T2
  Files: `docs/adr/0032-rectangle-is-a-free-shape.md`, `docs/adr/0033-free-shape-of-corners.md`, `docs/adr/README.md`, `docs/derivations/free-shape-stretch.md`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/domain/shapes.md`, `docs/guide/f09-stretchable-free-shape.md`, `playground/guided-steps-f09.ts`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`, `src/geometry/stretch-to-size.ts` and 8 more.
- **docs(backlog): set EN-011 done**
  Closes: EN-011
  Files: `F08`, `EN-011`, `docs/backlog/stories/README.md`.
- **feat(playground): convert a polygon unmoved; type the x and y of a point**
  A Convert button for the regular polygon in Points mode, greyed at size 0; X and Y fields for the one point selected, integers, greyed on a polygon; the page keeps the free shape on the shape selector, shown as Free shape, its exact size kept while the inputs show it; a rectangle whose point is typed becomes a free shape.
  Refs: US-014.T2
  Files: `playground/free-shape-editing.test.ts`, `playground/index.html`, `playground/listen-to-point-fields.ts`, `playground/listen-to-points.ts`, `playground/read-free-shape-shape.ts`, `playground/read-shape.ts`, `playground/read-stored-free-shape.ts`, `playground/show-point-fields.ts`, `playground/show-points.ts`, `playground/write-stored-free-shape.ts`.
- **docs(backlog): test card of US-014 for the Product Owner; help text**
  Refs: US-014.T3
  Files: `US-014`, `playground/index.html`.
- **fix(playground): no point typed nor polygon converted on a refused shape; review of US-014**
  The point fields hide on a refused shape and the handlers check the shape is valid, with exactly one point selected; a point moves on change, so a minus sign or an emptied field moves nothing on the way; a guided step drops the free shape kept; the kept text is checked on reading; the refusal message says what is checked (a box of some width and height).
  Refs: US-014.T2
  Files: `US-014`, `playground/free-shape-editing.test.ts`, `playground/listen-to-point-fields.ts`, `playground/read-free-shape-shape.ts`, `playground/read-stored-free-shape.ts`, `playground/show-guided-step.ts`, `playground/show-refused-shape.ts`.
- **docs(backlog): set US-014 done**
  Closes: US-014
  Files: `F08`, `US-014`, `docs/backlog/stories/README.md`.
- **refactor(playground): one reader of the shape kind, one input writer; rules of US-014**
  readShapeKind gets its own file, used by the free shape store too; writeInputValue writes the SVG decimals and replaces a copy of it; the domain says when a typed point applies and what stays open for VAL-005.
  Refs: CHK-005.T1
  Files: `docs/domain/interaction.md`, `playground/read-shape-kind.ts`, `playground/read-shape.ts`, `playground/read-stored-free-shape.ts`, `playground/write-input-value.ts`, `playground/write-stored-free-shape.ts`.
- **docs(backlog): light evolvability check of F08; test cards listed**
  Refs: CHK-005.T2
  Files: `.claude/rules/library.md`, `CHK-005`, `docs/process/improvements.md`.
- **docs(backlog): set CHK-005 done**
  Closes: CHK-005
  Files: `F08`, `CHK-005`, `docs/backlog/stories/README.md`.
- **feat(model): add a sharp point in the middle of an edge**
  midpoint (REF-MATHWORLD-MIDPOINT), edgeOfSelection (two neighbouring points selected, the last and the first included) and freeShapeWithMidpoint (the point right after the edge's first, point 1 kept first); hand-computed tests, the corner radius 1000 drawn 100 then 50.
  Refs: US-015.T1
  Files: `docs/domain/interaction.md`, `src/math/index.ts`, `src/math/midpoint.test.ts`, `src/math/midpoint.ts`, `src/model/edge-of-selection.test.ts`, `src/model/edge-of-selection.ts`, `src/model/free-shape-with-midpoint.test.ts`, `src/model/free-shape-with-midpoint.ts`, `src/model/index.ts`.
- **feat(playground): add a point in the middle of the edge of two neighbours**
  A button under the list of points, greyed unless two neighbouring points of a free shape are selected; the new point is selected alone, ready to be moved; on a polygon a line says to convert it first; test card.
  Refs: US-015.T2
  Files: `US-015`, `playground/free-shape-editing.test.ts`, `playground/index.html`, `playground/listen-to-point-fields.ts`, `playground/read-edited-free-shape.ts`, `playground/show-point-buttons.ts`, `playground/show-point-fields.ts`, `playground/write-added-midpoint.ts`.
- **fix(model): no edge for points outside the shape; review of US-015**
  edgeOfSelection asks two distinct whole numbers of points of the shape; properties for edgeOfSelection and freeShapeWithMidpoint; the domain says the new point is selected alone, and which change of corners clears the selection.
  Refs: US-015.T1
  Files: `docs/domain/interaction.md`, `src/model/edge-of-selection.test.ts`, `src/model/edge-of-selection.ts`, `src/model/free-shape-with-midpoint.test.ts`.
- **docs(backlog): set US-015 done**
  Closes: US-015
  Files: `F08`, `US-015`, `docs/backlog/stories/README.md`.
- **feat(model): remove points of a free shape, their neighbours joined**
  freeShapeWithoutPoints removes each point with its radius; the others keep their place and order, the box around them; the first and the last removed together wrap round.
  Refs: US-016.T1
  Files: `docs/domain/interaction.md`, `src/model/free-shape-without-points.test.ts`, `src/model/free-shape-without-points.ts`, `src/model/index.ts`.
- **feat(playground): delete the points selected, by a button or the Delete key**
  deletionOutcome in the model (at least 3 points left, spanning some width and height); a Delete points button greyed unless allowed, with a line saying why; the Delete key outside fields; the selection cleared after a deletion; test card.
  Refs: US-016.T2
  Files: `US-016`, `playground/delete-points.test.ts`, `playground/index.html`, `playground/listen-to-delete-key.ts`, `playground/listen-to-point-fields.ts`, `playground/show-delete-button.ts`, `playground/show-point-buttons.ts`, `playground/write-deleted-points.ts`, `src/model/deletion-outcome.test.ts`, `src/model/deletion-outcome.ts`, `src/model/index.ts`.
- **fix(playground): the backspace key deletes too, as the delete key of a Mac**
  The domain names Backspace (agent's choice, to show at VAL-005) and adds crossing edges to the open question; the removal cites the domain rule; the card says Cmd on a Mac; review of US-016.
  Refs: US-016.T2
  Files: `US-016`, `docs/domain/interaction.md`, `playground/delete-points.test.ts`, `playground/listen-to-delete-key.ts`, `src/model/free-shape-without-points.ts`.
- **docs(backlog): set US-016 done**
  Closes: US-016
  Files: `F08`, `US-016`, `docs/backlog/stories/README.md`.
- **fix(playground): keep the free shape when an edit would leave it flat; audit of F08**
  writeStoredFreeShape keeps only a valid free shape, so a point typed that leaves no width or height changes nothing; adding a point is greyed on a shape of width or height 0; the radii are written by freeShapeRadii.
  Refs: AUD-005.T1
  Files: `playground/free-shape-editing.test.ts`, `playground/show-point-buttons.ts`, `playground/write-added-midpoint.ts`, `playground/write-stored-free-shape.ts`.
- **test(model): a moved point keeps its radius; properties of the free shape edits**
  Infinity refused by finiteness alone; properties for freeShapeWithPoint, deletionOutcome and freeShapeAroundCorners; TSDoc brought in line with the code (polygon start, points spanning no box, fractions after an edit).
  Refs: AUD-005.T1
  Files: `src/model/deletion-outcome.test.ts`, `src/model/deletion-outcome.ts`, `src/model/free-shape-around-corners.test.ts`, `src/model/free-shape-with-midpoint.ts`, `src/model/free-shape-with-point.test.ts`, `src/model/free-shape-with-point.ts`, `src/model/is-valid-free-shape.test.ts`, `src/model/is-valid-free-shape.ts`, `src/model/regular-polygon-contour.ts`.
- **docs(geometry): identity at its own size comes from not computing, not from the formula**
  Refs: AUD-005.T2
  Files: `docs/derivations/free-shape-stretch.md`.
- **docs(backlog): findings of the F08 audit; Q28 gathers the decisions for VAL-005**
  Sources re-verified and dated; the ADR index row of 0003 matches its status; research list for F04; light evolvability check.
  Refs: AUD-005.T2
  Files: `docs/adr/README.md`, `AUD-005`, `docs/domain/README.md`, `docs/references.md`.
- **docs(backlog): set AUD-005 done**
  Closes: AUD-005
  Files: `F08`, `AUD-005`, `docs/backlog/stories/README.md`.
- **feat(playground): the radius field with the point fields in Points mode**
  Product Owner, VAL-005: the shape's radius for the whole shape, a radius with the points for the points selected.
  Refs: VAL-005.T4
  Files: `playground/free-shape-editing.test.ts`, `playground/index.html`, `playground/move-radius-field.ts`, `playground/show-points.ts`, `playground/show-refused-shape.ts`.
- **docs(adr): only the coordinate typed is an integer; edits keep the fractions**
  ADR-0034 (Product Owner, VAL-005) partly supersedes ADR-0031 and ADR-0003; Q28 settled, deleting joins the neighbours confirmed; Q29 opened for degenerate shapes and a cap on typed coordinates; moving several points by +10 and arithmetic in number fields noted for the applications.
  Refs: VAL-005.T4
  Files: `CLAUDE.md`, `docs/adr/0003-integer-model.md`, `docs/adr/0031-free-shape-keeps-fractional-vertices.md`, `docs/adr/0034-only-the-coordinate-typed-is-an-integer.md`, `docs/adr/README.md`, `F08`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/research/notes/0007-f08-node-editing.md`.
- **feat(playground): guided test of F08, steps 39 to 46**
  Refs: VAL-005.T2
  Files: `playground/guided-steps-f08.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`.
- **docs(docs): demo page of F08, editing points**
  Refs: VAL-005.T3
  Files: `docs/guide/README.md`, `docs/guide/f08-node-editing.md`, `docs/guide/images/f08-deleted-point.svg`, `docs/guide/images/f08-hexagon-moved.svg`, `docs/guide/images/f08-midpoint-peak.svg`.
- **fix(playground): a radius field for the points, the shape's always shown; typing keeps the focus**
  The radius field was moved at each key, which took the focus away: the points now have a radius field of their own and the shape's stays in place, in both modes; choosing Free shape again brings back its own size; Q29 partly settled (Product Owner, VAL-005).
  Refs: VAL-005.T4
  Files: `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/guide/f08-node-editing.md`, `playground/free-shape-editing.test.ts`, `playground/guided-steps-f08.ts`, `playground/index.html`, `playground/listen-to-inputs.ts`, `playground/listen-to-radius.ts`, `playground/move-radius-field.ts`, `playground/points-list.test.ts`, `playground/rectangle-free-shape.test.ts`, `playground/show-points.ts` and 4 more.
- **docs(docs): degenerate free shapes settled; ideas for symbols and rotation**
  Q29 settled (crossing edges flagged, no cap); Q30 opened on the saved position of a symbol; a clickable zone and a shared status zone for later; the rotation mode wished for F04 (Product Owner, VAL-005).
  Refs: VAL-005.T4
  Files: `F04`, `VAL-005`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/domain/symbols-and-views.md`, `docs/guide/f08-node-editing.md`.
- **docs(backlog): set VAL-005 and F08 done**
  Closes: VAL-005; Closes: F08
  Files: `CLAUDE.md`, `E01`, `F08`, `docs/backlog/features/README.md`, `VAL-005`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.12.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the F08 merge and the F04 refinement; a critique logged**
  Files: `docs/process/feedback.md`, `docs/process/journal.md`.
- **chore(main): release 0.12.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the F04 run authorization**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.12.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): refine F04, free rotation of shapes**
  Decisions of the Product Owner's interview, criteria, stories SP-007 to VAL-007 in draft, domain rules (center, direction, range, two boxes, position, point numbers never change) and the rotation mode written for E02.
  Files: `E02`, `F04`, `AUD-007`, `CHK-007`, `EN-014`, `EN-015`, `SP-007`, `US-018`, `US-019`, `VAL-007`, `docs/backlog/stories/README.md`, `docs/domain/README.md` and 1 more.
- **docs(backlog): set F04 in progress and its stories ready**
  Product Owner's agreement: /run after the proposed stories.
  Files: `E01`, `F04`, `docs/backlog/features/README.md`, `AUD-007`, `CHK-007`, `EN-014`, `EN-015`, `SP-007`, `US-018`, `US-019`, `VAL-007`, `docs/backlog/stories/README.md`.
- **docs(geometry): sources, derivations and ADR of F04**
  DERIV-turned-shape (angle from 0 to 359, clockwise, exact quarter turns, about the center of the box, kept in place when edited), DERIV-arc-bounding-box (straight box of segments and fillet arcs), ADR-0035 (an angle and a position for every shape), references.
  Refs: SP-007.T1
  Files: `docs/adr/0035-angle-and-position-for-every-shape.md`, `docs/adr/README.md`, `docs/derivations/README.md`, `docs/derivations/arc-bounding-box.md`, `docs/derivations/turned-shape.md`, `docs/references.md`.
- **docs(docs): research note of F04, stories updated**
  Refs: SP-007.T2
  Files: `EN-014`, `EN-015`, `SP-007`, `US-019`, `docs/domain/shapes.md`, `docs/research/notes/0009-f04-free-rotation.md`, `docs/research/notes/README.md`.
- **docs(geometry): review fixes of the F04 research**
  ADR-0035 declares what it partly supersedes (ADR-0003, ADR-0032, ADR-0033), the drift accepted on turned shapes, a typed coordinate exact in the drawing only; the position skipped when it is the stored top-left; quarter-turn checks that exercise step 3; an exact SVG 1.1 quote; the width rule attributed to the Product Owner only.
  Refs: SP-007.T2
  Files: `CLAUDE.md`, `docs/adr/0003-integer-model.md`, `docs/adr/0035-angle-and-position-for-every-shape.md`, `docs/adr/README.md`, `EN-014`, `docs/derivations/turned-shape.md`, `docs/domain/shapes.md`, `docs/references.md`, `docs/research/notes/0009-f04-free-rotation.md`.
- **docs(backlog): set SP-007 done**
  ADR-0032 and ADR-0033 point to ADR-0035.
  Closes: SP-007
  Files: `docs/adr/0032-rectangle-is-a-free-shape.md`, `docs/adr/0033-free-shape-of-corners.md`, `F04`, `SP-007`, `docs/backlog/stories/README.md`.
- **feat(math): angle kept from 0 to 359, exact turn direction, point turned about a center**
  degreesInTurn, turnDirection (quarter turns exact), turnAbout (DERIV-turned-shape).
  Refs: EN-014.T1
  Files: `src/geometry/index.ts`, `src/geometry/turn-about.test.ts`, `src/geometry/turn-about.ts`, `src/math/degrees-in-turn.test.ts`, `src/math/degrees-in-turn.ts`, `src/math/index.ts`, `src/math/turn-direction.test.ts`, `src/math/turn-direction.ts`.
- **feat(model): an angle and a position for every shape, corners turned about the center of the box**
  Free shape, regular polygon and rectangle tool store angle, x and y (ADR-0035); the straight contour is placed at the position, turnedCorners turns it; validity checks the angle and the position.
  Refs: EN-014.T2
  Files: `playground/create-box-elements.ts`, `playground/read-free-shape-shape.ts`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`, `src/model/deletion-outcome.test.ts`, `src/model/effective-corner-radius.test.ts`, `src/model/free-shape-around-corners.test.ts`, `src/model/free-shape-around-corners.ts`, `src/model/free-shape-contour.ts`, `src/model/free-shape-corners.test.ts`, `src/model/free-shape-with-midpoint.test.ts`, `src/model/free-shape-with-midpoint.ts` and 28 more.
- **docs(docs): angle and position of every shape in the domain and the glossary**
  Refs: EN-014.T3
  Files: `EN-014`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **test(model): review fixes of EN-014**
  A polygon and a free shape placed at a position, an odd-sized rectangle after four quarter turns; straight corners and creation values stated in TSDoc and the domain.
  Refs: EN-014.T2
  Files: `docs/domain/shapes.md`, `src/model/free-shape-around-corners.ts`, `src/model/free-shape-corners.test.ts`, `src/model/rectangle.ts`, `src/model/regular-polygon-as-free-shape.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/turned-corners.test.ts`.
- **docs(backlog): set EN-014 done**
  Closes: EN-014
  Files: `F04`, `EN-014`, `docs/backlog/stories/README.md`.
- **feat(geometry): straight box of a contour of segments and fillet arcs**
  arcAxisPoints, contourBoundingBox (DERIV-arc-bounding-box).
  Refs: EN-015.T1
  Files: `src/geometry/arc-axis-points.test.ts`, `src/geometry/arc-axis-points.ts`, `src/geometry/contour-bounding-box.test.ts`, `src/geometry/contour-bounding-box.ts`, `src/geometry/index.ts`.
- **feat(render): the view fitted to the straight box of the shape**
  boxToViewBox; the playground keeps its origin unless a shape goes to negative coordinates.
  Refs: EN-015.T2
  Files: `EN-015`, `docs/domain/shapes.md`, `playground/show-shape.ts`, `src/render/box-to-view-box.test.ts`, `src/render/box-to-view-box.ts`, `src/render/index.ts`.
- **fix(geometry): review fixes of EN-015**
  An arc without length counted by its ends only; the view's rule tested on both axes and away from the origin; the containment property on four sides; @see to the domain.
  Refs: EN-015.T2
  Files: `EN-015`, `docs/derivations/arc-bounding-box.md`, `src/geometry/arc-axis-points.ts`, `src/geometry/contour-bounding-box.test.ts`, `src/geometry/contour-bounding-box.ts`, `src/render/box-to-view-box.test.ts`, `src/render/box-to-view-box.ts`.
- **docs(backlog): set EN-015 done**
  Closes: EN-015
  Files: `F04`, `EN-015`, `docs/backlog/stories/README.md`.
- **feat(playground): turn a shape by an angle or quarter turns, move it by its straight box**
  Angle field kept from 0 to 359, −90° and +90° buttons, x and y of the straight box typed to move the shape, the turned box as a thin frame and the straight box dashed (fills only); the guided steps of F01 to F09 start straight at (0, 0).
  Refs: US-018.T1, US-018.T2
  Files: `playground/create-box-elements.ts`, `playground/create-straight-box-element.ts`, `playground/index.html`, `playground/listen-to-inputs.ts`, `playground/listen-to-position.ts`, `playground/listen-to-turn.ts`, `playground/move-shape.ts`, `playground/read-angle.ts`, `playground/read-free-shape-shape.ts`, `playground/read-position.ts`, `playground/read-shape.ts`, `playground/show-guided-step.ts` and 3 more.

## 2026-10-12

- **docs(docs): journal of the F04 run stop; two requests of the Product Owner logged**
  Files: `docs/process/journal.md`.
- **chore(main): release 0.12.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): add F10, round shape, to E01 after F04**
  Product Owner's idea and decisions (2026-10-12): a circle in a square box, a capsule otherwise, edited as the polygon; the ellipse stays out (Q4).
  Files: `E01`, `F10`, `docs/backlog/features/README.md`, `docs/domain/shapes.md`, `docs/process/journal.md`.
- **chore(main): release 0.12.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): happy-dom 21.0.1**
  Files: `package-lock.json`, `package.json`.
- **chore(main): release 0.12.5**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): test card of US-018 for the Product Owner**
  Refs: US-018.T3
  Files: `US-018`.
- **fix(playground): review fixes of US-018**
  Moving a kept free shape keeps the width, height and radii typed; a refused angle stays as typed; the position fields show the straight box again once left; no frame on a box too small for one; messages name the angle; tests of the frame, the field typed in, and a turned or moved free shape; test card step 3 corrected.
  Refs: US-018.T2
  Files: `US-018`, `playground/create-box-elements.ts`, `playground/create-straight-box-element.ts`, `playground/free-shape-editing.test.ts`, `playground/listen-to-position.ts`, `playground/listen-to-turn.ts`, `playground/mount-playground.test.ts`, `playground/move-shape.ts`, `playground/read-free-shape-shape.ts`, `playground/read-shape.ts`, `playground/rectangle-free-shape.test.ts`, `playground/turn-shape.test.ts`.
- **docs(backlog): set US-018 done**
  Closes: US-018
  Files: `F04`, `US-018`, `docs/backlog/stories/README.md`.
- **docs(docs): checkpoint of F04, state line and pitfalls**
  Refs: CHK-007.T1
  Files: `.claude/rules/docs.md`, `.claude/rules/library.md`, `CLAUDE.md`.
- **docs(docs): evolvability check of F04, improvement log, test cards**
  Refs: CHK-007.T2
  Files: `CHK-007`, `docs/process/improvements.md`.
- **docs(docs): review fixes of CHK-007**
  Editing a turned shape marked as US-019's; a pitfall made general; the straight box shown as the position.
  Refs: CHK-007.T1
  Files: `.claude/rules/library.md`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **docs(backlog): set CHK-007 done**
  Closes: CHK-007
  Files: `F04`, `CHK-007`, `docs/backlog/stories/README.md`.
- **feat(model): edits of a turned shape keep it in place**
  positionKeepingPoint, positionAfterResize, freeShapeKeptInPlace (DERIV-turned-shape step 5); a point typed on a turned free shape is turned back, the others stay drawn where they were; a point added or deleted, a turned polygon converted, keep the drawing.
  Refs: US-019.T1, US-019.T2
  Files: `src/model/free-shape-around-corners.ts`, `src/model/free-shape-kept-in-place.test.ts`, `src/model/free-shape-kept-in-place.ts`, `src/model/free-shape-with-midpoint.ts`, `src/model/free-shape-with-point.ts`, `src/model/free-shape-without-points.ts`, `src/model/index.ts`, `src/model/position-after-resize.test.ts`, `src/model/position-after-resize.ts`, `src/model/position-keeping-point.test.ts`, `src/model/position-keeping-point.ts`, `src/model/regular-polygon-as-free-shape.test.ts` and 2 more.
- **feat(playground): edit a turned shape without moving it**
  A width or height typed keeps the turned top-left in place; the point fields show and take the drawn coordinates; converting a turned polygon keeps its drawing.
  Refs: US-019.T1, US-019.T2
  Files: `docs/domain/shapes.md`, `playground/edit-turned-shape.test.ts`, `playground/listen-to-inputs.ts`, `playground/listen-to-point-fields.ts`, `playground/listen-to-position.ts`, `playground/move-shape.ts`, `playground/read-shape-position.ts`, `playground/show-shape.ts`, `playground/write-drawn-box.ts`, `playground/write-resized-position.ts`, `playground/write-shape-boxes.ts`, `playground/write-shape-position.ts`.
- **docs(backlog): test card of US-019 for the Product Owner**
  Refs: US-019.T3
  Files: `US-019`.
- **fix(playground): review fixes of US-019**
  A turned shape is placed for the size capped at 100 000 and the exact size of a kept free shape; a refused shape keeps no turned box; the domain says how a turned shape's box is placed after a point typed.
  Refs: US-019.T1
  Files: `docs/domain/shapes.md`, `playground/edit-turned-shape.test.ts`, `playground/show-refused-shape.ts`, `playground/write-resized-position.ts`.
- **docs(backlog): set US-019 done**
  Closes: US-019
  Files: `F04`, `US-019`, `docs/backlog/stories/README.md`.
- **fix(model): audit fixes of F04: one box center, one turn of degrees, properties**
  boxCenter and DEGREES_PER_TURN shared; no dead negative-zero guard in degreesInTurn; a width emptied then retyped keeps a turned shape in place; properties for positionKeepingPoint, arcAxisPoints, boxToViewBox and the turned edits at any angle.
  Refs: AUD-007.T1
  Files: `playground/clear-drawn-box.ts`, `playground/edit-turned-shape.test.ts`, `playground/show-refused-shape.ts`, `src/geometry/arc-axis-points.test.ts`, `src/math/degrees-in-turn.ts`, `src/math/degrees-per-turn.ts`, `src/math/index.ts`, `src/model/box-center.test.ts`, `src/model/box-center.ts`, `src/model/free-shape-with-point.ts`, `src/model/index.ts`, `src/model/is-shape-angle.ts` and 5 more.
- **docs(docs): audit of F04, findings, next feature, evolvability**
  TSDoc of straight corners, ADR-0034 partly superseded by ADR-0035, domain on a turned shape's integers and snapping.
  Refs: AUD-007.T2
  Files: `docs/adr/0034-only-the-coordinate-typed-is-an-integer.md`, `docs/adr/0035-angle-and-position-for-every-shape.md`, `docs/adr/README.md`, `AUD-007`, `docs/domain/README.md`, `docs/domain/interaction.md`, `docs/domain/shapes.md`, `docs/process/improvements.md`, `src/model/free-shape-around-corners.ts`, `src/model/free-shape-corners.ts`, `src/model/free-shape-with-midpoint.ts`, `src/model/free-shape-without-points.ts` and 3 more.
- **docs(backlog): set AUD-007 done**
  Closes: AUD-007
  Files: `F04`, `AUD-007`, `docs/backlog/stories/README.md`, `docs/process/improvements.md`.
- **docs(docs): three requests of the Product Owner for the end of E01**
  Files: `docs/process/improvements.md`.
- **docs(docs): a critique logged: the branch to test was not said**
  Files: `docs/process/feedback.md`.
- **fix(playground): the view stays put while a shape turns**
  The view no longer follows a turned shape into negative coordinates: on screen it turns about its center and may leave the canvas (Product Owner's first try of F04); boxToViewBox removed.
  Refs: VAL-007.T0
  Files: `VAL-007`, `docs/domain/shapes.md`, `playground/show-shape.ts`, `playground/turn-shape.test.ts`, `src/render/box-to-view-box.test.ts`, `src/render/box-to-view-box.ts`, `src/render/index.ts`.
- **feat(playground): guided test of F04, steps 47 to 57**
  Refs: VAL-007.T1, VAL-007.T2
  Files: `VAL-007`, `playground/guided-steps-f04.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`, `playground/turn-shape.test.ts`.
- **docs(docs): demo page of F04, turning a shape**
  Refs: VAL-007.T3
  Files: `VAL-007`, `docs/guide/README.md`, `docs/guide/f04-free-rotation.md`, `docs/guide/images/f04-rectangle-turned.svg`, `docs/guide/images/f04-triangle-turned.svg`.
- **docs(backlog): set VAL-007 and F04 done**
  The Product Owner's account, the journal, the state line; F10's decisions so far; a critique logged.
  Closes: VAL-007; Closes: F04
  Files: `CLAUDE.md`, `E01`, `F04`, `F10`, `docs/backlog/features/README.md`, `VAL-007`, `docs/backlog/stories/README.md`, `docs/domain/shapes.md`, `docs/guide/f04-free-rotation.md`, `docs/process/feedback.md`, `docs/process/journal.md`.
- **chore(main): release 0.13.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): vitest 5.0.4, coverage-v8 5.0.4, eslint-plugin-jsdoc 65.3.1**
  Files: `package-lock.json`, `package.json`.
- **docs(backlog): epics E14 for curves and E15 for text, F10 and F05 moved out of E01**
  The Product Owner cancelled F10 at its validation: a round shape in a long box must be an ellipse, which needs curves outside ADR-0001 (Q4). F10 moves to E14 with Bézier curves and handles, F05 to E15 (Q31); E04 keeps strokes only; F06 is the last feature of E01.
  Files: `CLAUDE.md`, `E01`, `E04`, `E14`, `E15`, `docs/backlog/epics/README.md`, `F05`, `F10`, `F05`, `docs/backlog/features/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md`.
- **docs(docs): round shape of E01 kept in research note 0010, with its account**
  Files: `docs/derivations/README.md`, `docs/derivations/round-shape.md`, `docs/process/feedback.md`, `docs/process/improvements.md`, `docs/process/journal.md`, `docs/references.md`, `docs/research/notes/0010-f10-round-shape.md`, `docs/research/notes/README.md`.
- **chore(main): release 0.13.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): refine F06, the polygon stretched to its box, research request 0011**
  Refs: SP-009.T1
  Files: `E01`, `F06`, `docs/backlog/features/README.md`, `AUD-009`, `CHK-009`, `EN-017`, `SP-009`, `US-021`, `VAL-009`, `docs/backlog/stories/README.md`, `docs/domain/README.md`, `docs/domain/shapes.md` and 1 more.
- **docs(docs): journal of the F06 run stopped at its spike, critique logged**
  Files: `docs/process/feedback.md`, `docs/process/journal.md`.
- **chore(main): release 0.13.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): typescript-eslint 8.71.2**
  Files: `package-lock.json`, `package.json`.
- **chore(main): release 0.13.3**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **build(deps): eslint-plugin-unicorn 78.0.0, its two new rules enabled**
  no-repeated-call-wrappers and no-unnecessary-default-arguments: functions theme, no violation.
  Files: `eslint/rules/unicorn/functions.ts`, `package-lock.json`, `package.json`.
- **chore(main): release 0.13.4**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): research note of F06, answer of request 0011 and its sources**
  Refs: SP-009.T1
  Files: `docs/references.md`, `docs/research/notes/0011-f06-stretched-polygon.md`, `docs/research/notes/README.md`, `docs/research/requests/0011-stretched-polygon-even-sides.md`.
- **docs(backlog): rule C chosen for the stretched polygon, derivation tasks**
  Refs: SP-009.T1
  Files: `F06`, `SP-009`, `docs/domain/shapes.md`, `docs/research/notes/0011-f06-stretched-polygon.md`.
- **docs(geometry): derivation of the stretched polygon, rule C**
  Refs: SP-009.T2
  Files: `EN-017`, `docs/derivations/README.md`, `docs/derivations/stretched-polygon.md`, `docs/references.md`, `docs/research/notes/0011-f06-stretched-polygon.md`.
- **docs(adr): a stretched option for the regular polygon (ADR-0037)**
  Refs: SP-009.T3
  Files: `docs/adr/0037-stretched-polygon.md`, `docs/adr/README.md`, `SP-009`.
- **docs(geometry): review fixes of the F06 research, the octagon is a family**
  Refs: SP-009.T2
  Files: `docs/derivations/stretched-polygon.md`, `docs/research/notes/0011-f06-stretched-polygon.md`.
- **docs(backlog): set SP-009 done**
  Closes: SP-009
  Files: `F06`, `SP-009`, `docs/backlog/stories/README.md`.
- **feat(geometry): the stretched polygon of rule C, sides equal while its corners stay sound**
  Refs: EN-017.T1
  Files: `src/geometry/box-aspect.test.ts`, `src/geometry/box-aspect.ts`, `src/geometry/half-side.ts`, `src/geometry/index.ts`, `src/geometry/is-sound-stretch.test.ts`, `src/geometry/is-sound-stretch.ts`, `src/geometry/mirror-across-axis.ts`, `src/geometry/stretch-log-bound.ts`, `src/geometry/stretch-scale.test.ts`, `src/geometry/stretch-scale.ts`, `src/geometry/stretched-polygon.test.ts`, `src/geometry/stretched-polygon.ts` and 9 more.
- **feat(model): a stretched option for the regular polygon, drawn by rule C**
  Refs: EN-017.T2
  Files: `playground/read-shape.ts`, `src/model/free-shape-with-midpoint.test.ts`, `src/model/free-shape-with-point.test.ts`, `src/model/is-valid-regular-polygon.test.ts`, `src/model/is-valid-regular-polygon.ts`, `src/model/regular-polygon-as-free-shape.test.ts`, `src/model/regular-polygon-contour.test.ts`, `src/model/regular-polygon-contour.ts`, `src/model/regular-polygon-corners.test.ts`, `src/model/regular-polygon.ts`, `src/model/turned-corners.test.ts`.
- **docs(model): the stretched option stored in the regular polygon**
  Refs: EN-017.T3
  Files: `EN-017`, `docs/domain/shapes.md`.
- **test(geometry): review fixes of the stretched polygon, its tall pentagon and rule C**
  Refs: EN-017.T1
  Files: `docs/derivations/stretched-polygon.md`, `src/geometry/stretch-scale.test.ts`, `src/geometry/stretch-scale.ts`, `src/geometry/stretched-polygon.test.ts`, `src/geometry/stretched-polygon.ts`, `src/math/bisection-steps.ts`, `src/math/bisection.ts`.
- **docs(backlog): set EN-017 done**
  Closes: EN-017
  Files: `F06`, `EN-017`, `docs/backlog/stories/README.md`.
- **docs(tooling): state line and pitfalls at the F06 checkpoint**
  Refs: CHK-009.T1
  Files: `.claude/rules/library.md`, `CLAUDE.md`.
- **docs(backlog): light evolvability check of F06**
  Refs: CHK-009.T2
  Files: `CHK-009`.
- **docs(backlog): review fixes of the F06 checkpoint, bracket ends guarded**
  Refs: CHK-009.T2
  Files: `CHK-009`, `docs/process/improvements.md`, `src/geometry/stretch-scale.test.ts`.
- **docs(backlog): set CHK-009 done**
  Closes: CHK-009
  Files: `F06`, `CHK-009`, `docs/backlog/stories/README.md`.
- **feat(playground): a Stretched check box for the polygon, its anchor disabled**
  Refs: US-021.T1
  Files: `playground/index.html`, `playground/listen-to-anchor.ts`, `playground/mount-playground.ts`, `playground/read-shape.ts`, `playground/show-shape.ts`, `playground/stretched-polygon.test.ts`, `src/model/effective-corner-radius.test.ts`, `src/model/effective-corner-radius.ts`.
- **docs(backlog): test card of US-021 for the Product Owner**
  Refs: US-021.T2
  Files: `US-021`.
- **fix(playground): review fixes of US-021, guided steps untick Stretched**
  Refs: US-021.T1
  Files: `US-021`, `playground/show-guided-step.ts`, `playground/stretched-polygon.test.ts`, `playground/write-stretched-value.ts`, `src/model/effective-corner-radius.ts`.
- **test(playground): the guided step unticks a ticked Stretched box**
  Refs: US-021.T1
  Files: `playground/stretched-polygon.test.ts`.
- **docs(backlog): set US-021 done**
  Closes: US-021
  Files: `F06`, `US-021`, `docs/backlog/stories/README.md`.
- **test(geometry): audit fixes of F06, sizes up to 100 000 and every λ**
  Refs: AUD-009.T1
  Files: `docs/derivations/stretched-polygon.md`, `docs/references.md`, `docs/research/notes/0011-f06-stretched-polygon.md`, `src/geometry/stretched-polygon.test.ts`, `src/geometry/stretched-side-directions.test.ts`, `src/geometry/unit-stretched-polygon.test.ts`.
- **docs(backlog): audit of F06, findings, next steps, evolvability**
  Refs: AUD-009.T2
  Files: `AUD-009`.
- **docs(backlog): set AUD-009 done**
  Closes: AUD-009
  Files: `F06`, `AUD-009`, `docs/backlog/stories/README.md`.
- **feat(playground): guided test of F06, steps 58 to 66**
  Refs: VAL-009.T2
  Files: `playground/guided-step.ts`, `playground/guided-steps-f06.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`, `playground/show-guided-step.ts`.
- **docs(docs): demo page of F06, rules C and B side by side**
  Refs: VAL-009.T3
  Files: `VAL-009`, `docs/guide/README.md`, `docs/guide/f06-stretched-polygon.md`, `docs/guide/images/f06-rule-b.svg`, `docs/guide/images/f06-rule-c.svg`.
- **docs(docs): journal of the F06 run stopped at VAL-009**
  Files: `docs/process/journal.md`.
- **docs(docs): rule C kept, rule B removed from the demo page of F06**
  Refs: VAL-009.T3
  Files: `US-021`, `docs/guide/README.md`, `docs/guide/f06-stretched-polygon.md`, `docs/guide/images/f06-rule-b.svg`, `docs/process/journal.md`.
- **docs(backlog): set VAL-009 and F06 done**
  Closes: VAL-009; Closes: F06
  Files: `CLAUDE.md`, `E01`, `F06`, `docs/backlog/features/README.md`, `VAL-009`, `docs/backlog/stories/README.md`.
- **chore(main): release 0.14.0**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the run stopped before the review of E01**
  Files: `CLAUDE.md`, `docs/process/journal.md`.
- **chore(main): release 0.14.1**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(backlog): set REV-001 ready**
  The Product Owner agreed: (agreed).
  Files: `E01`, `REV-001`, `docs/backlog/stories/README.md`.
- **docs(backlog): guided tests of E01 re-run on main**
  Refs: REV-001.T1
  Files: `REV-001`.
- **docs(docs): epic report of E01 and its user test, steps 67 to 72**
  The report sums up the eight features of E01 with their demo pages, the words, the decisions and the limits, and asks the Product Owner three questions. Its user test is in the playground too, each step crossing several features, tested end to end.
  Refs: REV-001.T2
  Files: `REV-001`, `docs/guide/README.md`, `docs/guide/e01-draw-shapes.md`, `playground/epic-1-steps.test.ts`, `playground/guided-steps-epic-1.ts`, `playground/guided-steps.ts`, `playground/mount-playground.test.ts`.
- **docs(backlog): review of E01 with the Product Owner, REV-001 done**
  Steps 67 to 72 showed what they meant to show. The closure criterion of E01 keeps the rectangle and the regular polygon only; E14 stays next. A shape seeming to leave the canvas at step 72, and the wording about Enter, go to the improvement log.
  Refs: REV-001.T3; Closes: REV-001
  Files: `E01`, `REV-001`, `docs/backlog/stories/README.md`, `docs/guide/e01-draw-shapes.md`, `docs/process/improvements.md`.
- **chore(main): release 0.14.2**
  Files: `.release-please-manifest.json`, `CHANGELOG.md`, `package-lock.json`, `package.json`.
- **docs(docs): journal of the review of E01**
  Files: `CLAUDE.md`, `docs/process/journal.md`.
- **docs(docs): improvement and feedback rows for the end of E01**
  Refs: RET-001.T1
  Files: `docs/process/feedback.md`, `docs/process/improvements.md`.
- **ci(ci): develop branch, checks run locally, one CI into main**
  ADR-0038 (proposed): main holds releases only, develop is the integration branch, feature branches start from it. Work branches are checked by the Git hooks only; the CI runs once, on the pull request into main, merged with a merge commit; the release pull request skips its jobs and the release attaches the test report of that CI instead of running the tests again. Dependency versions must be 7 days old (npm-check-updates and Dependabot cooldown); reading the release notes before an upgrade is a step of the procedure.
  Refs: RET-001.T2, ADR-0038, ADR-0028
  Files: `.claude/skills/review/SKILL.md`, `.claude/skills/run/SKILL.md`, `.claude/skills/story/SKILL.md`, `.github/dependabot.yml`, `.github/workflows/ci.yml`, `.github/workflows/codeql.yml`, `.github/workflows/release.yml`, `.github/workflows/story-automerge.yml`, `.gitignore`, `.ncurc.cjs`, `CLAUDE.md`, `docs/adr/0028-autonomous-runs.md` and 13 more.
- **chore(tooling): metrics of the agent's commands, tasks and stories**
  A background hook logs each tool call with its exact duration and outcome into the Git directory; npm run metrics reads that log and the Claude Code transcripts and prints commands by count and by time, the slowest calls, and active time and tokens per session, feature, story and task commit. Hooks and scripts run by Node.js name the .ts file in their imports (lint scope node-run) and use Temporal behind --harmony-temporal.
  Refs: RET-001.T2
  Files: `.claude/hooks/index.ts`, `.claude/hooks/log-tool-call.test.ts`, `.claude/hooks/log-tool-call.ts`, `.claude/rules/tooling.md`, `.claude/settings.json`, `docs/references.md`, `docs/tooling/README.md`, `docs/tooling/agent.md`, `docs/tooling/commands.md`, `docs/tooling/metrics.md`, `eslint/scopes/all.ts`, `eslint/scopes/tooling.ts` and 11 more.
- **docs(docs): journal of the start of RET-001**
  Refs: RET-001.T2
  Files: `docs/process/journal.md`.

## 2026-10-13

- **docs(adr): accept ADR-0038, merge commits for features and releases**
  The Product Owner accepted the develop branch and the local checks; features merge into develop and develop into main with merge commits, stories stay rebased; Dependabot version updates are dropped, security updates stay.
  Refs: RET-001.T2, ADR-0038
  Files: `.claude/skills/run/SKILL.md`, `.github/dependabot.yml`, `docs/adr/0038-develop-branch-and-local-checks.md`, `docs/adr/README.md`, `docs/tooling/ci.md`, `docs/tooling/git-workflow.md`, `docs/tooling/versions-and-security.md`.
- **chore(tooling): the bash guard skips here-document bodies**
  A commit message or a note written through a here-document is data: a guarded command quoted in it no longer denies the call.
  Refs: RET-001.T2
  Files: `.claude/hooks/guard-bash.test.ts`, `.claude/hooks/guard-bash.ts`, `.claude/rules/tooling.md`.
- **chore(tooling): status line with exact tokens, session report**
  The status line shows the agent's step, whether the session is active or silent, its running time and its exact tokens. npm run metrics:session reports the time, tokens, calls and commits of the current turn; each turn's last message ends with it.
  Refs: RET-001.T2
  Files: `.claude/hooks/status-line.test.ts`, `.claude/hooks/status-line.ts`, `.claude/settings.json`, `CLAUDE.md`, `docs/references.md`, `docs/tooling/metrics.md`, `package.json`, `scripts/metrics-events.test.ts`, `scripts/metrics-events.ts`, `scripts/session-report.test.ts`, `scripts/session-report.ts`.
- **feat(playground): a floating toolbar, grouped panels, a guided test per feature**
  The shapes and the modes sit in a toolbar over the drawing; the side panel groups the polygon's options, the size, the angle and position, the corners and the points, the pipeline output folded away. The guided test shows the steps of the feature being validated only, none between features; ?guide=epic gives every step of the epic for its review. The view keeps a margin of 40 so that a shape rounded a little past 0 stays in sight.
  Refs: RET-001.T2
  Files: `.claude/agents/functional-analyst.md`, `.claude/agents/math-expert.md`, `.claude/agents/verifier.md`, `playground/current-guided-steps.ts`, `playground/delete-points.test.ts`, `playground/edit-turned-shape.test.ts`, `playground/epic-1-steps.test.ts`, `playground/free-shape-editing.test.ts`, `playground/guided-step-address.test.ts`, `playground/index.html`, `playground/index.ts`, `playground/listen-to-modes.ts` and 15 more.
- **chore(tooling): a local library of the sources read, behind an MCP server**
  ADR-0039: the pages the agent reads online are stored as Markdown in a SQLite file of the machine, split at their headings and indexed with FTS5; library_search, library_get and library_read search it, fetch a missing or stale page, and read only the sections needed. No dependency: node:sqlite and a small stdio MCP server.
  Refs: RET-001.T2, ADR-0039
  Files: `.claude/settings.json`, `.mcp.json`, `docs/adr/0024-session-journal.md`, `docs/adr/0039-local-source-library.md`, `docs/adr/README.md`, `docs/references.md`, `docs/research/README.md`, `docs/tooling/sources.md`, `package.json`, `scripts/library-chunks.test.ts`, `scripts/library-chunks.ts`, `scripts/library-cli.ts` and 7 more.
- **feat(playground): reset, a pipeline column, valid fields after a free shape**
  The toolbar gets Reset, back to the rectangle the page opens with, and Pipeline, which shows or hides the pipeline output in a column of its own. Choosing a rectangle or a polygon after a free shape rounds the width, height and position and gives every corner one radius, instead of leaving the fields red. The angle buttons and the fields line up.
  Refs: RET-001.T2
  Files: `playground/index.html`, `playground/listen-to-toolbar.ts`, `playground/show-guided-step.ts`, `playground/toolbar.test.ts`, `playground/write-default-shape.ts`, `playground/write-step-values.ts`, `playground/write-whole-shape-fields.ts`.
- **chore(tooling): knip, a fitness function, a playground coverage floor**
  knip reports unused files, exports and dependencies in npm run check; the barrels of scripts/ and .claude/hooks/, which nothing imports, go. Every math and geometry function needs a property test, nine listed until EN-018. The playground gets a coverage floor of its own. package.json is reordered, lint-staged and the branch rule move to their own files, the repository-wide tests to tests/, the overrides checked and kept.
  Refs: RET-001.T2
  Files: `.claude/hooks/index.ts`, `.lintstagedrc.json`, `.validate-branch-namerc.json`, `docs/conventions/testing.md`, `docs/tooling/commands.md`, `docs/tooling/dependencies.md`, `docs/tooling/git-workflow.md`, `eslint/rules/local/modules.ts`, `eslint/scopes/tooling.ts`, `eslint/settings/files.ts`, `eslint/settings/layers.ts`, `knip.json` and 11 more.
- **docs(docs): writing rules for scanning readers and for the Product Owner's words**
  Refs: RET-001.T3
  Files: `docs/conventions/writing.md`, `docs/references.md`, `eslint/rules/markdown/structure.ts`.
- **docs(docs): domain and guide pages without the Product Owner's words**
  Decisions keep their reasons in neutral English; each demo page ends with a validation table: remark, and what was done or where it went. Links to the guided steps of E01 use ?guide=epic.
  Refs: RET-001.T3
  Files: `US-011`, `US-012`, `US-013`, `US-018`, `US-019`, `US-014`, `US-015`, `US-016`, `US-017`, `docs/domain/README.md`, `docs/domain/editions.md`, `docs/domain/interaction.md` and 13 more.
- **docs(docs): backlog, decisions and notes without the Product Owner's words**
  Validation sections state what was checked and what came of each remark; refinement decisions keep their reasons in neutral English; labels of the interface are written in bold.
  Refs: RET-001.T3
  Files: `docs/adr/0030-free-v1-then-pro-edition.md`, `docs/adr/0031-free-shape-keeps-fractional-vertices.md`, `docs/adr/0032-rectangle-is-a-free-shape.md`, `docs/adr/0033-free-shape-of-corners.md`, `docs/adr/0034-only-the-coordinate-typed-is-an-integer.md`, `docs/adr/0035-angle-and-position-for-every-shape.md`, `docs/adr/0037-stretched-polygon.md`, `E02`, `E03`, `E13`, `F03`, `F04` and 29 more.
- **docs(docs): diagrams written in Mermaid, the method on one page**
  A mermaid code block is drawn by the site's theme and by GitHub: the feature lifecycle, the branch history and the source library's flow are text kept with their pages. The method page explains the levels, the story types, who does what, and the cadences, among them a review of every rule every four to five epics.
  Refs: RET-001.T3
  Files: `docs/.vitepress/theme/draw-diagrams.ts`, `docs/.vitepress/theme/index.ts`, `docs/conventions/writing.md`, `docs/process/README.md`, `docs/process/agile.md`, `docs/references.md`, `docs/tooling/dependencies.md`, `docs/tooling/documentation-site.md`, `docs/tooling/git-workflow.md`, `docs/tooling/sources.md`, `eslint/settings/files.ts`, `knip.json` and 3 more.
- **docs(backlog): plan the debt of E01 in F11, before curves; where we are**
  A feature of E14, first in its order: shared test helpers, one path writer, property tests for the nine functions without one, and the folder layout researched before the files move. Its stories, the review and the retrospective of E14 are drafts. The backlog's front page says which epic is done, current and next.
  Refs: RET-001.T3
  Files: `docs/backlog/README.md`, `E14`, `F11`, `docs/backlog/features/README.md`, `AUD-010`, `CHK-010`, `EN-020`, `EN-021`, `EN-022`, `SP-010`, `VAL-010`, `RET-002` and 4 more.
- **chore(tooling): a status line of three lines, with the backlog place and the call running**
  The first line names the epic, feature and story of the branch and the agent's step; the second, the session: active or silent, running time, exact tokens, how full the conversation memory is, the model; the third, the tool call running and for how long, written by a new PreToolUse hook. Refreshed every second; usage limits left out, Claude Code shows them.
  Refs: RET-001.T2
  Files: `.claude/hooks/log-tool-call.ts`, `.claude/hooks/status-line.test.ts`, `.claude/hooks/status-line.ts`, `.claude/hooks/status-place.test.ts`, `.claude/hooks/status-place.ts`, `.claude/hooks/status-tokens.ts`, `.claude/hooks/track-tool-call.ts`, `.claude/settings.json`, `docs/references.md`, `docs/tooling/agent.md`, `docs/tooling/metrics.md`, `knip.json`.
- **chore(tooling): rules for any agent in AGENTS.md, five expert agents, skills updated**
  The rules any coding agent needs move to AGENTS.md, which CLAUDE.md imports before what only Claude Code uses. A doc-editor agent joins the auditor, math-expert, verifier and functional-analyst; the skills start them in the background, search the source library first, keep the terminal quiet during a run, write results in neutral words, and read the metrics at each retrospective.
  Refs: RET-001.T3
  Files: `.claude/agents/doc-editor.md`, `.claude/skills/audit/SKILL.md`, `.claude/skills/derivation/SKILL.md`, `.claude/skills/retro/SKILL.md`, `.claude/skills/review/SKILL.md`, `.claude/skills/run/SKILL.md`, `.claude/skills/spike/SKILL.md`, `.claude/skills/story/SKILL.md`, `AGENTS.md`, `CLAUDE.md`, `docs/references.md`, `docs/tooling/agent.md`.
- **docs(docs): the playbook after E01: principles, starter kit, interview, lessons**
  Refs: RET-001.T3
  Files: `docs/playbook/README.md`, `docs/playbook/agent.md`, `docs/playbook/analysis.md`, `docs/playbook/lessons.md`, `docs/playbook/principles.md`, `docs/playbook/process.md`, `docs/playbook/setup.md`.
- **docs(docs): retrospective record of E01; logs emptied, journal condensed**
  Every item of the improvement and feedback logs is decided in the record, with where it went; the rows still to do later start the new improvement log; the feedback rows became the playbook's sheet of habits.
  Refs: RET-001.T3
  Files: `docs/process/feedback.md`, `docs/process/improvements.md`, `docs/process/journal.md`, `docs/process/retrospectives/E01-retrospective.md`, `docs/process/retrospectives/README.md`.
- **fix(docs): diagrams drawn without the XML error, every diagram parsed by a test**
  Mermaid writes HTML inside its SVG labels (a \<br> without a closing slash), which the XML parser refused: the drawing is now read as HTML. A test parses every Mermaid block of the docs. The README, the home page, the history before 0.1.0 and the last stale words follow the new workflow.
  Refs: RET-001.T3
  Files: `.claude/hooks/guard-bash.ts`, `.claude/skills/audit/SKILL.md`, `.claude/skills/run/SKILL.md`, `README.md`, `docs/.vitepress/theme/draw-diagrams.test.ts`, `docs/.vitepress/theme/draw-diagrams.ts`, `docs/backlog/_checkpoint.md`, `docs/backlog/_review.md`, `REV-001`, `REV-002`, `docs/conventions/audit.md`, `docs/conventions/testing.md` and 7 more.
- **docs(backlog): set RET-001 ready, its first and last tasks done**
  Refs: RET-001
  Files: `AGENTS.md`, `E01`, `RET-001`, `docs/backlog/stories/README.md`.
- **docs(docs): second pass after the doc-editor reviews; ADR-0040**
  Three independent reviews of every folder: the last wording that read as the Product Owner's, pages still describing the old workflow, long cells split into lists, stale feature states, the references grouped. ADR-0040 records AGENTS.md; ADR-0038 names Dependabot's security updates as the one exception to releases only on main.
  Refs: RET-001.T3, ADR-0040
  Files: `.claude/rules/library.md`, `.claude/rules/tooling.md`, `.claude/skills/retro/SKILL.md`, `.claude/skills/review/SKILL.md`, `.claude/skills/run/SKILL.md`, `docs/adr/0001-geometric-scope.md`, `docs/adr/0003-integer-model.md`, `docs/adr/0004-orthogonal-pipes.md`, `docs/adr/0007-corner-radius-clamping.md`, `docs/adr/0009-toolchain.md`, `docs/adr/0010-lint-first-strictness.md`, `docs/adr/0011-function-kinds.md` and 65 more.
- **chore(tooling): count the uses of the source library**
  Each search and each page read is recorded; npm run library -- stats and the Sources section of npm run metrics rank the sources consulted most, the words searched, and the share of pages served without going online. The retrospective reads them and asks why, then writes the agent's own account of the epic.
  Refs: RET-001.T3
  Files: `.claude/skills/retro/SKILL.md`, `docs/tooling/sources.md`, `scripts/library-cli.ts`, `scripts/library-mcp.ts`, `scripts/library-store.ts`, `scripts/library-usage.test.ts`, `scripts/library-usage.ts`, `scripts/metrics-report.ts`.
