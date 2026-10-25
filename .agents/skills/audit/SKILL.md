---
name: audit
description: Run an audit (AUD story) — mathematics, sources, provenance, duplicates, consistency — with an independent auditor subagent. Use at a feature's end, at the end of an epic tagged method and tools, or for a story of type AUD.
argument-hint: "[audit id, e.g. AUD-001]"
---

# Audit

Audit: $ARGUMENTS. Checklist: `docs/conventions/audit.md` (A to H); the findings go into the audit's row of the records, each with its evidence. Stories have no review, so the audit is the feature's thorough check; an epic tagged **method and tools** has one, at its end, over the whole epic.

1. **Scope**: the feature's functions, tests, derivations, rules and sources in depth; the project in breadth.
2. **Independent review**, in the background: the `auditor` first; the `math-expert` too for a feature holding mathematics. They return findings with evidence and edit nothing.
3. **Mutations**, one per function at least: `target/release/heavy-lock target/release/mutate <files>` lists the survivors; a mutation it cannot write is made by hand, its test run, the file restored. A tooling epic's mutations are all done here.
4. **Properties** added where a `math` or `geometry` function lacks one.
5. **Each finding reproduced** before it is recorded; one that changes neither correctness nor a requirement is "no action", with its reason.
6. **Fixed**: small findings in the audit's commits; larger ones become stories for the Product Owner to order.
7. **Breadth**: `npm run check:all`, the full one; `AGENTS.md`, `CLAUDE.md` and `.agents/` still describe how the work is done.
8. **For the next feature**: the sources and questions its refinement needs; anything worth improving becomes an improvement row (`knowledge add improvement -`) for the next review.
9. Branch, commits and finish as a story (`/story`).
