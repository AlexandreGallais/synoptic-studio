---
name: spike
description: Run the research spike (SP story) that opens a feature, sourcing every story before any code. Use when starting a feature or a story of type SP.
argument-hint: "[spike id, e.g. SP-001]"
---

# Research spike

Spike: $ARGUMENTS. Protocol: `docs/research/README.md` §1–§8 (levels of sources, stop rule, recording).

1. **Inventory**: for each story of the feature, list every formula, algorithm, specification point and interface behavior it needs (the spike's Questions section).
2. **Internal first**: `docs/references.md`, `docs/derivations/`, `docs/research/notes/`, the domain files, then the local library (`library_search`). Reuse before searching. In parallel, in the background: the `functional-analyst` on the feature's business rules and open questions, the `math-expert` on its formulas.
3. **Search and read**: spec > paper > book > official docs > code (read only, never copied) > blog (never alone). Read the source itself, through `library_get` so that it is kept; a search result summary is not a reading.
   - PDF without text tools: decompress its streams in Python (`zlib`) and read the text operators, or find an HTML edition.
   - Paywalled or unreadable source: do not cite it; derive from readable sources, or write a research request (§6).
4. **Record before use**: a row in `docs/references.md` (`REF-*`, what it says that we use, `[verified] YYYY-MM-DD` with qualifiers if partial). A derivation when no single source states the result (`/derivation`).
5. **Write the note** `docs/research/notes/NNNN-<feature>.md`: per story, the sources and what remains open; add it to the notes index.
6. **Update the stories**: acceptance criteria from the sources' examples, `@see` targets in the tasks, blocked points; business questions go to the Product Owner (open questions of `docs/domain/README.md`).
7. Branch, commits and pull request as for a story (`/story` steps 3 to 9), scope `docs`.
