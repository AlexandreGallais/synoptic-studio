---
id: EN-121
epic: E02
feature: F23
title: Builds, lanes and the checks RET-004 listed
status: draft
points: 3
---

# E02 · F23 · EN-121 — Builds, lanes and the checks RET-004 listed

From the cards C8 and C9 and the rows I37, I51, I54, I55, I56, I58, I60, I81, I90, I97, I99, I100, I106, I107 and I112 (RET-004).

## Acceptance criteria

- Given a pull of `develop` in the main checkout after a merge on GitHub, when it ends, then the release binaries are rebuilt (C9, I112).
- Given a change without Rust, when `check:all` runs, then `check:rust` is skipped, and with Rust its coverage runs as a lane of its own (I97, I106).
- Given a load average near 60, when the tests reading Inter run, then they pass, their limit set from a measure under load (C8, I51).
- Given the checks of RET-004's list, when they run, then each holds: the Rust coverage floor at 95 %, `typeCheck` in `.oxlintrc.json`, the cargo tools' versions checked, the editor's TypeScript path, one anchor slug for GitHub and the site, `move-rows` naming the notes it leaves, `backlog:index --check` reaching the index tool, no `npm run <binary>` in the agent's pages, and a `bench` binary for the stories' measures (I37, I54 to I58, I60, I81, I99, I100).

## Tasks

- [ ] T1 — Each criterion's tests first, failing; then the builds, lanes and checks (4 h)

One task = one commit, referenced as `EN-121.T1` in the commit footer.
