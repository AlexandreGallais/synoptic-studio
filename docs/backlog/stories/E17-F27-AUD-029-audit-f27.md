---
id: AUD-029
epic: E17
feature: F27
title: Audit F27
status: done
points: 1
---

# E17 · F27 · AUD-029 — Audit F27

A small change of `tools/git-hooks`, audited by the main thread (ADR-0086).

## Acceptance criteria

- Given F27's criterion, when the audit ends, then it is traced to a test titled `f27_ac1_…`, seen failing first.

## Tasks

- [x] T1 — Criterion traced, the hook's refusal kept (main thread)

One task = one commit, referenced as `AUD-029.T1`.

## Findings

Audited by the main thread (ADR-0086).

| Id | Check | Result |
| --- | --- | --- |
| 1 | `f27_ac1_accepts_a_retired_scope_in_the_history_only` traced, seen failing before the fix (EN-128.T1) | passed after T2 |
| 2 | The `commit-msg` hook still refuses `eslint`: `f21_ac4_scope_enum_refuses_a_scope_out_of_the_list` unchanged and passing | no finding |
| 3 | Only `scope-enum` is dropped, and only for a retired scope: another error and an unknown scope stay refused (`f27_ac1`) | no finding |
| 4 | `git-hooks commits --from origin/main --to origin/develop` exits 0 on the real history | no finding |
