---
id: EN-057
epic: E17
feature: F18
title: The register of exceptions, and a byte buffer for font collections
status: done
points: 3
---

# E17 · F18 · EN-057 — The register of exceptions, and a byte buffer for font collections

Apply ADR-0066: list every place the code departs from a rule, keep the list in step by a check, and read large font collections through a byte buffer written once (I67; F18.AC13).

## Acceptance criteria

- Given `docs/conventions/exceptions.md`, when it is read, then each row gives the file, the function, the rule and the reason.
- Given an `eslint-disable` without its row, or a row whose code is gone, when the tests run, then a test fails and names it.
- Given Apple Color Emoji.ttc (192 MB), when `collectionFaceToFile` extracts a face, then it succeeds; the function builds one `Uint8Array` of the final size, written once, with its justified `eslint-disable`.

## Tasks

One task = one commit, referenced as `EN-057.Tn`.

- [x] T1 — The register, filled from the code, and its check, mutation-checked (2 h)
- [x] T2 — The byte buffer in `collectionFaceToFile`, its tests and the large collection read (2 h)

## For the main thread

- Name: the third criterion names `collectionFaceToFile`; since US-052 a face of a collection is given as parts, views of the collection (`collectionFaceToParts`), and the playground joined them with a `Blob`. The byte buffer written once is `facePartsToFile`, which joins those parts in `src/`; the playground's `readJoinedFile` is removed, both readers of a face (imported file, installed font) call it. Apple Color Emoji's first face is joined in about 0.2 s, its digest that of the parts read in order; the test is skipped where the file is missing.
- Folder: `src/io/font-tables/` would have held 16 modules; the collection modules moved to `src/io/font-collection/` (`files.md` updated), no rename of a function.
- Register: `docs/conventions/exceptions.md` holds 30 rows; a story merged later that adds an `eslint-disable` fails `tests/exceptions.test.ts` until its row is written. Rust `unsafe` (`tools/heavy-lock`) is not counted: no Rust lint forbids it, and no `#[allow]` exists.
