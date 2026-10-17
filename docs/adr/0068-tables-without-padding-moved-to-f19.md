# ADR-0068 — Tables without padding moved to F19's formatter

**Status**: Accepted, confirmed by the Product Owner at F18's validation (VAL-019, 2026-10-17). Accepted by the agent first (2026-10-17, EN-048 and AUD-020, the choice left to it by the Product Owner on 2026-10-17). Supersedes ADR-0065 decision 1 (Prettier's `proseWrap: "never"`); ADR-0065's other decisions stand. Decision 2 carried by EN-073 (2026-10-17): oxfmt pads every table with no option against it, so the Rust tool `tools/format` runs oxfmt, then writes the tables without padding.

## Context

- ADR-0065 decision 1 expected Prettier's `proseWrap: "never"` to stop padding Markdown tables, so that a wider cell no longer rewrites a whole table.
- Prettier prints a table without padding only with `proseWrap: "never"` **and** when the padded table is wider than `printWidth` (`REF-PRETTIER-TABLE-CODE`): about 61 of 607 tables would stay padded (SP-020, [note 0022](../research/notes/0022-f18-parallel-run.md)).
- Measured on the whole repository with Prettier 3.9.10 (EN-048): `proseWrap: "never"` joins 1 340 lines of paragraphs and lists into 59 lines of hundreds of characters; a Markdown override with `printWidth: 1` changes nothing; only a local JavaScript plugin reading an undocumented behaviour of Prettier's table printer unpads the tables alone.
- Every new tool is written in Rust (E17, 2026-10-16), and F19 moves formatting to Oxc and Rust.

## Decision

1. **Prettier keeps padding Markdown tables** until F19: `proseWrap` is unchanged, no plugin is added.
2. **F19's formatter prints Markdown tables without padding and keeps paragraphs as written**: a criterion of F19 ([feature](../backlog/features/E17-F19-oxc-and-rust-tools.md)).
3. **F18.AC4** keeps only the logs merged by union, rows kept unique (EN-048, `tests/logs.test.ts`).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Moved to F19's formatter (chosen) | readable paragraphs; no tool to drop | padded tables keep conflicting until F19 (`agent-tools.md` at EN-051, EN-052, EN-058, EN-059) |
| `proseWrap: "never"` (ADR-0065 decision 1) | Prettier's own option | 61 tables still padded; 1 340 lines joined into lines of hundreds of characters |
| A local Prettier plugin | every table unpadded now | a JavaScript tool against the Rust rule, on an undocumented behaviour, dropped at F19 |

## Consequences

- The shared files stay written by the main thread at merge (ADR-0065 decision 4), which keeps padded tables from conflicting between workers meanwhile.
- The stories index, generated, is written unpadded by `backlog-index`, then padded by Prettier (`npm run backlog:index`).

## References

ADR-0065, `REF-PRETTIER-TABLE-CODE`, [EN-048](../backlog/stories/E17-F18-EN-048-tables-without-padding-and-union-logs.md), [note 0022](../research/notes/0022-f18-parallel-run.md)
