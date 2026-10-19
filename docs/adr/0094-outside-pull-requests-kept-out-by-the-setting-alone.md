# ADR-0094 — Outside pull requests kept out by the repository's setting alone

**Status**: Accepted (Product Owner, 2026-10-18, on SP-024's proposal, research note 0026, §2). Supersedes, in ADR-0026's decision 1, its second barrier: the workflow `external-prs.yml`, which closed a pull request from an outside author. The rest of ADR-0026, the setting _Collaborators only_, input through issues and issue content as untrusted data, stands. Implemented by EN-134.

## Context

- ADR-0026 kept outsiders' pull requests out twice: the repository's setting _Pull requests → Collaborators only_, and, as a second barrier, `external-prs.yml`, started on every pull request opened or reopened, its job closing one whose author was neither the owner, a collaborator nor an allowed bot.
- Its last 200 runs, read on 2026-10-18, were all skipped: no outsider ever opened a pull request, the setting already refusing them (`REF-GH-PR-ACCESS`; research note 0026, §2).
- ADR-0088 asks the Actions tab to hold one workflow per purpose, used: a workflow whose every run is skipped shows a line on every pull request and does nothing.

## Decision

1. **`external-prs.yml` is removed.** The setting _Collaborators only_ (`REF-GH-PR-ACCESS`), done once by the owner ([repository settings](../tooling/ci.md#repository-settings-done-once-by-the-owner)), is the one barrier to an outside pull request.
2. **Outside input still comes through issues** (ADR-0026, decisions 2 and 3); `.github/CONTRIBUTING.md` says that only collaborators can open a pull request.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| The setting alone (chosen) | one workflow less, never run in 200 pull requests; the setting refuses an outsider before any pull request exists | a collaborator invited later could open one, which the owner reviews as any other |
| The setting and `external-prs.yml` (ADR-0026) | a second barrier if the setting were changed | a skipped run on every pull request, a `pull_request_target` workflow kept for a case that never came |

## Consequences

- `docs/tooling/ci.md` no longer lists `external-prs.yml`; `[F34.AC1]` (`tests/workflows.test.ts`) lists the workflows kept and reads `.github/CONTRIBUTING.md`.
- ADR-0026's status in the [index](./) names this decision.
- A change of the setting, opening pull requests to everyone, would need a new ADR, a barrier back included.

## References

ADR-0026, ADR-0088, `REF-GH-PR-ACCESS`, research note 0026, SP-024, EN-134
