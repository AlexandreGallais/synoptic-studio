---
id: F34
epic: E17
title: The use of each workflow
status: in-progress
---

# E17 · F34 — The use of each workflow

**Benefit hypothesis**: the Actions tab holds only the workflows the project uses, the auto-merge one workflow per base, each run once per pull request.

Taken out of F28 on 2026-10-18 so that F28 could end while its questions wait: which workflows stay is the Product Owner's decision, on SP-024's proposal (research note 0026). A tooling feature, validated by the agent (ADR-0072).

## Acceptance criteria

1. `story-automerge.yml` is two workflows, one per base, each one run per pull request, the last exception of `[F28.AC1]` removed; `external-prs.yml`, `links.yml` and `codeql.yml` are kept, changed or removed as the Product Owner decided (EN-134).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-134](../stories/E17-F34-EN-134-the-other-workflows-reviewed.md) | EN | The auto-merge per base and the other workflows reviewed | done |
| [AUD-036](../stories/E17-F34-AUD-036-audit-f34.md) | AUD | Audit F34 | done |
| [VAL-035](../stories/E17-F34-VAL-035-validate-f34.md) | VAL | Validate F34 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-134 | AUD-036, VAL-035 |
