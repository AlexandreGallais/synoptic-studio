# Code conventions

Technical rules of the project: **how** code is written. The business **what** lives in [domain](../domain/), the **why** of choices in [ADRs](../adr/).

Almost everything here is **checked by ESLint** (`eslint/`). When a rule is automatic, the page names the rule: when in doubt, the configuration wins.

| Page                                   | Content                                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------------------------- |
| [architecture.md](./architecture.md)   | layers, functional core / imperative shell                                                |
| [files.md](./files.md)                 | one export per file, file names, folders                                                  |
| [functions.md](./functions.md)         | `@kind`, size limits, immutability, explicit inputs, long procedures                      |
| [naming.md](./naming.md)               | casing, constants, booleans, names per kind                                               |
| [imports.md](./imports.md)             | `index.ts` per folder, canonical paths, `import type`, cycles, boundaries                 |
| [documentation.md](./documentation.md) | mandatory TSDoc, tag order, `@see` and references                                         |
| [typescript.md](./typescript.md)       | compiler options, TypeScript / ESLint split                                               |
| [formatting.md](./formatting.md)       | EditorConfig, Prettier, blank lines, format on save                                       |
| [lint.md](./lint.md)                   | layout of `eslint/`, adding or changing a rule, custom rules                              |
| [testing.md](./testing.md)             | Vitest, hand-computed expected values                                                     |
| [writing.md](./writing.md)             | how documentation is written: page types, austere API reference, style                    |
| [audit.md](./audit.md)                 | checklist of the feature audit: mathematics, sources, provenance, duplicates, consistency |
| [exceptions.md](./exceptions.md)       | register of the places where the code turns a lint rule off, with their reasons           |

## Language

**English everywhere**: code, comments, TSDoc, commit messages, documentation, lint messages. The only French left is the glossary's French column (the working vocabulary of the target users).

## Definition of done

In `AGENTS.md` (Definition of done): `npm run fix`, `npm run check:all` green, documentation up to date, a summary in French shaped as in [writing](./writing.md#messages-to-the-product-owner).
