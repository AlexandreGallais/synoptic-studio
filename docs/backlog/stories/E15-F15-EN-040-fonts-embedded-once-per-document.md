---
id: EN-040
epic: E15
feature: F15
title: Fonts embedded once per document as @font-face rules
status: done
points: 3
---

# E15 · F15 · EN-040 — Fonts embedded once per document as @font-face rules

The library writes the faces a document's texts are measured with, each once, as `@font-face` rules whose source is the whole file in a base64 `data:` URL, in one `<style>` of the document's `<defs>`; a face that may not be embedded, or whose file is not held, is left out and listed (ADR-0053, `DERIV-font-embedding`, steps 4 to 7).

## Acceptance criteria

- Given the bytes of `"f"`, `"fo"`, `"foo"` and `"foobar"`, when encoded in base 64, then they give `Zg==`, `Zm8=`, `Zm9v` and `Zm9vYmFy`; given an empty file, an empty text (`REF-RFC-4648`, §10); given any bytes, then the text is `4 · ⌈n / 3⌉` characters long, and decoding it by the alphabet gives the bytes back (a property test).
- Given a face of family `Arial`, weight 700, italic, from a TrueType file, when written, then its rule is `@font-face { font-family: "Arial"; font-style: italic; font-weight: 700; src: url(data:font/ttf;base64,…) format("truetype"); }`; from a CFF file (`OTTO`), `font/otf` and `format("opentype")`; no `local()` (`REF-CSS-FONTS-4`, §4.3, §11.2; `REF-BATIK-1159`).
- Given a family named `My "Font"`, when written, then its name is escaped as the `font-family` of the text is (ADR-0054, `formatFontFamilyName`).
- Given a document of 10 texts in Arial 400 and 700, the embedding on, when written, then its `<defs>` holds one `<style>` with 2 rules, in the order of the faces' first use (`DERIV-font-embedding`, step 7).
- Given a text in a font whose `fsType` is 2, or in Inter whose file is not held, when the document is written with the embedding on, then no rule is written for it and the faces left out are given with their reason (not permitted, no file).
- Given the embedding off, or no face to embed, when written, then the document has no `<style>`, and every shape and text of E01, E14, F05, F14 and F16 is written as before.

## Tasks

One task = one commit, referenced as `EN-040.Tn`.

- [x] T1 — Base 64 of a file's bytes, by groups of three bytes into four characters, padded with `=`, `@see REF-RFC-4648`, `DERIV-font-embedding` steps 4 to 6; tests first: the vectors of RFC 4648 §10, then the property test (2 h)
- [x] T2 — The `@font-face` rule of a face: family, style, weight, `src` as a `data:` URL with its media type and format hint, `@see REF-CSS-FONTS-4`, `REF-RFC-2397`; tests first (2 h)
- [x] T3 — The faces a document embeds: the face each text is measured with, each once, permitted and held, the others listed with their reason; the `<style>` in `<defs>` of the render (`style` allowed there by ADR-0053), the option off by default; `shapes.md` (Embedded fonts) updated; tests first (2 h)
