---
id: AUD-014
epic: E14
feature: F13
title: "Audit F13"
status: done
points: 1
---

# E14 · F13 · AUD-014 — Audit F13

Independent audit of F13 (ADR-0020): each new name checked against its source, the drawings unchanged, the list of calculations left inline.

## Acceptance criteria

- Given F13's criteria, when the audit runs, then each is traced, and the correctness findings fixed.

## Tasks

One task = one commit, referenced as `AUD-014.Tn`.

- [x] T1 — Run the auditor and fix its findings (2 h)

## Findings

Auditor, on F13's own diff (`feat/val-012-validate-f10...`), 2026-10-14. The library before and after F13 was bundled and compared on 70 970 seeded results, signed zeros told apart: no difference. No expected value of a test was edited.

| Check                  | Result                                                                                                        | Action                                                                                                               |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| AC4, playground        | the Add button's edge count used `!isFlat`, the action `hasArea`: they could disagree on an underflowing size | fixed: both `hasArea`                                                                                                |
| AC1, left inline       | three items carried to pass 2 not recorded; `halfSmallerSide` missing from the note                           | recorded in [research note 0016](../../research/notes/0016-f13-named-calculations.md), the list of what stays inline |
| AC2, wording           | the kept pairs (`midpoint`, `pointBetween`; `cubicVelocity`, `derivativeCoefficients`) against "merged"       | criterion aligned; both TSDoc say why                                                                                |
| E, TSDoc               | `pointBetween` and `midpoint` claimed more exactness than holds                                               | fixed                                                                                                                |
| E, Q35                 | the page never shows the 180° of a zero handle (`readKeptAngle`)                                              | Q35 settled: nothing to decide                                                                                       |
| E, folders and sources | `files.md` rows; the zero vector's quote in `REF-OPENSTAX-CALC3-VECTORS`                                      | fixed                                                                                                                |
| D, test helpers        | `isAddGreyed`, `isGreyed` remain beside `readDisabledState`                                                   | recorded: they read the state with its message                                                                       |
| H, evolvability        | the golden file is a scratch tool                                                                             | already in the improvement log, to become a script                                                                   |
