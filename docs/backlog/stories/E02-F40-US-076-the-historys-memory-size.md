---
id: US-076
epic: E02
feature: F40
title: The history kept within a memory size the designer sets
status: done
points: 2
---

# E02 · F40 · US-076 — The history kept within a memory size the designer sets

As a symbol designer, I want to set how much memory the history keeps, and how many steps it keeps whatever their size, so that a long session never slows the editor while the last steps stay undoable.

After US-075. The history is bounded by a memory size and by a minimum of steps kept whatever their size, both set by the designer (Q7, SP-029's answers 6 and 7, 2026-10-23): 100 MB and 5 steps by default (`DEFAULT_HISTORY_BOUND`, EN-146), two fields beside the Undo and Redo buttons, kept in the browser for the next session, as the checklists keep their ticks (answer 6 (i)); the size shown with about how many steps it holds, explained for the designer (answer 6). Its size is counted from its data (ADR-0095, decision 4), the oldest steps dropped first, never below the steps kept, as GIMP drops them (`boundedHistory`, EN-146; [note 0030](../../research/notes/0030-f40-undo-and-redo.md), §3, §5). What the fields accept, and how the steps held are counted, are the agent's choices, for the review's summary, F40 having no validation of its own.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, 614 characters as the page keeps it, 1 228 bytes as the history counts them, the only shape of the drawing and selected. A move of A alone adds 1 244 bytes to the history: A as it was, 1 228, its reference, 8, and the index of its selection, 8.

- Given the page opened for the first time, then beside the Undo and Redo buttons the field Memory shows 100, in MB, and the field Steps kept 5; the toolbar shows `History 0 B of 100 MB` and `about 80,380 moves`, a move of one shape adding about 1,244 bytes, as the fields' titles explain: 100 000 000 / 1 244 = 80 385.9, rounded down to tens (agent's choice).
- Given 20 typed in Memory and 3 in Steps kept, each applied by Enter or by leaving its field, when the page is opened again in the same browser, then the fields show 20 and 3, the toolbar `of 20 MB` and `about 16,070 moves` (20 000 000 / 1 244 = 16 077.2), the history bounded by them; without the browser's storage, or with a value it did not write, 100 and 5 (SP-029's answer 6 (i)).
- Given a memory size and 0 steps kept, when A is moved again and again, then each move adds 1 244 bytes and the oldest are dropped first, the fewest that bring the history within the size: 12 440 bytes keep 10 moves, the eleventh dropping the first; 10 MB keeps 8 038 moves (10 000 000 / 1 244 = 8 038.6), shown as `about 8,030 moves`; in a drawing of 100 rectangles, a move of A adds 2 036 bytes, its 100 references 800, so that 10 MB keeps 4 911.
- Given steps kept, then the history never holds fewer, whatever their size: 0 MB and 2 steps kept, the page's rectangle nudged 4 times, the history holds 2 steps, 2 × 1 248 = 2 496 bytes, `History 2.5 kB of 0 B`, and a third Ctrl + Z does nothing; at 1 MB and 5 steps, 1 000 rectangles selected and `+50` typed in W, a step of about 1 228 000 + 8 000 + 8 000 = 1 244 000 bytes, the rectangles as they were and their references and indices, larger than the whole size, is kept, with the 4 steps before it (SP-029's answer 7).
- Given a history over a bound just set, a memory size smaller or fewer steps kept, then its oldest steps are dropped at once until it fits, the steps to redo counted but never dropped: the page's rectangle nudged 4 times, 2 undone, then 0 MB and 1 step kept set: 1 step left to undo, the 2 to redo kept, the size shown at once, `History 3.7 kB of 0 B` (3 × 1 248 = 3 744).
- Given a value that is not a whole number, or out of 0 to 1 000 MB, or of 0 to 100 steps kept, applied in its field, then the field shows again the value in force, the bound and what the browser keeps unchanged (agent's choice).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Ctrl stands for Cmd on a Mac. A **step** is one finished change of the drawing, which Undo takes back; **the history** is the steps kept for Undo and Redo.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page in a private window | beside Undo and Redo, Memory 100 MB and Steps kept 5, then `History 0 B of 100 MB` and `about 80,380 moves` |
| 2 | Hover over Memory, Steps kept and the moves | a title for each: what it bounds, and a move of one shape adding about 1,244 bytes |
| 3 | Click the rectangle, press Right four times, releasing it each time, then click Undo twice | `History 5 kB of 100 MB` after the four nudges, 4 × 1 248 bytes; still 5 kB after the two Undo, the steps to redo counted |
| 4 | Type 0 in Memory and press Enter | `History 5 kB of 0 B` and `about 0 moves`: nothing dropped, the 4 steps within the 5 kept |
| 5 | Type 1 in Steps kept and press Enter, then click Undo twice | `History 3.7 kB of 0 B` at once; the first Undo goes back one nudge, then Undo is greyed out, and Redo brings back three |
| 6 | Reload the page, click the rectangle, press Right three times, then Ctrl + Z twice outside the fields | Memory 0 and Steps kept 1 as set; one nudge undone, the second Ctrl + Z doing nothing |
| 7 | Type 2.5 in Memory and press Tab, then 1001 and Tab, then 101 in Steps kept and Tab | each field back to 0 and 1, nothing else changed |
| 8 | Type 100 in Memory and 5 in Steps kept, each with Enter, then reload the page | 100 and 5 again, `about 80,380 moves` |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers 6 and 7; the Memory and Steps kept fields, the steps held shown, what the browser keeps: interface and failing tests (1 h)
- [ ] T2 — The bound set, kept in the browser, the history bounded by it at each step and when it changes, the steps held shown; the rules in `interaction.md` §7 and the Q table; the Product Owner test card and F40's guide page; the tests of T1 passing (2 h)

Files: `playground/history/` (US-074's), `playground/index.html` (the fields), `src/model/history/` (the steps held). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-GIMP-PREFS-RESOURCES`, `REF-GIMP-UNDO-CODE`.

One task = one commit, referenced as `US-076.T1` in the commit footer.
