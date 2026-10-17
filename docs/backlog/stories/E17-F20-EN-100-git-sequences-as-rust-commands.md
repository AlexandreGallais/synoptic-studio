---
id: EN-100
epic: E17
feature: F20
title: The agent's frequent Git sequences as Rust commands
status: ready
points: 2
---

# E17 · F20 · EN-100 — The agent's frequent Git sequences as Rust commands

A worktree refuses chained Git commands, so a worker sends each in its own call. From 2026-10-14 to 2026-10-17, 793 of 2,297 Git calls came right after another Git call of the same agent — `add` then `commit` 59 times, `commit` then `push` 46, `fetch` then `rebase` 39, `fetch` then `diff` 18 — and each call saved is about 2.6 s and 166,000 tokens read from the cache in a subagent: about 34 minutes and 132 million tokens over the three days ([research note 0024](../../research/notes/0024-f20-context-diet.md)). Chosen at F20's refinement (2026-10-17).

## Acceptance criteria

- Given a Git sequence seen more than 30 times in the hook log, when the agent needs it, then one Rust command of the workspace does it in one call — the feature branch fetched and the story rebased on it; the files named staged and committed from a message file — and stops at the first failing step with Git's own message.
- Given the worker brief and the run skill, when they are read, then they name the commands in place of the sequences.
- Given a run after the change, when the hook log is read, then the Git calls per story are counted and compared with F19's.
- Given each command, when it runs, then it records its use as every tool of our own does (F21), or in the hook log until then.

## Tasks

One task = one commit, referenced as `EN-100.Tn`.

- [ ] T1 — The commands, their crate and tests, stopping at the first failure (3 h)
- [ ] T2 — The brief and the run skill updated; the calls per story compared after one run (1 h)

## For the main thread

- **Measured** in temporary repositories, 10 rounds each, the release binary: `fetch` then `rebase` as two Git calls 188 ms, `git:sync` 237 ms (it also lists the remote branches and counts the commits ahead); `add`, `commit`, `push` as three Git calls 213 ms, `git:commit --push` 171 ms. Commands run in about the same time; the gain is the agent's calls, 2 to 1 and 3 to 1, each saved call about 2.6 s and 166,000 cached tokens in a subagent (research note 0024). In this worktree, `npm run git:commit` made T1's commit with its hooks in 1.45 s, and `npm run git:sync` rebased it on the feature branch in 1.04 s.
- **Still open, T2's comparison**: the Git calls per story counted from the hook log after the next run, against F19's.
- **Use recorded**: each call is a Bash call in the hook log (`npm run git:sync`, `npm run git:commit`) until F21's log of the tools' own use.
- `Rule-docs: none affected, a tool of the agent only`.

### Journal

- `#run` EN-100: `npm run git:sync` and `npm run git:commit` run fetch-and-rebase and add-commit-push in one call each; the worker brief, `/story` and `/run` name them.
