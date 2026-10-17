---
id: US-062
epic: E17
feature: F18
title: Installed fonts read whole before they are listed
status: done
points: 3
---

# E17 · F18 · US-062 — Installed fonts read whole before they are listed

As a symbol designer, I want an installed font listed only once each of its files has been read whole, so that the list never offers a font that cannot be drawn or embedded.

Decided by the Product Owner at VAL-019 (2026-10-17): the list holds only fonts known to be sound. Until now the first reading of the installed fonts kept each face's metrics, and a face could still be refused when its family was chosen and its files read again (US-053, US-060, US-061).

## Acceptance criteria

- Given the installed fonts being read, when a face's file cannot be read whole, then that face is not listed, and a family left with no face is not listed, the counting line saying how many were refused.
- Given a face's file read whole at the first reading, when it has been checked, then its bytes are released at once and only its metrics are kept, so that the memory held stays that of one file at a time (a 191 MB file is read, checked and released).
- Given a family chosen from the list, when its files are read again, then the reading still happens and a refusal at that moment, possible only for a font removed meanwhile, is handled as before (US-061) and said.
- Given the first reading timed on this computer before and after, when the story ends, then both durations are written in the story and the domain.

**What changed** (2026-10-17): the first reading already read each face's file whole and checked it; it now releases each face's bytes as soon as it is checked, and no longer remembers a face it refused, which was read again when its family was chosen and could then be listed. The faces refused in the families read are said by the counting line, after the families left out: `Installed fonts read: 2 families. 1 faces of the families read refused: 1 unreadable (Helvetica Light).` (the agent's wording, in VAL-019's decisions to confirm).

**Durations, before and after** (this Mac, 2026-10-17): the first reading of its 837 faces of 377 families, emulated in the page's tests (happy-dom) from the font files of `/System/Library/Fonts` and `~/Library/Fonts`, each face given as Chrome on macOS gives it, its own file: **77.4 s before, 76.1 s after**; the memory still held when the next file is read (array buffers, after a garbage collection): **889 MB before, 19 MB after**. The line said: `Installed fonts read: 153 families; 224 left out: …. 3 faces of the families read refused: 1 not embeddable (STSongti-SC-Black); 2 no Latin letters (HoeflerText-Ornaments, IowanOldStyle-Titling).` Chrome's own duration, `blob()` included, needs the browser: the Product Owner times it with the test card below.

## Product Owner test

Chrome, `npm run dev`, `http://localhost:5173/`, a stopwatch.

| Step | Do                                                         | You should see                                                                                         |
| ---- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| 1    | Text tool, **Installed fonts**, allow; start the stopwatch | `Reading the installed fonts: 12 of … families…`                                                       |
| 2    | stop it when the line turns to `Installed fonts read: …`   | the line ends with `3 faces of the families read refused: …` (on this Mac); write the time in the chat |
| 3    | choose Hoefler Text                                        | `Hoefler Text: … faces read.`, without `refused`                                                       |
| 4    | tick **Embed fonts**                                       | no line `…: not embedded, its file not held.`                                                          |

## Tasks

One task = one commit, referenced as `US-062.Tn`.

- [x] T1 — Each face read whole and checked at the first reading, its bytes released; tests (2 h)
- [x] T2 — The counting line and the domain; durations measured; Product Owner test card (1 h)

## For the main thread

- Rule-docs: `docs/domain/shapes.md` (Installed fonts, Read whole before listed), the Q39 row of `docs/domain/README.md`, VAL-019's decisions to confirm and the guide page `docs/guide/f18-safe-and-cheap-parallel-run.md` are in T2.
- Checklist line proposed for E17 (`?checklist=E17`), US-062: "Installed fonts, allow, wait: the counting line ends with the faces refused, if any; a family chosen says no `refused`; with Embed fonts on, no face listed `its file not held`."
- The bench is not committed: a temporary test reading this Mac's font files, kept at `/private/tmp/claude-501/-Users-alex-Documents-github-SVG-editor/48a404b1-d4ec-444c-a6e8-7780daa5a399/scratchpad/us-062-worker/bench.ts`.
