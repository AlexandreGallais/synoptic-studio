---
id: US-061
epic: E17
feature: F18
title: Font list held while the default font is read again
status: done
points: 2
---

# E17 · F18 · US-061 — Font list held while the default font is read again

As a symbol designer, I want the font list held while the default font is read again, and a face refused at that reading no longer measured, so that I never choose at the wrong moment nor measure a face the browser no longer gives (left by US-060: I77, I78; F18.AC14).

## Acceptance criteria

- Given the default font's files read again, when the reading is not finished, then the font list is disabled and says it is reading.
- Given a face refused at the second reading of its family, when a text uses that family, then the face's metrics read ahead are dropped and the text is measured as for a face not held, or the refusal is said.

**Default font read again** (US-060): when the browser does not draw Inter, the first installed family read is chosen once the reading ends and its files are read a second time; that is the reading this story holds the list for. When the browser draws Inter, nothing is read again.

**Second criterion, the behaviour chosen** (the agent's reading, from `docs/domain/shapes.md`, Text): the face's metrics read ahead are **dropped**, and the refusal stays said as before (`Test Sans: 1 faces read, 1 refused (…)`) when the family is chosen by hand. The domain already leans that way: every font offered can be exported with Embed fonts, a font chosen never ending as `its file not held` (US-060, settled), and a family chosen keeps only its faces that are listed (US-056); a refused face kept with its metrics read ahead broke both, being measured and then left out of the export. Saying the refusal alone would have kept that face measured. A family none of whose faces is read again keeps its metrics read ahead, the list staying on it (US-059, unchanged); Inter's shipped faces the browser draws are kept.

## Product Owner test

Chrome, on a computer where Inter is **not** installed. Run `npm run dev`, open `http://localhost:5173/`.

| Step | Do                                                                      | You should see                                                                                                                                            |
| ---- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | choose the Text tool, click **Installed fonts**, allow                  | the font list and Installed fonts are greyed while the counting line runs (`Reading the installed fonts: 12 of 312 families…`)                            |
| 2    | watch the moment the counting line turns into `Installed fonts read: …` | the list stays greyed, the line under the buttons says `Reading Helvetica, the default font: the font list can be used once it is read.` (a short moment) |
| 3    | wait for the end                                                        | the list is enabled, Helvetica chosen, the line says `Helvetica is chosen, the first font read: Inter is not installed on this computer.`                 |
| 4    | open the weight list, tick **Embed fonts**                              | only weights the browser could read again; no line `…: not embedded, its file not held.`                                                                  |

Look closely at step 2: before this story, the list became usable for a moment there, between the two readings. The second criterion cannot be reproduced by hand (the browser must give a face once, then refuse it): the tests prove it.

## Tasks

One task = one commit, referenced as `US-061.Tn`.

- [x] T1 — The list disabled during the second reading; tests (1 h)
- [x] T2 — A face refused at the second reading dropped; tests and playground (1 h)

## For the main thread

- Cause of I77: the font list was enabled from the session's installed fonts left to read; once the last family was read ahead that list was empty, while the default font's files were still being read again. The session now holds whether the installed fonts are being read (`FontSession.isReading`), from the listing until the default font is held, cleared on a failure too.
- Cause of I78: `readInstalledFamily` kept a refused face's metrics read ahead; they are now dropped when another face of the family is read again (`readFontsWithoutFacesReadAhead`). The F17.AC10 test that asserted the old behaviour (700 kept after its join was refused) now asserts it dropped.
- Domain: `docs/domain/shapes.md`, Text, Installed fonts: the list held during the second reading, the message, and the refused face dropped.
- Checklist line proposed for E17 (`?checklist=E17`), US-061: "Inter not installed: Text tool, Installed fonts, allow; when the counting line ends, the font list stays greyed with `Reading …, the default font…` until the default font is chosen; with Embed fonts on, no face listed `its file not held`."
