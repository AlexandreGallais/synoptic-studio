# ADR-0043 — Folders by business domain, files and folders never side by side

**Status**: Accepted (Product Owner, F11, 2026-10-13). Supersedes ADR-0042.

## Context

ADR-0042 named folders by theme and made 15 modules a signal. EN-022 applied it, but left two things the Product Owner refused at the stop of the F11 run:

- `src/math/` held `vector/` and seven modules beside it, `src/geometry/` three folders and eleven modules, `playground/` nine folders and its wiring: a reader opening such a folder sees two kinds of entry and no rule for which goes where.
- Folders were cut to satisfy a count (`box/`, `polygon/`, the types left at the root) rather than along the business. A library is divided into the domains of its business; the count only says when a folder may hide a domain not yet named.

## Decision

1. **Folders are named after the business domains of `docs/domain/`**, the same name in every layer where the domain has code: `corner-radius/` in `geometry/` and `model/` (the corner radius of `shapes.md` §2), `regular-polygon/`, `placement/` (position, turn, straight and turned boxes: `shapes.md` §1 and the glossary), `layout-box/` (the layout box of Q21: alignment, anchor, fit, stretch, size, and their checks in `model/`), `contour/` (the pieces of a contour, `shapes.md` §1), `selection/` (the corners chosen, `interaction.md`). `math/` has no business: its folders name mathematical domains (`vector/`, `angle/`, `bisection/`, `precision/`). The playground's domains are its panels. A new domain gets its folder when its first module is written (the ellipse of E14).
2. **A code folder holds modules or domain folders, never both**: only the barrel `index.ts` (and the playground's entry, `index.ts` with `index.html`) sits beside folders. Tests count as files: a test of several domains goes to the domain it is mainly about, or to `tests/` when it spans the project.
3. **15 modules per folder is a signal**, tests and barrel not counted: a folder past it asks whether a domain is missing. It is split along that domain, or listed with its reason in `docs/conventions/files.md`.
4. Kept from ADR-0042: tests beside their module; a shape's type in its folder; folders may use each other's types, their value imports go one way (`import-x/no-cycle`); no folder by kind of code (`types/`, `utils/`).
5. **`local/folder-layout` checks rules 2 and 3 for TypeScript files, in the editor and in `npm run lint`**; `tests/folders.test.ts` checks rule 2 for every other file, keeps the rule's list of folders past the signal (`eslint/settings/folders.ts`) and the page equal, and checks that the rule is switched on.
6. The domains and the signal are reviewed with every rule, every four to five epics (`docs/process/agile.md`).

## Consequences

- EN-023 moves `math/`, `geometry/`, two tests of `model/` and the playground's wiring (`playground/mount/`), and renames `rounded-corners/`, `radii/`, `polygon/` and `box/` after their domain.
- A file added beside folders, or a 16th module, fails the lint at once, in the editor, not only at the push.
- Imports of `src/` from outside are unchanged: the barrels re-export every folder.

## References

ADR-0042 and its sources (`REF-ANGULAR-STYLE`, `REF-ANGULAR-STYLE-17`, `REF-KCD-COLOCATION`, `REF-VITEST-INCLUDE`, `REF-SCREAMING-ARCHITECTURE`), ADR-0012, ADR-0019, `docs/domain/shapes.md`
