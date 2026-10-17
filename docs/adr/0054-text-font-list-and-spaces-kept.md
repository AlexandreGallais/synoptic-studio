# ADR-0054 — A text's font list and its spaces kept

**Status**: Accepted (agent, US-031 review, 2026-10-15). Amends ADR-0052, decision 4 (what the `<text>` carries); the rest of ADR-0052 stands. Number 0053 stays reserved for F15's HTML layer (ADR-0052, decision 8).

## Context

ADR-0052 lists what a text's `<text>` carries: its font family, "the font, then its fallback", its size, weight, style, color and spacing. Building the Text tool (US-031) showed two gaps:

- **The font list.** The refinement names a font with a fallback, `Inter` else `sans-serif`; a fallback typed by the designer may itself be a font a reader lacks. CSS draws a text with the first family of the list it has; a generic family is "guaranteed to match on all systems" (`REF-CSS-FONTS-4`, §2.1.2). A family name other than a generic one is written "quoted as `<string>`s" or as identifiers; quoting is recommended for names "that contain white space, digits, or punctuation" (§2.1.1).
- **The spaces.** The library measures every space of a line, runs of spaces and spaces at its ends included (`DERIV-text-width`, step 3). A reader keeps them only if told: SVG 2 controls white space with the `white-space` property, "New content should not use 'xml:space'" (`REF-SVG2-TEXT`, §11.10.3.1 and §11.10.3.2), and `white-space` is a presentation attribute (`REF-SVG2-STYLING`, §6.6); `pre` "prevents user agents from collapsing sequences of white space" (`REF-CSS-TEXT-3`, §3). SVG 1.1 readers know only `xml:space="preserve"`, with which a reader "will draw all space characters, including leading, trailing and multiple contiguous space characters" (`REF-SVG2-TEXT`, §11.10.3.2); resvg and Batik are among them (`REF-RESVG-SUPPORT-TABLE`). When both are set, "the value of 'xml:space' is ignored" (§11.10.3.3).

## Decision

1. **Font list.** A text's `font-family` is its font, then its fallback, then `sans-serif`, each family once (names compared without regard to case nor to white space at their ends); a generic family is written as its keyword, any other name in double quotes, a double quote or a backslash in it escaped by a backslash and a control character by its hexadecimal code (`REF-CSS-VALUES-4`, §4.4; `REF-CSS2-SYNDATA`, §4.1.3). `Inter` with the fallback `sans-serif` gives `"Inter", sans-serif`.
2. **Spaces kept.** The `<text>` carries both `white-space="pre"`, the property SVG 2 readers use, and `xml:space="preserve"`, the attribute SVG 1.1 readers use, so that every reader draws the spaces the library measured; a reader that knows both follows `white-space`.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Both `white-space` and `xml:space` | every reader keeps the spaces; SVG 2 readers ignore `xml:space` | two attributes for one rule |
| `xml:space` alone | read by SVG 1.1 and SVG 2 readers today | deprecated by SVG 2: "New content should not use 'xml:space'" |
| `white-space` alone | SVG 2's property | SVG 1.1 readers (resvg, Batik) collapse the spaces and draw a narrower line |
| `sans-serif` always last in the font list | a reader lacking both families still draws a sans-serif text | none: a fallback that is already `sans-serif` is not written twice |

## Consequences

- ADR-0052's status names this amendment; its text is unchanged.
- A tab typed in a line is drawn as a space by an SVG 1.1 reader (`xml:space="preserve"` turns tabs into spaces) and as a tab by an SVG 2 reader, while the library measures the font's tab glyph; the domain does not forbid a tab in a line, so this is noted in US-031.

## References

`REF-CSS-FONTS-4`, `REF-CSS-VALUES-4`, `REF-CSS2-SYNDATA`, `REF-SVG2-TEXT`, `REF-SVG2-STYLING`, `REF-CSS-TEXT-3`, `REF-RESVG-SUPPORT-TABLE`, ADR-0052, `DERIV-text-width`
