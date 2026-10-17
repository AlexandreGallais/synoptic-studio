# ADR-0065 — Parallel branches merged without conflicts on shared files

**Status**: Accepted (Product Owner, 2026-10-16, at RET-003). Decision 1 superseded by ADR-0068 (2026-10-17): tables without padding moved to F19's formatter.

## Context

- On 2026-10-16 parallel story branches each edited the stories index, the feature file, the logs and the checklist. Their merges conflicted, and the conflicts were left open for hours.
- Prettier pads Markdown tables to their widest cell: a wider cell rewrites the whole table. In E15, 18 commits rewrote more than 100 of the stories index's 181 lines; `improvements.md` holds 138 KB for 34 KB of text, so each read costs about 35 000 tokens instead of 9 000.
- Git's `union` merge driver keeps both sides' lines without conflict markers, in an order to check (Git docs, gitattributes, read 2026-10-16). Tools such as Changesets and Towncrier keep one file per change and generate the summary from them.
- Tried on a copy: Prettier with `proseWrap: "never"` no longer pads a table wider than its print width; 276 docs files change once, the docs' Markdown goes from 3.56 MB to 2.55 MB.

## Decision

1. **Tables without padding**: Prettier's `proseWrap` is `"never"`.
2. **Append-only logs merged by union**: `journal.md`, `improvements.md` and `feedback.md` get `merge=union` in `.gitattributes`; a test checks each log's rows stay unique and dated. Never for a table with a sorted order or a file with front matter.
3. **The stories index is generated** from each story's front matter by `npm run backlog:status`, never written by hand.
4. **Shared files are written by the main thread only, at merge**: the indexes, the logs, the feature file and the checklist. A worker writes its rows in its story file; a pre-push check refuses a story branch that edits a shared file.
5. **One merge at a time**: `story:finish` takes a merge lock, rebases the next branch on the updated feature branch, then merges it.
6. **Not adopted**: `git rerere`, which replays a resolution recorded by hand; its gain is low here and the Product Owner is unsure of an automatic resolution.

## Reasons

| Option                   | For                                                                   | Against                                          |
| ------------------------ | --------------------------------------------------------------------- | ------------------------------------------------ |
| Avoid conflicts (chosen) | no conflict to resolve on indexes and logs; fewer tokens to read them | five changes in the tools (EN-048 to EN-051)     |
| Resolve conflicts better | no change in the tools                                                | each conflict read and resolved again, in tokens |
| A merge queue on GitHub  | checks each branch on the latest target                               | only for repositories owned by an organization   |

## Consequences

- EN-048 (padding, union), EN-049 (generated index), EN-050 (shared files at merge) and EN-051 (one merge at a time, from a worktree) apply it.
- The worker's brief in the run skill says which files are shared.

## References

ADR-0038, ADR-0045, ADR-0063, Git docs (gitattributes, git-rerere), `docs/process/retrospectives/E15-retrospective.md`
