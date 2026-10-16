# ADR-0060 — No HTML layer; fonts embedded only when a file is written

**Status**: Accepted (Product Owner, 2026-10-15, at the validation of E15, VAL-017). Supersedes ADR-0053, decision 1 (the HTML layer) and decision 3 for `switch` and `foreignObject`; amends its decision 2 (embedded fonts) on when the fonts are written; its decision 3 for `style` and its decision 4 stand. Amends ADR-0052, decisions 6 and 8.

## Context

- **ADR-0053** added an HTML layer to each text: a `switch` holding a `foreignObject` laid out by the browser, then the positioned lines for every other reader. Its purpose was to let a browser shape the scripts the library does not measure, Arabic and Indic scripts among them (ADR-0052, research note 0020).
- **No speed to gain.** The library already computes a text's lines, about 3 µs per text: the layer does not make a view faster, and it writes each text twice.
- **No script to serve.** Industrial synoptic views are written in Latin script. The editor supports Latin script explicitly; Greek, Cyrillic and Hebrew, measured since F05, and right to left stay as they are, and no other script is added. A research note confirming that industrial HMIs mostly use Latin script is a task of F17's spike (SP-019).
- **Fewer options make a clearer product**: the same reason removed the balanced wrap and the choice for a word too long at the same validation (`shapes.md`, Text, Box).
- **Embedded fonts** made the drawing heavier while it was edited: the whole file of every face used was written in the drawing's `<defs>` at each redraw.

## Decision

1. **No HTML layer.** A text is written as ADR-0052, ADR-0054 and ADR-0055 say, and only so: no `switch`, no `foreignObject`, no XHTML. The option, its playground toggle, its guided steps and its code are removed (F17).
2. **Allowed SVG elements**: `svg`, `g`, `path`, `text`, `tspan`, `defs`, and `style` only inside `defs`, holding only `@font-face` rules (ADR-0053, decision 3, kept for `style` only).
3. **Embedded fonts are written only when a file is exported or saved**, never during editing: the drawing on screen holds no `<style>`; **Export SVG** writes the `@font-face` rules of ADR-0053, decision 2, when the option is on. The rules themselves (each face once, its `fsType`, a family whole or not at all) do not change.
4. **Scripts**: Latin script is the editor's target; Greek, Cyrillic, Hebrew and right to left keep what F05 built; Arabic and Indic scripts are not supported, and nothing lays them out.

## Reasons

| Option                                    | For                                                                     | Against                                                                 |
| ----------------------------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| No HTML layer (chosen)                    | one copy of each text; one way to draw it; fewer options                | no Arabic nor Indic script                                              |
| The HTML layer kept (ADR-0053)            | browsers shape every script                                             | two copies of each text; no speed gained; scripts industrial views lack |
| Fonts embedded at export or save (chosen) | a light drawing while editing; the file still stands alone once written | the drawing on screen differs from the file by its `<style>`            |
| Fonts embedded in the drawing (ADR-0053)  | the drawing on screen is the file                                       | the whole file of every face rewritten at each redraw                   |

## Consequences

- `AGENTS.md`'s Never line no longer allows `switch` nor `foreignObject`.
- `shapes.md` (Text) and Q39 state the rules, the sections on the HTML layer replaced by its removal; US-044 removes its code, its playground option and its guided steps. `DERIV-html-layer` and research note 0020 stay as a record.
- F15's criterion 3 (the HTML layer) is withdrawn by US-044; F15's criterion 2 is met at export (US-045).
- Q24 (a `foreignObject` tool) stays open, with no `foreignObject` allowed today.

## References

ADR-0052, ADR-0053, ADR-0054, ADR-0055, `DERIV-html-layer`, `DERIV-font-embedding`, [research note 0020](../research/notes/0020-f15-fonts-and-html-layer.md)
