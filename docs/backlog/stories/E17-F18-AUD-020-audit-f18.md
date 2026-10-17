---
id: AUD-020
epic: E17
feature: F18
title: Audit F18
status: ready
points: 1
---

# E17 · F18 · AUD-020 — Audit F18

Audit F18 in depth, the project in breadth ([audit.md](../../conventions/audit.md)).

## Acceptance criteria

- Given F18's stories merged, when the audit ends, then each criterion is traced to a test, the tools are mutation-checked, and the findings are fixed or logged.

## Tasks

One task = one commit, referenced as `AUD-020.Tn`.

- [x] T1 — Criteria traced to the Rust tools, scripts, hooks and lint rules; `backlog-index --check` tested on a backlog of its own (1 h)
- [x] T2 — Test gaps of the reviews closed: exceptions in Rust, a synthetic 200 MB buffer, boundaries and selectors (1 h)
- [ ] T3 — Skills and tooling pages say the flow of worktrees, `story:finish` and the release (1 h)
- [ ] T4 — The agent tools page, the other pages and the done stories cleaned (1 h)
- [ ] T5 — The agent limit said as it is, tables moved to F19 by an ADR, accepted limits written (1 h)
- [ ] T6 — The findings, the next feature's sources and the evolvability check (30 min)
