---
name: spike
description: Run a research spike (SP story) — an epic's refinement from `develop`, or the short spike that opens a feature with questions left — sourcing every story before any code. Use when starting an epic or a feature with open questions, or for a story of type SP.
argument-hint: "[spike id, e.g. SP-001]"
---

# Research spike

Spike: $ARGUMENTS. Protocol: the research protocol of the records (`knowledge search research protocol`): levels of sources, stop rule, recording.

Two kinds:

- **An epic's refinement**, when a new epic starts: an `SP` story from `develop`, merged into `develop` by its auto-merge (`npm run story:finish -- develop …`). It records the Product Owner's account of the epic, the documentary research on it (steps 2 to 5), and cuts its features, each of a few `US` or `EN` stories of at most 3 tasks, written as rows of the records. A larger story is not written: the agent proposes its split to the Product Owner, with an example.
- **A feature's spike**, only when questions remain after the refinement: a short `SP`, the first story of its feature branch.

1. **Inventory**: for each story, every formula, algorithm, specification point and interface behavior it needs.
2. **Internal first**: the records (`knowledge search`, `--kind ref`, `--kind note`), the rules, then the local library (`library_search`). Reuse before searching. Then, one at a time, in the background, and only when the feature holds them: the `functional-analyst` on its business rules and open questions, the `math-expert` on its formulas.
3. **Search and read**: spec > paper > book > official docs > code (read only, never copied) > blog (never alone). Read the source itself, through `library_get` so that it is kept; a search result summary is not a reading. A paywalled or unreadable source is not cited: derive from readable sources, or write a research request.
4. **Record before use**: a `REF-*` in the records (what it says that we use, `[verified] YYYY-MM-DD`, with qualifiers if partial); a derivation when no single source states the result (`/derivation`).
5. **Write the note** into the records (`knowledge add note <file>`): per story, the sources and what remains open.
6. **Update the stories** in the records: acceptance criteria from the sources' examples, `@see` targets in the tasks, blocked points; business questions go to the Product Owner, all at once.
7. Branch, commits and pull request as for a story (`/story` steps 3 to 7).
