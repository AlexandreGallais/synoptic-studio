---
id: EN-094
epic: E17
feature: F19
title: The ADRs reviewed with the Product Owner, the review applied
status: done
points: 3
---

# E17 · F19 · EN-094 — The ADRs reviewed with the Product Owner, the review applied

On 2026-10-17 the Product Owner went over every accepted ADR with the agent, one by one, to say whether they still agree now that the project has moved on. They kept every ADR of the domain and of the method, and asked for the changes below. Superseded ADRs (0015, 0042, 0051) were not asked again. This story writes the answers down before EN-093 reviews the rules and tools.

## The review (Product Owner, 2026-10-17)

| ADR | Verdict |
| --- | --- |
| 0001–0008, 0011, 0012, 0014, 0016–0019, 0021, 0023, 0025–0027, 0029–0035, 0037, 0040, 0043, 0044, 0046–0049, 0052, 0054–0062, 0064–0066 | kept as they are (0011's limits and 0012's barrels measured again by EN-093; 0014's "never modify an argument" measured at the end-of-project optimisation epic; 0062's forced export decided in E02) |
| 0053 | already superseded by ADR-0060 for its HTML layer: nothing to change |
| 0009, 0010, 0013 | replaced by EN-085's ADR on Oxc: the toolchain changes, the dependency rules stay (dev only, exact versions, each justified, audited); "the linter first" stays, the linter being oxlint and the project's Rust linter |
| 0020 | kept; a tooling feature's validation is made by the agent, who opens its pull request into `develop` without waiting (decided 2026-10-17) |
| 0022 | decisions 1 and 2 kept; decision 3 (mutation tool deferred) replaced: mutation testing is done by the project's own `mutate` (EN-058) |
| 0024 | kept until SP-022 measures a local memory; a new ADR replaces it only if the measure says so |
| 0028, 0038, 0045, 0050, 0063 | gathered into one up-to-date ADR on how a run works: feature branches from `develop`, stories merged by GitHub, `/run` as the go-ahead, a tooling feature validated by the agent, releases when the Product Owner asks, at most three subagents and one heavy command at a time — no longer a trial (0063) |
| 0039 | kept; its implementation moves to Rust in F21; SP-022 may extend it |
| 0050 | decision 1 ("the agent's tools are TypeScript scripts") replaced: every new script, hook or agent tool is written in Rust (2026-10-16), the rule recorded by an ADR for the first time |
| 0067, 0068 | confirmed (VAL-019): their status lines say so |
| the agent's own method (Product Owner, 2026-10-17) | a new ADR: the agent is given growing freedom over its own method and tools; it logs each friction, error or slow step as it happens in the improvement log, with each reading that suggests a faster, cheaper or healthier way to work; each epic end holds a "method and tools" point at the retrospective, beside the tooling pass, where it presents what it wants to change — the backlog method itself included when it does not fit an agent — with its evidence, and new methods or Rust tools to adopt; the Product Owner decides |

## Acceptance criteria

- Given the review, when the story ends, then a new ADR gathers the runs (superseding 0028, 0038, 0045 and 0063, and 0050's decisions 2 and 3, each status line updated), a new ADR records "every new tool in Rust" (superseding 0050's decision 1), 0022's status says decision 3 is replaced by `mutate`, 0020's says how a tooling feature is validated, and 0067 and 0068 read "Accepted".
- Given EN-085, when its ADR on Oxc is written, then it supersedes 0009, 0010 and 0013 as decided here; this story writes that requirement in EN-085's file.
- Given `AGENTS.md`, `CLAUDE.md`, the run skill and the worker brief, when the ADRs change, then they cite the new ADRs.
- Given the decision on the agent's own method, when the story ends, then a new ADR records it, and `docs/process/agile.md` and the retro skill hold the logging during the work and the "method and tools" point of each epic end.

## Tasks

One task = one commit, referenced as `EN-094.Tn`.

- [x] T1 — The ADR of the runs, the ADR of the Rust tools, the status lines, the index (2 h)
- [x] T2 — EN-085's requirement, AGENTS.md, CLAUDE.md, skills and brief in step (1 h)
- [x] T3 — The ADR of the agent's own method, `agile.md` and the retro skill (1 h)
