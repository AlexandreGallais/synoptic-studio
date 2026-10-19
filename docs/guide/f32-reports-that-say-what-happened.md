# F32 — Reports that say what happened

The status line, `metrics`, `session-report`, `epic-check`, `val-prepare` and `context-size` now report what really ran. A tooling feature, validated by the agent (VAL-033).

## What changed

| Before | After |
| --- | --- |
| the status line showed a subagent "running" long after it ended (174 refused calls left in `.git/metrics/running/`) | a subagent's calls are dropped when it stops (`SubagentStop`), and the status line drops those of an ended one itself |
| `metrics` counted `cd …` for every worker's call | the command after the brief's `\|\| exit 9` is named |
| a Sonnet subagent's tokens undercounted (657 written against 23,875) | `session-report --all` and `metrics`' Sessions table read Claude Code's own count by model |
| `epic-check` left a `dropped` story open and said nothing of the CI into `main` | a `dropped` story is closed; the CI of the last pull request into `main` is reported, job by job |
| 59.6 s shown as `60 s` | `1 min 00 s`, the unit chosen after rounding, in the reports and the status line |
| `val-prepare` wrote a line for a struck criterion | no line for a criterion struck whole; one partly struck keeps its line |
| `context-size` read only sessions | given a file, it prints its tokens at EN-097's 2.30 characters per token |

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. the status line and `metrics` say what ran | `f32_ac1_*` (`hooks`, `metrics`) | passed |
| 2. the review's checks and reports | `f32_ac2_*` (`repo-tools`, `metrics`, `val-prepare`, `hooks`) | passed |
