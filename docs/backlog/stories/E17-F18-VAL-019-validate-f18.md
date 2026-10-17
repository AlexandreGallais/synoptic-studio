---
id: VAL-019
epic: E17
feature: F18
title: Validate F18
status: ready
points: 1
---

# E17 · F18 · VAL-019 — Validate F18

Show the Product Owner F18 on one demo page: the figures before and after (subagents refused, heavy commands queued, conflicts on shared files, `check:all` under load), and US-061 on the checklist.

## Acceptance criteria

- Given the demo page, when the Product Owner reads it, then each criterion of F18 is shown with its evidence.
- Given the checklist, when the Product Owner tries US-061's lines, then the font list behaves as its test card says.

## Decisions to confirm

- US-061: a face refused when its family is read again is dropped, and a text asking for it is measured with the nearest face held, as for a weight the family lacks; the alternative, only saying the refusal, would have measured a face then left it out of the export (rule of US-060). Chosen by the agent on 2026-10-17, the choice left to it by the criterion.
- ADR-0067, decision 2: before each release, the Product Owner merges release-please's release pull request into `develop`; only then the pull request `develop` → `main`.
- EN-048: tables without padding moved to F19's formatter (ADR-0068); F18.AC4 keeps the logs merged by union, rows kept unique.
- EN-062: the tests on Helvetica, Apple Color Emoji, Arial and STIXGeneral stay macOS-only; F18.AC19 is read as the font reader's own tests on Inter Regular, which run everywhere.
- US-062: the faces refused at the first reading are said once it ends, after the families left out, counted by reason and named, without the browser's reason: `Installed fonts read: 153 families; 224 left out: …. 3 faces of the families read refused: 1 not embeddable (STSongti-SC-Black); 2 no Latin letters (HoeflerText-Ornaments, IowanOldStyle-Titling).`; such a face is never read again, so the line `…: 1 faces read, 1 refused (…)` now appears only for a font removed between the two readings. The agent's wording, 2026-10-17.

## Decisions confirmed

The Product Owner answered the decisions above on 2026-10-17, before the checklist was tried.

| Decision                 | Answer                                                                                                                                                                                                                                                                                                |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| US-061, a refused face   | confirmed: a face refused at the second reading of its family is dropped, and a text asking for it is measured with the nearest face held; every face measured can then be embedded at export                                                                                                         |
| ADR-0067, decision 2     | confirmed: the Product Owner merges release-please's release pull request into `develop` first, then the pull request `develop` → `main`, #259 for 0.3; on `main` release-please only tags and publishes                                                                                              |
| ADR-0068, EN-048         | confirmed: the Markdown tables without padding move to F19's formatter; F18 keeps the append-only logs merged by union, their rows kept unique                                                                                                                                                        |
| EN-062, macOS-only tests | not accepted: the tests check the code, not a font installed on one system, so none may depend on macOS's fonts. EN-067 was added to F18 and made every font test run on every system: a collection and a 191 MB table built from Inter Regular's tables, and a second family built from Inter's file |

## Tasks

One task = one commit, referenced as `VAL-019.Tn`.

- [x] T1 — One test per criterion of F18, checked with F18 counted done (0.5 h)
- [x] T2 — F18's lines in E17's checklist, `?checklist=E17` (1 h)
- [x] T3 — The decisions the Product Owner confirmed on 2026-10-17 recorded (0.5 h)
- [x] T4 — The demo page, `docs/guide/f18-safe-and-cheap-parallel-run.md` (1 h)
- [x] T5 — E15's lines that ask for macOS font files reviewed (0.5 h)
- [ ] T6 — The Product Owner validates; the result recorded (1 h)

## Criteria traced

Each of the 20 criteria of F18 has at least one test titled or named after it, `[F18.ACn]` in a TypeScript test or `f18_acn_…` in a Rust tool's tests (2026-10-17): AC2 in `tools/hooks/tests/limit.rs` and `tasks.rs`, AC19 in ten font test files, the others in one to five files each. With F18's status set to `done` in a copy of its file, the criteria trace test of `tests/backlog.test.ts` passes; with a 21st criterion added and no test for it, it fails naming `F18.AC21`.

## E15's lines that ask for macOS font files

They are F15's and F17's lines, not F18's: left as they are, to be tried on a Mac, or rewritten when E15's checklist is next reworked.

| File                                                                  | Line                                                                                                                                                            |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `playground/checklists/epic-15-color-and-font-lines.ts`               | F15: Import Arial from `/System/Library/Fonts/Supplemental`; F15: Import Helvetica.ttc from `/System/Library/Fonts`; F17 (US-052): Import Apple Color Emoji.ttc |
| `playground/guided-steps/epic-15/guided-steps-f15.ts`                 | Arial.ttf from `/System/Library/Fonts/Supplemental`, Arial Bold.ttf and Arial Italic.ttf, Helvetica.ttc from `/System/Library/Fonts`                            |
| `playground/guided-steps/epic-15/guided-steps-f15-first-decisions.ts` | Arial Bold Italic.ttf, Arial.ttf twice, Arial.ttf and Arial Bold.ttf after a reload, STIXGeneral.otf from `/System/Library/Fonts/Supplemental`                  |
| `playground/guided-steps/epic-15/guided-steps-f15-last-decisions.ts`  | Arial.ttf, Arial Bold.ttf                                                                                                                                       |
| `playground/guided-steps/epic-15/guided-steps-f14-options.ts`         | Arial.ttf, for the space hung at the end of a right-aligned line                                                                                                |
