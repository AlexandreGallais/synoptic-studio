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

## Models

Set in T1, each agent file's `model` and a line of its body giving the reason (`tests/agent-models.test.ts` checks both); the same table is in [agent configuration](../../tooling/agent.md#models). An alias rather than `inherit`: a file naming `opus` keeps its agent on Opus when the main conversation runs on another model.

| Agent type | Before | After | Why |
| --- | --- | --- | --- |
| `clerk` | `inherit` | `sonnet` | records copy facts the prompt gives into formats their files set, and a merge or a rebase follows fixed steps: nothing to derive or judge, at half the price |
| `story-worker` | `inherit` | `opus` | code, hand-computed tests and the docs of its rules; stays on Opus until T3's measured trial shows Sonnet does as well |
| `researcher` | `inherit` | `opus` | research on the strongest model: it judges what each source states, and the feature is built from its stories |
| `auditor` | `inherit` | `opus` | audits on the strongest model: a weaker reviewer than the worker misses more |
| `verifier` | `inherit` | `opus` | mutations by thought and recomputed values, the audit's reasoning on one story |
| `math-expert` | `inherit` | `opus` | mathematics on the strongest model: the tests' expected values come from it |
| `functional-analyst` | `inherit` | `opus` | finds the two readings of a rule and prepares the Product Owner's decisions |
| `doc-editor` | `inherit` | `opus` | a rewrite keeps every fact of the page; read-only and short, so its share of the tokens is small |
