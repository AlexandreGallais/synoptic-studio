---
id: SP-018
epic: E15
feature: F15
title: "Research for F15: fonts from files, embedded fonts and the HTML layer"
status: in-progress
points: 3
---

# E15 · F15 · SP-018 — Research for F15: fonts from files, embedded fonts and the HTML layer

Find, read and record the sources of every story of F15 before its implementation, then write its stories (ADR-0020, research protocol §8). F05's refinement, round 2, settled the business rules of fonts: a font named with a fallback; TrueType and OpenType files, imported or installed, read once and kept; an option embeds a font in a symbol, and once per synoptic view; no text converted to paths; Arabic and Indic scripts laid out by the browser's HTML layer, Hebrew right to left by the library. What they leave open is decided here, provisional, to confirm at the feature's validation; ADR-0053, reserved since ADR-0052, is written here.

## Acceptance criteria

- Given each story of F15, when the spike ends, then every behavior, format and calculation it needs has a verified `REF-*`, a `DERIV-*` or an open research request.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and research note 0020 records what was found and what remains open.
- Given F15's third criterion, when the spike ends, then ADR-0053 says which SVG elements the HTML layer and the embedded fonts add, and `AGENTS.md` lists them in the same change.
- Given the open points left by F05 (Inter without Hebrew, `hhea` against `OS/2` line metrics, right to left in resvg and Batik, the size of Inter's data, `GPOS` read for the common case only, letter spacing after the last letter and the underline's position in browsers), when the spike ends, then each is settled, given to a story, or recorded as open with its reason.

## Questions

| Question                                                                                                                      | Story          |
| ----------------------------------------------------------------------------------------------------------------------------- | -------------- |
| How does a browser page read a font file the designer chooses (File API, `<input type="file">`)?                              | EN-039, US-036 |
| How does it read a font installed on the computer (Local Font Access API): permission, user gesture, browsers that have it?   | US-036         |
| Which names, weight and style does a font file give for its face (`name`, `OS/2` `usWeightClass` and `fsSelection`)?          | EN-039         |
| What does each `fsType` value and bit permit (installable, restricted, preview and print, editable, no subsetting, bitmap)?   | EN-039         |
| How is a font written into an SVG: `@font-face`, a `data:` URL, base64, its media type and format hint; how long is it?       | EN-040         |
| Which SVG readers draw an `@font-face` with a `data:` URL: browsers, resvg, Batik, Inkscape?                                  | EN-040         |
| Which SVG element holds the style sheet, and where?                                                                           | EN-040         |
| How does `switch` choose its child; which `requiredExtensions` value do browsers accept; what do resvg, Batik, Inkscape show? | EN-041         |
| How do CSS properties lay out the HTML layer as the library lays out its lines (spaces, wrapping, ellipsis, maximum lines)?   | EN-041         |
| Where does a browser put the first baseline (`hhea`, `OS/2` typographic or Windows metrics, the line gap)?                    | EN-041         |
| What must every shape and text of E01, E14, F05, F14 and F16 keep?                                                            | every story    |

## Tasks

One task = one commit, referenced as `SP-018.Tn`.

- [x] T1 — Inventory the behaviors, formats and calculations of the stories (1 h)
- [ ] T2 — Find, read and record the sources; write `DERIV-font-embedding` (3 h)
- [ ] T3 — ADR-0053: the HTML layer and the embedded fonts as SVG elements; `AGENTS.md` (1 h)
- [ ] T4 — Research note 0020, the domain's provisional rules and F15's stories (2 h)
