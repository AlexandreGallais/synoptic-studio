---
name: spike
description: Run a research spike (SP story) — an epic's refinement from `develop`, or the short spike that opens a feature with questions left — sourcing every story before any code. Use when starting an epic or a feature with open questions, or for a story of type SP.
argument-hint: "[spike id, e.g. SP-001]"
---

# Research spike

Spike: $ARGUMENTS. Protocol: `docs/research/README.md` §1–§8 (levels of sources, stop rule, recording).

Two kinds (ADR-0086):

- **An epic's refinement**, when a new epic starts: an `SP` story from `develop`, without a feature (`docs/backlog/_spike.md`), merged into `develop` by a merge commit, its own bubble, **by the Product Owner**: `feature-automerge.yml` lets into `develop` only a feature branch or a `REV`, so `npm run story:finish -- develop …` opens its pull request without the `autonomous` label and says at once that it waits for the Product Owner's merge, never waiting for GitHub's; run again once merged, it cleans up (AUD-024, EN-118). It records the Product Owner's account of what the epic describes, the documentary research on it (steps 2 to 5), and cuts its features, each of 2 to 4 `US` or `EN` stories, each story of at most 3 tasks. A feature or a story larger than that is not written: the agent proposes its split to the Product Owner, with an example (ADR-0084).
- **A feature's spike**, only when questions remain after the refinement: a short `SP`, the first story of its feature branch, its own small bubble. The steps below, for that feature's stories.

1. **Inventory**: for each story of the feature, list every formula, algorithm, specification point and interface behavior it needs (the spike's Questions section).
2. **Internal first**: `docs/references.md`, `docs/derivations/`, `docs/research/notes/`, the domain files, then the local library (`library_search`). Reuse before searching. Then, one at a time, in the background, and only when the feature holds them (ADR-0086, decision 1): the `functional-analyst` on its business rules and open questions, the `math-expert` on its formulas.
3. **Search and read**: spec > paper > book > official docs > code (read only, never copied) > blog (never alone). Read the source itself, through `library_get` so that it is kept; a search result summary is not a reading.
   - PDF without text tools: decompress its streams in Python (`zlib`) and read the text operators, or find an HTML edition.
   - Paywalled or unreadable source: do not cite it; derive from readable sources, or write a research request (§6).
4. **Record before use**: a row in `docs/references.md` (`REF-*`, what it says that we use, `[verified] YYYY-MM-DD` with qualifiers if partial). A derivation when no single source states the result (`/derivation`).
5. **Write the note** `docs/research/notes/NNNN-<feature>.md`: per story, the sources and what remains open; add it to the notes index.
6. **Update the stories**: acceptance criteria from the sources' examples, `@see` targets in the tasks, blocked points; business questions go to the Product Owner (open questions of `docs/domain/README.md`).
7. Branch, commits and pull request as for a story (`/story` steps 3 to 9), scope `docs`.
