# 0012 — Sources of F11: folders and test placement

Spike SP-010, 2026-10-13. What the sources say about laying out a TypeScript library and its tests, set against this project's rules: one export per file (ADR-0019), a barrel per folder (ADR-0012), canonical imports, layer boundaries, `import-x/no-cycle`, 100 % coverage of `src/`.

## Where tests go

| Source               | What it says                                                                                                                                                                      |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `REF-ANGULAR-STYLE`  | "Unit tests should live in the same directory as the code-under-test. Avoid collecting unrelated tests into a single `tests` directory."                                          |
| `REF-KCD-COLOCATION` | "Place code as close to where it's relevant as possible"; tests sit with the files they test; end-to-end tests, which span the whole project, "make more sense to go at the root" |
| `REF-VITEST-INCLUDE` | default `include`: `**/*.{test,spec}.?(c\|m)[jt]s?(x)`, anywhere in the tree; coverage leaves the test files out by itself                                                        |

**Finding**: the current placement already follows the sources — a module's test beside it, the repository-wide tests (backlog, bibliography, barrels, diagrams, dependencies, fitness) in `tests/`, which match the exception for tests that span the project. Nothing to move; the tests move with their module in EN-022 (`git mv` of both).

## How folders are split

| Source                       | What it says                                                                                                                                                                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `REF-ANGULAR-STYLE`          | "Organize your project into subdirectories based on the features of your application or common themes"; "Avoid creating subdirectories based on the type of code"; "As the number of files in a directory grows, consider splitting further"                        |
| `REF-ANGULAR-STYLE-17`       | Style 04-04: "Do keep a flat folder structure as long as possible. Consider creating sub-folders when a folder reaches seven or more files"; "when a folder has ten or more files, it may be time to create subfolders"; "Base your decision on your comfort level" |
| `REF-SCREAMING-ARCHITECTURE` | the top-level folders should tell what the system does ("do they scream: Health Care System"), not which framework it uses                                                                                                                                          |

**Finding**: split by theme, never by kind of code (`types/`, `validators/`). The figure of seven to ten files is given for Angular, where a file holds a whole component; here a file holds one function (ADR-0019), so the same reading effort is reached later. No source gives a limit for one-function files: the limit is the Product Owner's choice.

## What the project's rules allow

- **Layers**: `import-x/no-restricted-paths` zones target `./src/model` and the other layers as folders: a subfolder belongs to its layer without any change (`eslint/settings/layers.ts`).
- **Barrels**: `local/barrel-exports` already lists a subfolder with an `index.ts` in its parent's barrel; `tests/barrels.test.ts` walks every `index.ts` below `src/`.
- **Imports**: `local/canonical-import-path` imports a sibling subfolder through its barrel (`../free-shape`) and refuses the parent's own barrel (a cycle).
- **Cycles**: `import-x/no-cycle` counts type imports. Two subfolders that use each other's types make a cycle through their barrels. Today `turned-corners` and `is-shape-placement` import the type `FreeShape`, and the free-shape functions import them: a `placement/` subfolder and a `free-shape/` subfolder would cycle. **Rule found**: the shape types (`FreeShape`, `Rectangle`, `RegularPolygon`, `Corner`, `ContourPiece`…) stay at the root of their layer; subfolders hold the functions and import the types from the parent (`../free-shape`), never the other way round.

## Layout proposed (moves done in EN-022)

Counts are modules, tests not counted (each test follows its module).

| Folder                             | Today    | Proposed                                                                                                                                                                                                               |
| ---------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/model`                        | 49       | root: the 3 shape types; `rectangle/` 5, `regular-polygon/` 6, `free-shape/` 12, `radii/` 5, `placement/` 8, `selection/` 10                                                                                           |
| `src/geometry`                     | 40       | root: the types (`arc`, `segment`, `contour-piece`, `corner`, `corner-points`, `size`, `bounding-box`, `alignment`, `alignments`, `anchor`) and `cyclic-item`; `rounded-corners/` 11, `box/` 9, `stretched-polygon/` 9 |
| `playground`                       | 77       | root: entry, mount, update; `guided-test/` 17, `shape/` (kind, sizes, validity, drawing) about 20, `points/` (list, fields, selection, delete) about 20, `position/` (position, anchor, angle, boxes) about 17         |
| `src/math`, `src/io`, `src/render` | 18, 7, 5 | unchanged                                                                                                                                                                                                              |

The exact lists are fixed in EN-022, checked by `import-x/no-cycle`; a folder over the chosen limit is split again by theme.

## Decision of the Product Owner

On 2026-10-13 (ADR-0042): folders by business theme first; 15 modules as a signal, a folder past it split only along a theme that makes sense, else listed with its reason; the playground split by panel now; the limit reviewed with every rule, every four to five epics.

## Open

- None for EN-020 and EN-021: they need no new source (property tests follow ADR-0022 and `REF-FASTCHECK-PBT`; the shared helpers and the single path writer change no formula).
