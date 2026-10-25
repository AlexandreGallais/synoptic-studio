---
name: spike
description: Run a research spike (SP story) — an epic's refinement from `develop`, or a short spike opening a feature with questions left — sourcing every story before any code. Use when starting an epic or a feature with open questions, or for a story of type SP.
argument-hint: "[spike id, e.g. SP-001]"
---

# Research spike

Spike: $ARGUMENTS. The protocol: `knowledge show PAGE-research` (the order of the sources, how many, when to stop); what is written goes into the records, not into the repository.

- **An epic's refinement**: an `SP` from `develop`. The Product Owner's account of the epic, the research, then its features of 2 to 4 stories, each of at most 3 tasks, written as rows of the records; every question asked at once, each with options and a numeric example. An epic about the agent's own work is tagged **method and tools**: one audit, at its end.
- **A feature's spike**, only for questions left after the refinement: a short `SP`, the first story of its feature branch.

1. **Inventory**: per story, each formula, algorithm, specification point and behavior it needs.
2. **The records first** (`knowledge search`, `--kind source`, `--kind note`, the rules), then the library (`library_search`). Then, in the background and only when the feature holds them, the `functional-analyst` on its business rules, the `math-expert` on its formulas.
3. **Read the source itself** through `library_get`, so that it is kept: spec > paper > book > official docs > code (read, never copied) > blog (never alone). An unreadable source is not cited: a derivation (`PAGE-derivations`: statement, steps each citing a read source, cross-check, check table, then `knowledge add derivation -`) or a research request.
4. **Record**: each source a `REF-*` (`knowledge add source -`, what it says that we use, `[verified] YYYY-MM-DD`); the note (`knowledge add note -`): per story, the sources and what stays open.
5. **The stories updated** in the records: criteria from the sources' examples, the `@see` targets, the business questions for the Product Owner.
6. Finished as a story (`/story`, step 6); an epic's refinement into `develop`: `npm run story:finish -- develop …`.
