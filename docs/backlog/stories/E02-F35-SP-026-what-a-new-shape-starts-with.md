---
id: SP-026
epic: E02
feature: F35
title: What a new shape starts with
status: ready
points: 2
---

# E02 · F35 · SP-026 — What a new shape starts with

A short spike, the first story of F35 after its split (2026-10-22). When a tool places a shape, its settings other than its box — a corner radius, a polygon's corners, its anchor, a text's content, font and size — come either from the values the panel shows, so that a triangle placed again is the same triangle, or from fixed defaults, a repeat then left to duplicating. The Product Owner asked for the choice to be made from how graphic editors do it, with its reasons (Figma first, as the editor that thinks most about it, but not copied blindly), and decided by the agent.

## Acceptance criteria

- Given Figma, Penpot, Inkscape, Illustrator, Sketch and Affinity Designer, when the spike ends, then what each gives a new shape and a new text — fixed defaults, the last values used, a style chosen — is recorded from read sources, with the cases where each way helps.
- Given those sources, when the spike ends, then the rule F35 applies is written in `interaction.md` §1 with its reasons, the agent's choice to confirm at VAL-036, and US-063's T3 cites it.

## Questions

- What does each editor give a new rectangle, polygon and text: fixed defaults, the last values used, or a style the user sets as default?
- In which cases does each way help a designer placing several alike shapes, and where does duplicating take over?

## Tasks

One task = one commit, referenced as `SP-026.Tn`.

- [ ] T1 — The sources found, read and recorded, the research note (2 h)
- [ ] T2 — The rule in `interaction.md` §1 and US-063 updated (1 h)
