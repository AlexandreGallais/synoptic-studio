---
id: EN-066
epic: E17
feature: F20
title: A model chosen per agent type
status: done
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
- [x] T3 — One story worked on Sonnet, compared with one on Opus, recorded (2 h): the clerk and doc editor measured here; the story-worker trial runs on EN-096 (Sonnet) beside EN-097 (Opus), two stories of the same kind, its result recorded in CHK-021

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

## Measures

Short real tasks, 2026-10-17, each run once by `claude -p --agent <type> --output-format json` in this story's worktree, with the agent files of T1; the model from the file, or forced by `--model` for the comparison. Tokens: input, cache writes, cache reads and output, from `modelUsage`; cost as Claude Code reports it.

| Agent type | Task | Model | Requests | Time | Tokens | Written | Cost |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `clerk` | the journal entry of EN-066 from the journal's header and the branch's last two commits | `sonnet`, from the file | 7 | 10.3 s | 102,340 | 773 | $0.11 |
| `clerk` | same | `opus`, forced | 6 | 9.4 s | 80,103 | 783 | $0.21 |
| `doc-editor` | review of the section Models of `docs/tooling/agent.md` | `opus`, from the file | 6 | 27.3 s | 87,330 | 2,132 | $0.20 |
| `doc-editor` | same | `sonnet`, forced | 4 | 15.4 s | 59,711 | 1,530 | $0.10 |

- The file's `model` is applied: `clerk` answered on `claude-sonnet-5-5`, `doc-editor` on `claude-opus-5-5`, without `--model`.
- `clerk`: the same entry on both models, the same facts and format; Sonnet took one more request and 28 % more tokens, and still cost half.
- `doc-editor`: both found the same three problems (the order of the model's sources as a list, the unexplained `inherit`, the reason repeated in the table); Opus also flagged the story worker's condition as bound to go stale. Their common findings are applied to the section.
- A run's real share per model comes from the session report's new row: on this session, `opus-5-5 2,580,122,562 · haiku-5-5 237,698`.
- Not measured here: the trial of T3, one real story worked by a Sonnet `story-worker` beside a comparable Opus one, with its failed calls and its audit findings. A worker cannot start another agent: the main thread runs it, with `model: "sonnet"` on the `Agent` call.
