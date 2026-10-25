# ADR-0081 — The private records in a local folder shared by every project, never sent

**Status**: Accepted (Product Owner, 2026-10-18, EN-114). Supersedes ADR-0041 (a private repository, committed and pushed) and ADR-0044 (that repository in `records/` inside the project), both removed since (ADR-0101).

## Context

- `records-save` copied the metrics, the transcripts and the source library into `records/`, a Git repository with a private GitHub remote, then committed and pushed (ADR-0041, ADR-0044).
- On 2026-10-18 the push was refused: one transcript had grown to 238 MB, and GitHub refuses a file over 100 MB. The records held 2.3 GB on the disk, 1.9 GB of them transcripts, and 321 MB of Git history.
- The options weighed: transcripts cut into pieces of 50 MB, compressed (238 MB to 19 MB, but a whole new copy at each save), Git LFS (1 GB free, under what the records already hold), or kept on the computer only.
- The Product Owner chose to keep the records on the computer only, without a backup, in a folder they chose outside the project, which every future project with the same method uses too.

## Decision

1. **The records live in `<AGENT_RECORDS_ROOT>/<project>`**, a plain folder: no Git, no remote, nothing sent. `records-save` copies into it and writes the instant of the save into `last-save.txt`.
2. **`AGENT_RECORDS_ROOT` names the folder of every project's records**, set in the `env` of the user's `~/.claude/settings.json`, outside every repository: the folder's path names the user's account, which a public repository never holds. Unset, `records-save` fails and writes nothing; a folder that does not exist fails too, so that a mistyped path never makes a new one.
3. **Each project writes only into its own folder**, named after its repository (the last part of the `origin` remote's address, without `.git`), the same in every clone and every worktree; without a remote, the main checkout's folder name. Another project joins by copying `records-save`: its records land beside, never over, this one's.
4. The project's tools no longer skip a `records/` folder: none is left inside the project.

## Consequences

- A disk failure loses the records: the Product Owner accepts it.
- `epic-check` reports the last save, or the variable unset, instead of the records repository's Git status.
- The GitHub repository of the records is no longer written; the Product Owner deletes it.
- A transcript of any size is copied, since nothing limits a local file.

## References

ADR-0101 (ADR-0041 and ADR-0044, removed)
