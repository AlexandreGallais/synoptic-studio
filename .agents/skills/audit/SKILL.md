---
name: audit
description: Run an audit (AUD story) — mathematics, sources, provenance, duplicates, consistency — with an independent auditor subagent. Use at a feature's end (a light audit, none in an epic tagged method and tools), at the end of an epic tagged method and tools (the thorough one), or for a story of type AUD.
argument-hint: "[audit id, e.g. AUD-001]"
---

# Audit

Audit: $ARGUMENTS. Checklist: the records' `audit_checks` (A to H, `knowledge sql "SELECT section, number, item FROM audit_checks"`); the findings go into the audit's row of the records, each with its evidence. A feature's audit is light: that feature only, before its full check; an epic tagged **method and tools** has no feature audit but one thorough audit at its end, over the whole epic and the project in breadth.

1. **Scope**: the feature's functions, tests, derivations, rules and sources in depth; at an epic's audit, the project in breadth too.
2. **Independent review**, in the background: the `auditor` first; the `math-expert` too for a feature holding mathematics. They return findings with evidence and edit nothing.
3. **Mutations**, one per function at least: `target/release/heavy-lock target/release/mutate <files>` lists the survivors; a mutation it cannot write is made by hand, its test run, the file restored. A tooling epic's mutations are all done here.
4. **Properties** added where a `math` or `geometry` function lacks one.
5. **Each finding reproduced** before it is recorded; one that changes neither correctness nor a requirement is "no action", with its reason.
6. **Fixed**: small findings in the audit's commits; larger ones become stories for the Product Owner to order.
7. **Breadth**, at an epic's audit: `AGENTS.md`, `CLAUDE.md` and `.agents/` still describe how the work is done. The full check (`npm run check:all`) follows every audit, mechanical as a CI.
8. **For the next feature**: the sources and questions its refinement needs; anything worth improving becomes an improvement row (`knowledge sql "INSERT INTO improvements …"`) for the next review.
9. Branch, commits and finish as a story (`/story`).
