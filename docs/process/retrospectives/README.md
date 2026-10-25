# Retrospectives

One record per epic retrospective, named after its epic (`E01-retrospective.md`): a `RET` story up to E17 (ADR-0023), then the method's part of the review that follows the epic's end, E02's at REV-008 (ADR-0086). A review that ends no epic records its method's part in its own story file. Template: `_retrospective.md` in this folder (not published).

A review reads `target/release/metrics --since <day of the last merge into main>` ([metrics](../../tooling/metrics.md#at-each-retrospective)): the commands by count and by time, the stories and tasks, the failed calls, the tokens, and its **Tools** section — each Rust tool's calls, failures, median and slowest durations, from the uses each tool records (EN-102) — read as a tools team reviews its tools: unused, failing or slow.

| Epic or review | Record |
| --- | --- |
| E01 | [E01-retrospective.md](./E01-retrospective.md) |
| E14 | [E14-retrospective.md](./E14-retrospective.md) |
| E15 | [E15-retrospective.md](./E15-retrospective.md) |
| E17 | [E17-retrospective.md](./E17-retrospective.md) |
| E02 | [E02-retrospective.md](./E02-retrospective.md) |
