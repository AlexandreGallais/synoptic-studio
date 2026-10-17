---
id: EN-097
epic: E17
feature: F20
title: The conventions written as principles
status: ready
points: 3
---

# E17 · F20 · EN-097 — The conventions written as principles

Asked by the Product Owner on 2026-10-17: many rules an agent reads were written close to how a decision was first explained in conversation — its examples, its order, its wording — so an agent has to infer the principle and may doubt where it applies. Each rule is rewritten to state the idea in general terms: the principle, its reason, how to tell a case, what to do on conflict, with an example only where it removes a doubt. The agent is critical of how a decision was explained, gives a wider range of examples when they help, and leaves out what does not matter: shorter docs that serve the reader beat exhaustive ones. Split at F20's refinement into one story per family of pages, each measured in tokens before and after (2026-10-17). A later idea, not for these stories: keep superseded ADRs out of the agent's context.

This story covers `docs/conventions/`.

## Acceptance criteria

- Given each page of `docs/conventions/`, when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR whose wording follows a conversation too closely, when a page applies it, then the ADR is left as written and the page states the principle.
- Given a page or a section that guards nothing an agent needs, when the pass finds it, then it is cut, not rewritten.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost: each old rule is traced to its new sentence.

## Tasks

One task = one commit, referenced as `EN-097.Tn`.

- [ ] T1 — Measure the pages in tokens and list their rules (1 h)
- [ ] T2 — Rewrite the pages as principles, each old rule traced (4 h)
- [ ] T3 — Measure again and record the figures (1 h)

## Measures

Pages of `docs/conventions/`, measured on 2026-10-17. Tokens are the input a worker pays to load a page: each page was printed whole into this story's worker, one call at a time where it could be, and the growth of the next request's input read from the transcript (`usage`, input and cache tokens); 170,731 characters gave 74,118 tokens, **2.30 characters per token** (Markdown with code, tables and identifiers; the tool's framing included). Each page's figure is its characters divided by 2.30, rounded to the hundred.

| Page | Words before | Tokens before |
| --- | --- | --- |
| `README.md` | 337 | 1,100 |
| `architecture.md` | 412 | 1,200 |
| `audit.md` | 613 | 1,800 |
| `documentation.md` | 416 | 1,300 |
| `exceptions.md` | 993 | 3,800 |
| `files.md` | 1,567 | 4,500 |
| `formatting.md` | 616 | 1,800 |
| `functions.md` | 988 | 2,900 |
| `imports.md` | 411 | 1,400 |
| `lint.md` | 4,347 | 13,600 |
| `naming.md` | 614 | 1,600 |
| `rule-catalogue.md` | 2,729 | 12,100 |
| `rules.md` | 5,691 | 21,300 |
| `testing.md` | 1,777 | 4,800 |
| `typescript.md` | 283 | 1,000 |
| `writing.md` | 1,638 | 4,100 |
| **all pages** | **23,432** | **78,300** |
| a library story's set: `README`, `architecture`, `files`, `functions`, `naming`, `imports`, `documentation`, `testing` | 6,522 | 18,800 |

## Rules before the pass

Each rule of the pages, by page, with an id the trace of T2 uses. `rules.md` and `rule-catalogue.md` are the record and the catalogue of EN-093: verdicts with their evidence, and rule sets for other kinds of project; they state no rule of their own but the test for keeping a rule (`rules.md`, introduction).

- `README.md`: R1 conventions say how, the domain what, the ADRs why; R2 a rule checked by a tool names it, and the configuration wins on doubt; R3 English everywhere, the glossary's French column apart; R4 the definition of done is `AGENTS.md`'s.
- `architecture.md`: A1 the pipeline (ADR-0005); A2 the layers, their imports, DOM and kinds (`LAYERS`); A3 a layer exists from its first module; A4 an interaction's pure decisions live in `model`; A5 the core is pure (no mutation, `let`, loop, `throw`, host global); A6 the shell has effects, never mutates a parameter nor the model; A7 every decision in the core, the shell applies it; A8 portability (ECMAScript only, plain read-only data, no class, returned closure nor exception, TSDoc states the intent where JavaScript differs, `number` binary64, `EPSILON` and `SVG_DECIMALS` in the specification); A9 the layers present after E01.
- `files.md`: F1 one exported value per module, `export <declaration>`, in `src/` and `playground/`, barrels, entries and tests apart; F2 the file is the kebab-case of its export; F3 no export list nor re-export outside barrels; F4 a type beside the value only when in its signature; F5 any other type in its own file; F6 a private helper serves the one export, a shared one is a module; F7 kebab-case files and folders; F8 `<module>.test.ts` beside the module; F9 repository-wide tests and shared fixtures in `tests/`; F10 the playground tests' actions in `playground/user-actions/`; F11 an `index.ts` in every code folder; F12 folders named after the domains, the same in every layer, never a kind of code; F13 a folder holds modules or domain folders, never both; F14 the inventory of every domain folder; F15 a shape's type in its folder, value imports one way; F16 a whole-page test in the panel's folder; F17 15 modules a signal: split by a missing domain, or listed with its reason.
- `functions.md`: Fn1 a name is a field's standard term; Fn2 a high-level function reads like the formula; Fn3 top-level `function` declarations, the module read top-down; Fn4 arrows for inline callbacks only; Fn5 a function never modifies its arguments (rules listed); Fn6 model types `readonly`; Fn7 a procedure may call a DOM element's methods; Fn8 no default nor optional parameter in `src/`, defaults in the model, a variant is a named function or an explicit field; Fn9 exactly one `@kind`, a function of two kinds split; Fn10 why a tag and not a decorator; Fn11 one business context per function, the limits guardrails, over a limit split where a field names the parts, else stop and propose a new limit; Fn12 the limits measured by EN-093; Fn13 the limits per kind, how lines and complexity count, 3 parameters at most; Fn14 a long procedure is a list of named steps, its decisions in tested pure functions, no annotation lifts a limit; Fn15 no `any`, `!`, `as` but `as const` without a directive and its reason; Fn16 no class, `this`, `enum`, `for…in`, accessor, dynamic `import()`, and why no `enum`; Fn17 no spread of a fold's accumulator; Fn18 only `-1`, `0`, `1`, `2` inline.
- `naming.md`: N1 casing by role, the type read from the written code; N2 boolean prefixes; N3 2 to 40 characters, single letters for mathematics; N4 denied names; N5 names by kind; N6 a procedure verb added to `PROCEDURE_VERBS`, never a name bent; N7 every calculation a field names is a call to a function of that name, bare arithmetic only where none does or naming changes a float, with a comment; N8 each review and each epic's `NAM` story look for unnamed calculations; N9 a file named like its export.
- `imports.md`: I1 an `index.ts` per folder; I2 a barrel holds only sorted `export *` lines, autofixed; I3 its exceptions (tests, the playground's entry, `tests/index.ts`); I4 the canonical path, the child of the common ancestor, never `.ts` nor `/index`, never the own layer's barrel; I5 `import type` on its own statement; I6 the order of groups, written by oxfmt; I7 forbidden: cycles, self and duplicate imports, namespace imports, default exports but root configurations, dynamic imports, undeclared or development packages in `src/`, Node.js modules outside tooling, a layer not allowed, unused imports.
- `documentation.md`: D1 a TSDoc block on every top-level function, type and module constant, and on type members in `src/`; D2 the description in sentences, with the formula; D3 `@param` and `@returns` fragments, required but on test helpers; D4 the tag order and blank lines; D5 no type in JSDoc; D6 a one-line block when it fits; D7 no `@todo`, `@author`, `@version`, `@since`; D8 every function has a `@see`, its accepted forms; D9 a missing or unverified target is an error, an unread source never cited; D10 non-trivial `math` and `geometry` cite a reference or a derivation; D11 no source: a derivation or a research request; D12 the API pages built from the TSDoc.
- `typescript.md`: T1 TypeScript 7 alone, through oxlint and `npm run typecheck`; T2 the linters check what they can, `tsconfig.json` keeps the type system's options; T3 a file outside `tsconfig.json` is not type-checked; T4 declarations written per module, so an exported constant whose value is not a literal has its type written; T5 the type-aware rules run in oxlint, the project's linter reads the written code.
- `formatting.md`: Fo1 three tools, each its scope, no overlap; Fo2 one formatter command for every file; Fo3 which tool formats each file kind; Fo4 tables without padding, prose on its lines; Fo5 what `--check` does; Fo6 targets named, never `.`; Fo7 `npm run fix` and `lint-staged`; Fo8 blank lines between statements; Fo9 comments above the line, no `TODO`; Fo10 format on save in the editors.
- `lint.md`: L1 two linters in order; L2 every rule decided once, no preset; L3 everything an error; L4 strictest by default, relaxed by zone; L5 the configuration's test; L6 the zones; L7 changing a rule, and reading oxlint's release notes at an upgrade; L8 one-off exceptions, their reason and register row; L9 the `import.meta.dirname` pitfall; L10 how the project's linter is built; L11 the project's readings of rules without types (`no-restricted-syntax`, `require-atomic-updates`, `functional/no-return-void`, `naming-convention`, `no-non-function-verb-prefix`, `immutable-data`, `prefer-immutable-types`, `no-mixed-types`, `no-conditional-statements`); L12 the functional plugin's other rules and where they run; L13 `@stylistic`'s rules; L14 `import-x`'s rules; L15 the `local/` rules; L16 Unicorn's ported rules and the dropped ones; L17 the Markdown rules and the dropped one.
- `testing.md`: Te1 Vitest without globals, tests beside the module; Te2 tests first for `math` and `geometry`; Te3 expected values computed by hand, the computation in a comment; Te4 not computable: stop; Te5 floats with `toBeCloseTo` or `EPSILON`; Te6 shared test data in `tests/geometry-fixtures.ts`, never copied, never in `src/`; Te7 the font file read once; Te8 DOM tests under happy-dom, the shell thin; Te9 playground tests through the page and `user-actions/` only; Te10 one claim, one place (ADR-0070); Te11 shared fixtures; Te12 what a helper may do; Te13 time limits; Te14 the two Vitest projects; Te15 properties after examples; Te16 a criterion's id in its test title; Te17 the test strategy; Te18 coverage and fitness functions; Te19 a test that reads files made to fail once, audits' mutations; Te20 regression and end-to-end later; Te21 lint relaxations in tests.
- `writing.md`: W1 five rules (point first, one purpose, show then tell, short blocks, one fact one place); W2 page types, a reference links its explanation, titles; W3 building blocks; W4 images; W5 the Product Owner's decisions, never their words; W6 writing a rule; W7 style; W8 the austere TSDoc; W9 the word list; W10 page skeletons; W11 messages to the Product Owner; W12 what the linter checks; W13 where things go.
- `audit.md`: Au1 the audit's evidence in its Findings table, a problem fixed or a story; Au2 the `auditor` first, its findings reproduced; Au3 to Au10 checks A to H.
- `exceptions.md`: X1 a rule turned off for one place only, with its directive; X2 a row added or removed in the same change, checked by `tests/exceptions.test.ts`; X3 one row per directive; the rows.
