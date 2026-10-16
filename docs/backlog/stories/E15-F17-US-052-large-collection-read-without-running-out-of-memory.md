---
id: US-052
epic: E15
feature: F17
title: Read a large font collection without running out of memory
status: ready
points: 3
---

# E15 · F17 · US-052 — Read a large font collection without running out of memory

As a symbol designer, I want to choose any installed font, Apple Color Emoji among them, so that the page never crashes when a font is read.

From the Product Owner's first pass of VAL-018 (2026-10-16) and AUD-019's finding A2: choosing Apple Color Emoji, installed or imported, crashed the page. Cause, reproduced in Node.js 24 with the repository's own parsers:

- `collectionFaceToFile` wrote a face of a collection as a plain array of numbers, spread table by table, before `Uint8Array.from`. Apple Color Emoji.ttc (192 120 460 bytes) holds 2 faces, each with an `sbix` table of 191 134 508 bytes, past V8's longest array (about 134 million elements): "invalid array length", the heap exhausted, a crash of the process, not an error a `try` catches.
- `parseCollectionFaces` was called outside the `try` of the installed face's read, and it wrote **every** face of a collection before the face of the PostScript name was chosen: 2 × 191 MB for one face.

**Decision** (the Product Owner left the choice to the agent, VAL-018; the agent chose the analyst's option b): the library gives a face of a collection as its **parts**, never copied: its table directory written anew (a few hundred bytes), each table a view of the collection's bytes (`subarray`), and the zeros that pad it to four bytes; the playground joins them with the browser (`new Blob(parts)`), only for the face kept. The face chosen by its PostScript name is found by reading name ID 6 straight from the collection, at the face's directory, no face assembled. The face kept is read from its own joined file, since the page gives that file to `FontFace` and embeds it at export (EN-039). Every error of a read is said in one line, never left uncaught (`shapes.md`, Installed fonts).

## Acceptance criteria (F17.AC10)

- Given a collection whose face holds a table of 191 134 508 bytes (Apple Color Emoji.ttc on macOS), when that face is read, then its parts are given without any array of numbers: the table is a view of the collection's bytes, its offset the table's own, its length 191 134 508.
- Given Helvetica.ttc, when each of its 6 faces is joined from its parts, then its bytes are those `collectionFaceToFile` wrote before (the same length and SHA-256 for each face: 416 420, 437 820, 660 564, 355 044, 319 036 and 325 764 bytes).
- Given a collection given whole by the browser for one face, when that face is read, then its PostScript name is read at each face's directory in the collection and only that face's parts are given: `Helvetica-Bold` gives the second face; a name the collection does not hold gives none.
- Given an installed or imported face whose read throws (a join refused by the browser, a file too large), when its family is chosen or its file imported, then one line names the face and gives the error, the other faces kept, and no error is left uncaught.

## Product Owner test

| Step | Do                                                                    | You should see                                                                    |
| ---- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| 1    | List the installed fonts, choose Apple Color Emoji                    | no crash: "Apple Color Emoji: … read.", or one line saying why it was refused     |
| 2    | Import `/System/Library/Fonts/Apple Color Emoji.ttc` with Import font | no crash: its faces imported, or one line naming the file and why it was refused  |
| 3    | Choose Helvetica in the installed fonts                               | "Helvetica: 6 faces read.", a text in Helvetica 700 drawn and measured as before  |
| 4    | Import `/System/Library/Fonts/Helvetica.ttc`                          | "Helvetica 400, Helvetica 700, … imported.", 6 faces, the text drawn as in step 3 |

## Tasks

One task = one commit, referenced as `US-052.Tn`.

- [ ] T1 — The library gives a face of a collection as its parts, and finds a face by its PostScript name read in the collection, with tests on Helvetica.ttc's bytes and Apple Color Emoji.ttc's `sbix` (2 h)
- [ ] T2 — The playground joins a face's parts with a `Blob`, only for the face kept, every read error said in one line; `shapes.md` and the checklist line (2 h)
