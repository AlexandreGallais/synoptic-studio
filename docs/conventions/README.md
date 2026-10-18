# Code conventions

How code is written here. What the code does lives in the [domain](../domain/), and why a choice was made lives in the [ADRs](../adr/). Most rules are checked by a linter, and each page names the check. When a page and the configuration disagree, the configuration wins and the page is fixed.

## Pages to apply

Read the pages a change touches:

| Page | Content |
| --- | --- |
| [architecture.md](./architecture.md) | layers, functional core and imperative shell, portability |
| [files.md](./files.md) | one export per file, file and folder names, folders by domain |
| [functions.md](./functions.md) | one context per function, `@kind` and its limits, inputs, forbidden constructs |
| [naming.md](./naming.md) | casing, names by kind, every calculation named |
| [imports.md](./imports.md) | barrels, canonical paths, `import type`, forbidden imports |
| [documentation.md](./documentation.md) | TSDoc, its shape, `@see` |
| [typescript.md](./typescript.md) | compiler options, what TypeScript checks and what the linters check |
| [formatting.md](./formatting.md) | the formatter, blank lines, comments, format on save |
| [testing.md](./testing.md) | hand-computed examples, properties, one claim in one place, coverage |
| [lint.md](./lint.md) | the two linters, their zones, changing a rule, one-off exceptions |
| [exceptions.md](./exceptions.md) | the register of every place a lint rule is turned off |
| [writing.md](./writing.md) | how documentation and messages are written |
| [audit.md](./audit.md) | the checklist of a feature audit |

## Records

Read only to change a rule or to set up another project:

| Page | Content |
| --- | --- |
| [rules.md](./rules.md) | every rule, setting, check and hook, with what it guards and its verdict (EN-093) |
| [rule-catalogue.md](./rule-catalogue.md) | rule sets for other kinds of project, on paper only |

Code, comments, documentation and commits are written in English. The only exception is the glossary's French column. The definition of done is in `AGENTS.md`.
