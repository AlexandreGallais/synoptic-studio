# Principles

What makes code written by an agent trustworthy, and the work fast. Each principle names the decision behind it.

| # | Principle | In practice | Source |
| --- | --- | --- | --- |
| 1 | **Checks, not instructions, give control** | every rule that matters becomes a test, a lint rule, a hook or a CI job; a written rule alone is forgotten | ADR-0021, `REF-BOCKELER-SDD` |
| 2 | **Every claim has a source that was read** | references recorded with their date before the code; no source → a derivation from read sources, or a research request | ADR-0011, ADR-0020 |
| 3 | **Evidence over assertion** | "done" means the full check ran green and its output was seen | `REF-CC-BEST-PRACTICES` |
| 4 | **The one who did the work does not grade it** | independent reviewers in a fresh context — auditor, math expert; tests that read files are mutation-checked | ADR-0020, ADR-0021 |
| 5 | **Small, named, pure functions** | one export per file, one kind, one source; a pure core, effects in a thin shell | ADR-0011, ADR-0014, ADR-0019 |
| 6 | **Business rules belong to the Product Owner** | the agent asks instead of inventing; every answer goes to the domain docs in the same change | `AGENTS.md` |
| 7 | **Inspect at the end of every delivery** | what changed tried at each return, a review of the work and the method at each return of the Product Owner, every rule questioned every four to five epics | ADR-0023, ADR-0086, `docs/process/agile.md` |
| 8 | **Zero runtime dependency** | every dev dependency justified, read before upgrade, at least 7 days old, audit blocking | ADR-0072, ADR-0075 |
| 9 | **Decisions are written once** | ADRs superseded, never rewritten, and removed into one record once they no longer hold; docs say "why" once and link to it | `docs/conventions/writing.md` |
| 10 | **The repository is the memory** | backlog, decisions, sources and lessons in versioned Markdown that the agent reloads at each session | ADR-0021, `REF-ANTHROPIC-CONTEXT` |
| 11 | **Measure, then speed up** | every command timed, every story's time and tokens known; the slowest and the most frequent become tools | `docs/tooling/metrics.md` |
| 12 | **Read once, keep it** | pages read online kept in a local library, searched before going online: fewer requests, fewer tokens | ADR-0039 |
| 13 | **Never wait for a server** | checks run locally before a push; one CI, before a release | ADR-0072, ADR-0088 |
| 14 | **Write decisions, not words** | the Product Owner's choices recorded in neutral, complete sentences, never quoted | `docs/conventions/writing.md` |
