# ADR-0042 — Folders by theme, 15 modules as a signal, a shape's type in its folder

**Status**: Accepted (Product Owner, SP-010, 2026-10-13).

## Context

E01 left `src/model/` with 49 modules, `src/geometry/` with 40 and `playground/` with 77, each flat: one export per file (ADR-0019) makes many small files, and a reader no longer finds a shape's functions at a glance. F11 prepares the code base before curves add a new kind of point.

Sources (research note 0012): tests belong beside the code they test, and tests that span the project at the root (`REF-ANGULAR-STYLE`, `REF-KCD-COLOCATION`; Vitest finds them anywhere, `REF-VITEST-INCLUDE`); folders are split by feature or theme, never by kind of code (`REF-ANGULAR-STYLE`), and the top folders tell what the system does (`REF-SCREAMING-ARCHITECTURE`); Angular advises subfolders from seven to ten files, "based on your comfort level" (`REF-ANGULAR-STYLE-17`). No source gives a limit for one-function files (ADR-0019).

The Product Owner wants folders that make sense in the business, not folders made to satisfy a number; a limit is still needed, as a signal.

## Decision

1. **Tests stay beside their module**; the project-wide tests stay in `tests/`. A module and its test move together (`git mv`).
2. **Folders are named by business theme** — a shape (`rectangle/`, `regular-polygon/`, `free-shape/`), a concern of the editor (`selection/`, `placement/`, `radii/`), a construction (`rounded-corners/`, `stretched-polygon/`), a panel of the playground (`guided-test/`, `points/`…) — never by kind of code (`types/`, `validators/`, `utils/`).
3. **15 modules per folder is a signal, not a rule to split blindly**: tests do not count. A folder past 15 is split only along a theme that makes sense; when no such theme exists, it stays, listed with its reason in `docs/conventions/files.md`. A test fails on a folder past 15 that is not listed.
4. **A shape's type lives in its folder**, with its functions (`free-shape/free-shape.ts`): a file and a folder of the same name cannot sit side by side, since an import of `../free-shape` reaches the file first. Folders may use each other's types (`import type`, which `import-x/no-cycle` ignores); their value imports go one way, which it checks. Types that every folder of a layer needs and that name no theme (`Arc`, `Segment`, `ContourPiece`, `Corner`, `Size`…) stay at the root of `src/geometry/`. Decided after the review of SP-010, which found the name clash and that type imports make no cycle for the lint.
5. `src/math/`, `src/io/` and `src/render/` stay flat while under 15. The playground is split by panel now.
6. The limit and the themes are reviewed with every rule, every four to five epics (`docs/process/agile.md`).

## Consequences

- EN-022 moves the files, writes the layout and the list of exceptions in `docs/conventions/files.md`, and adds the folder test.
- Imports get one level longer (`../free-shape`, `./radii`); the canonical-import rule and the barrels already handle subfolders.
- A new kind of shape (the ellipse of E14) gets its folder from the start.

## References

`REF-ANGULAR-STYLE`, `REF-ANGULAR-STYLE-17`, `REF-KCD-COLOCATION`, `REF-VITEST-INCLUDE`, `REF-SCREAMING-ARCHITECTURE`, ADR-0012, ADR-0019, [research note 0012](../research/notes/0012-f11-folders-and-tests.md)
