---
id: EN-138
epic: E17
feature: F31
title: replace and npm run fix do what they say
status: done
points: 2
---

# E17 · F31 · EN-138 — `replace` and `npm run fix` do what they say

Two cards written during F22 ([tool incidents](../../process/tool-incidents.md), REV-006): `target/release/replace` called alone, to see its usage, printed nothing and waited on its standard input until the call's 120 s time-out, by the main thread and by EN-117's worker; `npm run fix` after VAL-023's checklist lines rewrote `export type * from "./checklist-line"` into `export *` in `playground/checklists/index.ts`, a type-only re-export turned into a value one, set back by hand.

## Acceptance criteria

- Given `replace` with no argument and its standard input a terminal, or with `--help`, when it runs, then it prints its usage and exits at once, 0 for `--help`, 2 otherwise; given its blocks on a pipe, then it works as before.
- Given a barrel holding `export type * from "./x"`, when `npm run fix` runs on it, then the line is unchanged; the test that proves it names the tool and the rule that rewrote it, found by T1, and the rule is set so in its configuration or fixed in `tools/lint`.

## Tasks

- [x] T1 — The tests of both cards, the rewriting rule found, failing (1 h)
- [x] T2 — `replace`'s usage, the rule set or fixed, `docs/tooling/agent-tools.md` (1 h)

One task = one commit, referenced as `EN-138.T1` in the commit footer.
