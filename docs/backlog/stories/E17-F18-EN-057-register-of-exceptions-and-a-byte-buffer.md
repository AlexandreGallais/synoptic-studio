---
id: EN-057
epic: E17
feature: F18
title: The register of exceptions, and a byte buffer for font collections
status: ready
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

- [ ] T1 — The register, filled from the code, and its check, mutation-checked (2 h)
- [ ] T2 — The byte buffer in `collectionFaceToFile`, its tests and the large collection read (2 h)
