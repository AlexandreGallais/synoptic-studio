# F33 — The checks RET-004 listed

The checks E17's retrospective asked for now hold by themselves: the Rust coverage floor at 95 %, the type check in oxlint's configuration, the cargo tools' versions, the editor on TypeScript 7, one anchor per heading on GitHub and the site, the notes `move-rows` leaves, the index check, the agent pages, and a `bench` binary for the stories' timings. A tooling feature, validated by the agent (VAL-034).

## What changed

| Before | After |
| --- | --- |
| the Rust coverage floor at 93 % | 95 % of lines and functions (95.33 % and 96.46 % measured) |
| `--type-check` passed by each script | `typeCheck` in `.oxlintrc.json` |
| the cargo tools' versions looked up by hand | `deps:outdated` checks every place they are written, a newer release, and a pin under 7 days old, yanked or missing |
| the editor pointed at no TypeScript 7 server | TypeScript 7's extension and its two settings ([ADR-0093](../adr/0093-the-editors-typescript-the-installed-typescript-7.md)) |
| a link to a heading could work on GitHub and break on the site | `local/anchor-slugs` refuses a heading with two anchors; 14 links fixed |
| `move-rows` left notes silently | it names them, and the `done` commit shows them |
| `backlog:index --check` failed on the formatter's options | it runs the index tool's own check |
| timings written by hand | `target/release/bench --runs 5 -- <command>`: each run, the median, the load before and after |

## The guard and `bench`

`bench` runs any command it is given, so `guard-bash` judges that command as it judges `heavy-lock`'s; AUD-035 also closed a way past the guard through `cargo r` and cargo's global options, on `develop` before F33. Whether `bench` may run without asking is left to the Product Owner.

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. floors, versions and paths | `[F33.AC1]` and `f33_ac1_*` | passed |
| 2. anchors, notes, index check, agent pages | `[F33.AC2]` and `f33_ac2_*` | passed |
| 3. a `bench` binary | `f33_ac3_*`; the guard's tests under `bench` | passed; its permission waits for the Product Owner |
