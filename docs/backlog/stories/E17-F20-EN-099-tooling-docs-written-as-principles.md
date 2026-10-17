---
id: EN-099
epic: E17
feature: F20
title: The tooling docs written as principles
status: draft
points: 3
---

# E17 · F20 · EN-099 — The tooling docs written as principles

Asked by the Product Owner on 2026-10-17: many rules an agent reads were written close to how a decision was first explained in conversation — its examples, its order, its wording — so an agent has to infer the principle and may doubt where it applies. Each rule is rewritten to state the idea in general terms: the principle, its reason, how to tell a case, what to do on conflict, with an example only where it removes a doubt. The agent is critical of how a decision was explained, gives a wider range of examples when they help, and leaves out what does not matter: shorter docs that serve the reader beat exhaustive ones. Split at F20's refinement into one story per family of pages, each measured in tokens before and after (2026-10-17). A later idea, not for these stories: keep superseded ADRs out of the agent's context.

This story covers `docs/tooling/`.

## Acceptance criteria

- Given each page of `docs/tooling/`, when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR whose wording follows a conversation too closely, when a page applies it, then the ADR is left as written and the page states the principle.
- Given a page or a section that guards nothing an agent needs, when the pass finds it, then it is cut, not rewritten.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost: each old rule is traced to its new sentence.

## Tasks

One task = one commit, referenced as `EN-099.Tn`.

- [ ] T1 — Measure the pages in tokens and list their rules (1 h)
- [ ] T2 — Rewrite the pages as principles, each old rule traced (4 h)
- [ ] T3 — Measure again and record the figures (1 h)
