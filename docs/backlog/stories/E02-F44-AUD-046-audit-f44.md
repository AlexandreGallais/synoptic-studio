---
id: AUD-046
epic: E02
feature: F44
title: Audit F44
status: ready
points: 1
---

# E02 · F44 · AUD-046 — Audit F44

Audit F44 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F44's criteria, when the audit ends, then each is traced to a test titled `[F44.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — A text typed kept apart from the value applied (findings 1 and 2): the number a relative value starts from read at the focus and at each key while no text typed waits in the field, a redraw reading the value applied; the test helper typing as a browser does, a key per character and for Enter (1 h)
- [ ] T2 — The rule for every number field (findings 5 and 9 to 20): a value beyond a field's range taken at its bound or wrapped, a fraction rounded once at the end, only a text that is no number refused; `interaction.md` §2 and §7, Q56, US-085 AC4 and US-087 AC2 (2 h)
- [ ] T3 — The number fields' mechanism made one (findings 3 and 4), the rest of the findings, the criteria traced and broken by hand, the mutations (2 h)

One task = one commit, referenced as `AUD-046.Tn`; split in three on 2026-10-24 as the work outgrew one commit.
