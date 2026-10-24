---
id: AUD-049
epic: E02
feature: F47
title: Audit F47
status: done
points: 1
---

# E02 · F47 · AUD-049 — Audit F47

Audit F47 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F47's criteria, when the audit ends, then each is traced to a test titled `[F47.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-049.T1`.

## Findings

The `auditor` alone reviewed F47 read-only (ADR-0086) and found no wrong result in US-091's fix of Space. F47's criterion is traced to the four `[F47.AC1]` tests of `press-left-open.test.ts`.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | The arrows held were released only at Cmd's `keyup` (`listenToHeldArrows`), which a system shortcut may take, as for Space before US-091: Up released under Cmd stayed held, and Right then moved A by (+1, −1) | fixed: any `keydown`, `pointerdown` or `pointermove` carrying Cmd releases every arrow held (`removeArrowsUnderCommand`, the pointer's in capture), Cmd's `keyup` no longer read; an arrow still down is held again at its next repeat without Cmd. Tests first, 3 failing before the fix: Up, then Cmd's `keydown` and no `keyup`, then Right gives (11, 19), not (11, 18); the same after a `pointerdown` or a `pointermove` with Cmd; Up repeated after Cmd, then Right, (11, 17). `interaction.md` §2 (Two arrows held), Q59, the guide page of F39 |
| 2 | A burst of arrows in a field ended only at Cmd's `keyup` (`closeBurstAtCommand`), the same shortcut able to take it, a modifier pressed in the field keeping it open | fixed: a `keydown` or a `pointerdown` carrying Cmd, heard in capture on the document before the panel's keys, ends the burst, its step taken; the `keyup` branch dropped. Tests first, 2 failing before the fix: W stepped from 100 to 103, then Cmd's `keydown` alone, 103 recorded at once, one Ctrl + Z giving back 100; the same at a `pointerdown` with Cmd. The test keeping a burst open at a modifier no longer names Cmd. `interaction.md` §7 (A burst of arrows in a field), Q7, the guide page of F40, `listenToArrowBursts`'s TSDoc |
| 3 | US-091 cited `REF-MOZILLA-CMD-KEYUP` for Cmd's `keydown` coming before any key a shortcut may take, where that bug shows Cmd pressed first only | fixed: US-091 says the order is inferred, not observed |

Mutations (A4), `mutate --all`, the work staged first, on `listen-to-held-arrows.ts` and `listen-to-arrow-bursts.ts` against their related tests: 22 caught, 0 survived.

By hand, the criterion once: `removeSpaceUnderCommand` never letting Space go, against the tests of `selection-drag`, `selection-keys` and `view`, the four `[F47.AC1]` tests fail and one `[F46.AC2]`.

## For the main thread

- For the done commit's footer: the rule docs changed, `interaction.md` §2 (Two arrows held) and §7 (A burst of arrows in a field), the Q rows Q7 and Q59 of `docs/domain/README.md`, and the guide pages of F39 and F40; F47 has no guide page of its own and VAL-048 is dropped.
