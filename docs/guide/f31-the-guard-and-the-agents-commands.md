# F31 — The guard and the agent's commands

`guard-bash` now reads a command as the shell does, word by word, so that it refuses less by mistake and nothing it must refuse gets through; and the agent's own commands say what they do. A tooling feature, validated by the agent (VAL-032).

## What changed

| Before | After |
| --- | --- |
| in a worktree, `git status; git log -3` refused like any chain | after the brief's `cd "<worktree>" \|\| exit 9`, a chain of Git reads goes through; `target/release/git-sequences state` prints the branch, its base, the changes and the commits ahead in one call |
| a fake here-document (`echo '<<EOF'`) or a text piped into `bash` could hide a `git push -f`; a push from a `cd` chain was judged in the wrong folder | a here-document read as the shell reads it, the text a shell reads judged as commands, a push judged in every folder it may run in and refused when that folder cannot be known (AUD-033) |
| a worker could write a file of the main checkout | on `Write` and `Edit`, a worker is refused a file of the main checkout or of another worktree; the handler fails closed |
| `mkdir .git/heavy.lock` took the heavy lock by hand | refused in every form, `install -d` and variables included |
| `replace --help` printed nothing; `npm run fix` turned `export type *` into `export *` | `replace` prints its usage; a types-only barrel line is kept (ADR-0092) |

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. the guard reads by words, refuses no less | `f31_ac1_*` (`hooks`, `git-sequences`); 109 audit cases, every form `CLAUDE.md` lists refused | passed |
| 2. `replace` and `npm run fix` do what they say | `f31_ac2_*` (`repo-tools`, `lint`) | passed |

A push from the main checkout must use the brief's form: `cd "<worktree>" \|\| exit 9`, then the push.
