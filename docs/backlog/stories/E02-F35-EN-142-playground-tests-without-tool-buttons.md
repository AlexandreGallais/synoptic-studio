---
id: EN-142
epic: E02
feature: F35
title: The playground's tests set their shape without the tool buttons
status: ready
points: 3
---

# E02 · F35 · EN-142 — The playground's tests set their shape without the tool buttons

Since E01 a tool button turns the edited shape into its kind and keeps its values (120 × 80 at (0, 0)), and 31 of the playground's test files rely on it (US-063's attempt, 2026-10-22: 234 tests failed once the tools placed shapes as Figma does). This enabler makes them independent of the tool buttons first, so that US-063 changes what the buttons do without breaking them (option A, the Product Owner, 2026-10-22).

## Acceptance criteria

- Given each playground test that pressed a tool button only to choose the edited shape's kind, when the story ends, then it sets that kind by one named test helper, and every playground test passes as before.
- Given the tests of the tool buttons themselves, when the story ends, then they are listed in the story, to be rewritten by US-063 with the new behavior.
- Given the lines of the checklists of E01 to E15 that choose a tool to change the edited shape, when the story ends, then their rewording for Figma's way is written under `## For the main thread`, which writes them with US-063.

## Tasks

One task = one commit, referenced as `EN-142.Tn`.

- [x] T1 — The helper, and the shape tests moved to it (2 h)
- [x] T2 — The text and font tests moved to it; the tool tests and checklist lines listed (2 h)

## The tool tests, for US-063

`chooseShapeKind` (`playground/user-actions/choose-shape-kind.ts`) sets the edited shape's kind through the page's hidden kind selector `#shape` and its `change` event, as a guided step does: no tool button, no memory of X, Y and angle per tool. The 31 test files that pressed a tool button only to get a kind use it; the button's pressed state they also checked is now the panel's title (`#shape-title`), which US-063 keeps. `turned-text-frames.test.ts` opens the page again before typing its text, so that the text is not placed from the turned rectangle drawn before it.

The tests below check the tool buttons themselves, and still press them; US-063 rewrites them with the new behavior:

- `playground/toolbar/toolbar.test.ts`, `toolbar`: "starts on the rectangle, its button pressed and the polygon's options hidden", "draws a polygon from its button, its options shown";
- `playground/toolbar/toolbar.test.ts`, `toolbar after a free shape`: its `beforeEach` (the Polygon button), "draws a valid rectangle, its width rounded and one radius for every corner" (the Rectangle button after a free shape, `writeWholeShapeFields`), "goes back to the shape the page opens with on Reset, in Shape mode" (the Rectangle button pressed after Reset);
- `playground/text/tool-placement.test.ts`, the whole file: each tool keeps its own X, Y and angle (US-032), the memory US-063 removes;
- `playground/text-placement/turned-text-in-place.test.ts`: `[F05.AC5] keeps the exact position behind the 5 decimals shown, across a tool change`;
- `playground/guided-steps/epic-15/f05-steps.test.ts`: `[F05.AC5] each tool keeps its placement`, with its guided step `F05 · Each tool keeps its placement` (`playground/guided-steps/epic-15/guided-steps-f05-options.ts`).

The guided step `F14 · A turned text's frames (US-050)` (`playground/guided-steps/epic-15/guided-steps-f14-options.ts`) says "then the Rectangle tool, Width 60, Height 40, Angle 30": with US-063, "then the Rectangle tool, a click on the drawing, Width 60, Height 40, X 0, Y 0, Angle 30".

## For the main thread

The lines of the checklists of E01 to E15 that choose a tool to change the edited shape, and their rewording for Figma's way (`interaction.md` §1), to write with US-063. The playground holds the checklists of E15 and E17 only; E01 to E14 have none, their guided steps set the kind without a tool button.

- `playground/checklists/epic-15-text-lines.ts`, F05: todo "Choose the Text tool, Inter 4.1 installed" becomes "Inter 4.1 installed: choose the Text tool, click the drawing, type “Pump P-101”, Enter, “Running”"; see unchanged, "“Pump P-101” over “Running” in Inter 32, the dashed frame hugging the letters" (a new text's content is `Text`, so the content is typed).
- `playground/checklists/epic-15-text-lines.ts`, F05: todo "Turn the text, choose the rectangle, then the text again", see "each tool keeps its own X, Y and angle; the rectangle 120 × 80 as before", replaced by todo "Turn the text by 30, then choose the Rectangle tool and click the drawing", see "the text stays as it was, turned; a rectangle 100 × 100 added on top, straight, centred on the click, the tool back to the selector" (US-063 removes the memory per tool).
- `playground/checklists/epic-15-color-and-font-lines.ts`, F17: todo "Inter not installed (US-056, US-059, US-060): open the page, Text tool; …" becomes "… open the page, choose the Text tool and click the drawing; …", the rest unchanged.
- Outside E01 to E15, the same wording: `playground/checklists/epic-17-f18-lines.ts`, F18, "Chrome, Inter not installed: Text tool, Installed fonts, …" becomes "Chrome, Inter not installed: the Text tool, a click on the drawing, Installed fonts, …"; `docs/guide/f18-safe-and-cheap-parallel-run.md` repeats it in its two rows starting "Chrome".
