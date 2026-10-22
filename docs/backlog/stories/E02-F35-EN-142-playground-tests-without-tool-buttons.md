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

- [ ] T1 — The helper, and the shape tests moved to it (2 h)
- [ ] T2 — The text and font tests moved to it; the tool tests and checklist lines listed (2 h)
