---
id: SP-025
epic: E02
title: Cut the editing work into small features
status: in-progress
points: 2
---

# E02 · SP-025 — Cut the editing work into small features

The Product Owner asked on 2026-10-19, before E02 starts, for the editing work to be cut again into small features that each make sense, so that the history keeps one bubble per story and one per feature, and so that no epic holds a long list of features; epics may be split or regrouped with the others where the themes fit better. This spike is the cut only, from `develop`: each feature's questions and sources come with its own short spike when it starts (ADR-0086).

## Acceptance criteria

- Given E02's scope and the Product Owner's ideas filed under it, when the spike ends, then each piece of it belongs to one epic of 2 to 5 features grouped by one theme, each feature with the 2 to 4 stories it is expected to hold, in the order they are to be made.
- Given the other epics and the domain, when the spike ends, then work that fits another epic better is moved there, the domain's references to E02 point at the epic that now holds the work, and the epics' order says where the new epics stand.

## Tasks

- [x] T1 — The epics cut and regrouped, their features and expected stories, the domain's references moved (main thread)

One task = one commit, referenced as `SP-025.T1` in the commit footer.
