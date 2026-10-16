---
id: EN-059
epic: E17
feature: F18
title: The done commit checks the docs a rule touches
status: done
points: 1
---

# E17 · F18 · EN-059 — The done commit checks the docs a rule touches

Keep the Q rows, the decisions to confirm and the guide pages up to date when a story changes a rule (row 30 of RET-003; I43, I49; F18.AC16).

## Acceptance criteria

- Given a story that changes a rule in `docs/domain/`, when its done commit is made, then a check asks for the Q row of `docs/domain/README.md`, the VAL's decisions to confirm and the guide page to be in the same change, or a line saying none is affected.

## Tasks

One task = one commit, referenced as `EN-059.Tn`.

- [ ] T1 — The check in `story:finish` or a hook; its test (2 h)

## For the main thread

- Technical choices made by the worker: the check is the Rust crate `tools/rule-docs/` (binary `check-rule-docs`, in `check:rust`), run by the `commit-msg` hook only when the message has a `Closes:` footer. At commit rather than at push: the `done` commit is refused before it exists and made again, never amended after a push; the hook sees the staged files and the message together.
- A story's change is the files of its commits — those whose `Refs:` or `Closes:` footer names the story or one of its tasks — plus the files staged; a rule is changed when one of them is under `docs/domain/`. Each doc is asked for when a file is changed (the README, the feature's VAL file, the guide page `f<nn>-*.md`, `e<nn>-*.md` for an epic's story), not the rows inside it.
- The line saying none is affected is a footer of the `done` commit, exactly `Rule-docs: none affected, <why>`, its reason after the comma; it covers every doc the check would ask for. Written in the story skill's step 7 and the worker brief.

### Journal

- EN-059: a story's `done` commit that changes a rule in `docs/domain/` is refused by the `commit-msg` hook (`tools/rule-docs`, Rust) until its change holds the Q row, the feature's VAL decisions to confirm and the guide page, or the footer `Rule-docs: none affected, <why>`.
