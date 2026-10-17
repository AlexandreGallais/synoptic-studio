---
id: EN-066
epic: E17
feature: F20
title: A model chosen per agent type
status: ready
points: 2
---

# E17 · F20 · EN-066 — A model chosen per agent type

Decided at F20's second round (2026-10-17): Sonnet for logs, backlog, merges and step texts; Opus for the rest; and one measured trial of Sonnet on a real story worker, compared with Opus. Every agent file says `model: inherit` today; Sonnet 5.5 costs half of Opus 5.5 on every kind of token, a worker about $12 of tokens on Opus against about $6 on Sonnet if it needs no more requests (`REF-CLAUDE-PROMPT-CACHING`, [research note 0024](../../research/notes/0024-f20-context-diet.md)). The model is resolved from the `Agent` call's `model`, then the agent file, then `CLAUDE_CODE_SUBAGENT_MODEL`, then the main conversation's (`REF-CC-SUBAGENTS`).

## Acceptance criteria

- Given each agent type of EN-065, when its file is read, then it names its model and the reason.
- Given one real story worked by a Sonnet worker, when it ends, then its tokens, cost, duration, failed calls and audit findings are written beside a comparable Opus story's, and the run skill keeps Opus or moves story workers to Sonnet from that comparison, shown to the Product Owner.
- Given a run, when the session report is read, then it shows the tokens per model.

## Tasks

One task = one commit, referenced as `EN-066.Tn`.

- [ ] T1 — The model in each agent file, with its reason (1 h)
- [ ] T2 — Tokens per model in the session report (2 h)
- [ ] T3 — One story worked on Sonnet, compared with one on Opus, recorded (2 h)
