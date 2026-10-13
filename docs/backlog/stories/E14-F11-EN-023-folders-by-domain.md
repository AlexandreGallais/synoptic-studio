---
id: EN-023
epic: E14
feature: F11
title: Folders by business domain, files and folders never side by side
status: done
points: 2
---

# E14 · F11 · EN-023 — Folders by business domain, files and folders never side by side

Asked by the Product Owner at the stop of the F11 run (2026-10-13), who set it ready in the same message: EN-022 left modules beside folders (`src/math/`, `src/geometry/`, `playground/`) and cut some folders to satisfy a count rather than along the business. ADR-0043 supersedes ADR-0042.

## Acceptance criteria

- Given a code folder of `src/` or `playground/`, when the story ends, then it holds modules or domain folders, never both, only its barrel (and the playground's entry) beside folders.
- Given the folders of `geometry/` and `model/`, when the story ends, then each is named after a domain of `docs/domain/`, the same name in each layer where the domain has code.
- Given a file added beside folders, or a 16th module in a folder not listed, when the lint runs, then `local/folder-layout` reports it, in the editor too.
- Given the moves, when `npm run check:all` runs, then it is green: the 838 tests of before pass with the same values, beside the rule's and the folder test's own (853 in all), and the public exports of `src/` are unchanged.

## Tasks

One task = one commit, referenced as `EN-023.Tn`.

- [x] T1 — Move the modules beside folders into domain folders: `math/`, `geometry/`, the tests of `model/`, the playground's wiring (1 h)
- [x] T2 — Name the folders of `geometry/` and `model/` after the domain (1 h)
- [x] T3 — ADR-0043, the rule `local/folder-layout`, the folder test and the conventions (1 h)
- [x] T4 — The review's findings: four modules to their domain, the rule's missing cases and its wiring, files other than TypeScript (1 h)

## Decisions during the story

| Decision                                                                                                                                     | Reason                                                                                                                                                              |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `math/` kept as a layer, its folders named after mathematical domains                                                                        | the layers of ADR-0005 and ADR-0014 are what the lint checks for purity; math has no business, so its domains are mathematical: vector, angle, bisection, precision |
| `geometry/box/` split into `layout-box/` and `placement/`                                                                                    | the layout box (Q21) and the straight box of a turned shape (F04) are two domains of `docs/domain/shapes.md`; `placement/` now matches `model/placement/`           |
| `rounded-corners/` and `model/radii/` both renamed `corner-radius/`, `polygon/` renamed `regular-polygon/`                                   | the same domain carries the same name in every layer                                                                                                                |
| the playground's wiring in `mount/`, not in `page/`                                                                                          | `page/` is used by every panel and `mount/` uses every panel: one folder would import itself through the panels (`import-x/no-cycle`)                               |
| `page/` kept: the fields every panel reads and writes                                                                                        | decided by the agent, as the Product Owner asked, rather than left as a question                                                                                    |
| `isAnchor` and `isModelSize` in `model/layout-box/`, `boxAspect` in `geometry/regular-polygon/`, the angle's field in `playground/position/` | found by the review: each served the domain of another folder (the anchor and the size of the layout box, the stretched polygon, the Angle and position panel)      |
| the list of folders past 15 in `eslint/settings/folders.ts`, the page kept equal by a test                                                   | the rule must read its list without parsing Markdown; the test keeps the reasons on the page                                                                        |

## Evidence

- Mutation checks: a module `src/math/x.ts` beside folders is reported (`mixed`); `user-actions` taken off the list is reported (`past`) and the folder test fails; a wrong count in `files.md` fails the folder test.
- The 139 names exported under `src/` are the same before and after (scripted comparison).

## Review

Auditor and verifier, before the last commit. Fixed: four modules in the folder of another domain (above); the rule's boundary at exactly 15, a barrel in the count, a nested listed path, the data of `mixed`, hidden and tool folders (`.cache/`, `__snapshots__/`); a test that the rule is switched on for `src/` and `playground/` (mutation-checked: the scope removed, three tests fail); a walk of the tree for files the lint does not read (mutation-checked with a `.css` and a `.json`); `testing.md`'s path of the playground tests. Not fixed: the `\\` normalization of Windows paths is untestable on the project's systems; a duplicated row of the list in `files.md` would pass the test (low value).
