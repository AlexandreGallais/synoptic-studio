# 0012 — Sources of F11: folders and test placement

Spike SP-010, 2026-10-13. What the sources say about laying out a TypeScript library and its tests, set against this project's rules: one export per file (ADR-0019), a barrel per folder (ADR-0012), canonical imports, layer boundaries, `import-x/no-cycle`, 100 % coverage of `src/`.

## Where tests go

| Source               | What it says                                                                                                                                                                                                                                                                            |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `REF-ANGULAR-STYLE`  | "Unit tests should live in the same directory as the code-under-test. Avoid collecting unrelated tests into a single `tests` directory."                                                                                                                                                |
| `REF-KCD-COLOCATION` | "Place code as close to where it's relevant as possible"; unit tests sit "with the file or group of files they are testing"; tests spanning several modules go in their feature's folder; end-to-end tests, which "span beyond the project itself", "make more sense to go at the root" |
| `REF-VITEST-INCLUDE` | default `include`: `**/*.{test,spec}.?(c\|m)[jt]s?(x)`, anywhere in the tree; coverage leaves the test files out by itself                                                                                                                                                              |

**Finding**: modules and their tests already follow the sources: each test sits beside its module. The repository-wide checks in `tests/` (backlog, bibliography, barrels, diagrams, dependencies, fitness) test no module of `src/`: keeping them apart is the project's own reading, not a case the sources describe (Angular advises against a `tests` folder of unrelated unit tests, which these are not). Nothing to move; each test moves with its module in EN-022 (`git mv` of both).

## How folders are split

| Source                       | What it says                                                                                                                                                                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `REF-ANGULAR-STYLE`          | "Organize your project into subdirectories based on the features of your application or common themes"; "Avoid creating subdirectories based on the type of code"; "As the number of files in a directory grows, consider splitting further"                        |
| `REF-ANGULAR-STYLE-17`       | Style 04-04: "Do keep a flat folder structure as long as possible. Consider creating sub-folders when a folder reaches seven or more files"; "when a folder has ten or more files, it may be time to create subfolders"; "Base your decision on your comfort level" |
| `REF-SCREAMING-ARCHITECTURE` | the top-level folders should tell what the system does ("do they scream: Health Care System"), not which framework it uses                                                                                                                                          |

**Finding**: split by theme, never by kind of code (`types/`, `validators/`). Angular's seven to ten files count every file of a folder; no source gives a limit for one-function files (ADR-0019), so the limit is the Product Owner's choice.

## What the project's rules allow

- **Layers**: `import-x/no-restricted-paths` zones target `./src/model` and the other layers as folders: a subfolder belongs to its layer without any change (`eslint/settings/layers.ts`).
- **Barrels**: `local/barrel-exports` already lists a subfolder with an `index.ts` in its parent's barrel; `tests/barrels.test.ts` walks every `index.ts` below `src/`.
- **Imports**: `local/canonical-import-path` imports a sibling subfolder through its barrel (`../placement`) and refuses the parent's own barrel. A file and a folder of the same name in one folder cannot both be reached: `free-shape.ts` beside `free-shape/` resolves to the file (`eslint/plugin/utils/paths.ts`, TypeScript's `bundler` resolution), so a shape's type lives inside its folder (`free-shape/free-shape.ts`).
- **Cycles**: `import-x/no-cycle` ignores type-only imports, which the project always writes as `import type` (`node_modules/eslint-plugin-import-x/lib/rules/no-cycle.js`, `consistent-type-imports`). Two folders may use each other's types; their value imports must go one way. Today `turned-corners` and `is-shape-placement` (`placement/`) import the type `FreeShape`, and the free-shape functions call them: allowed, no cycle at run time.

## Layout proposed (moves done in EN-022)

Counts are modules, tests not counted (each test follows its module).

| Folder                             | Today    | Proposed                                                                                                                                                                                                               |
| ---------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/model`                        | 49       | `rectangle/` 6, `regular-polygon/` 7, `free-shape/` 13 (each with its type), `radii/` 5, `placement/` 8, `selection/` 10; the root keeps its barrel only                                                               |
| `src/geometry`                     | 40       | root: the types (`arc`, `segment`, `contour-piece`, `corner`, `corner-points`, `size`, `bounding-box`, `alignment`, `alignments`, `anchor`) and `cyclic-item`; `rounded-corners/` 11, `box/` 9, `stretched-polygon/` 9 |
| `playground`                       | 77       | root: entry, mount, update; `guided-test/` 17, `shape/` (kind, sizes, validity, drawing) about 20, `points/` (list, fields, selection, delete) about 20, `position/` (position, anchor, angle, boxes) about 17         |
| `src/math`, `src/io`, `src/render` | 17, 6, 4 | math split in EN-022 (`vector/`), see [EN-022](../../backlog/stories/E14-F11-EN-022-folder-layout.md); io and render unchanged                                                                                         |

The exact lists are fixed in EN-022; `import-x/no-cycle` checks that value imports between folders go one way. A folder over the limit is split again by theme, or listed with its reason.

## Decision of the Product Owner

On 2026-10-13 (ADR-0042):

| Topic      | Retained                                                               | Source                             |
| ---------- | ---------------------------------------------------------------------- | ---------------------------------- |
| Folders    | named by business theme, never by kind of code                         | `REF-ANGULAR-STYLE`, Product Owner |
| Limit      | 15 modules a signal; past it, split by theme or listed with its reason | Product Owner                      |
| Shape type | in its folder, after the review of SP-010 found the name clash         | review of SP-010                   |
| Playground | split by panel now                                                     | Product Owner                      |
| Review     | the limit and the themes with every rule, every four to five epics     | `docs/process/agile.md`            |

## Open

Nothing open: EN-020 and EN-021 need no new source (property tests follow ADR-0022 and `REF-FASTCHECK-PBT`; the shared helpers and the single path writer change no formula).
