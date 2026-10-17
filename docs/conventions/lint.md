# Lint

Two linters run one after the other until F19 ends (`npm run lint`): **oxlint** first, with its native rules and `tsgolint`'s type-aware ones (`.oxlintrc.json`, written by `@oxlint/migrate` from `eslint/`, EN-074), then **ESLint**, for the rules oxlint lacks, which F19 rewrites in the project's Rust linter before ESLint leaves (EN-085). Each rule runs in one linter only: the last entry of ESLint's configuration (`eslint/scopes/oxlint-owned.ts`) turns off every rule `.oxlintrc.json` names and every rule the triage dropped (`eslint/settings/dropped-rules.ts`; the reasons in research note 0023), and `tests/oxlint-config.test.ts` checks it, with the scopes carried as `overrides`.

## Principle

- **Every rule of every plugin is decided once**, explicitly: enabled, configured, or disabled with its reason as a comment. No preset (`recommended`, `strict`…) is imported: the list is read, every rule is settled.
- Everything is `error`. `npm run lint` uses `--max-warnings=0`.
- **Strictest by default, relaxed by zone**: the files of `eslint/rules/` describe the strictest setting (the functional core's); `eslint/scopes/` relaxes or tightens by path.
- The test `eslint/config.test.ts` (run by `npm run test`) fails when a rule is undecided, decided twice, non-existent, or when a zone touches an undecided rule.

## Layout of `eslint/`

```text
eslint.config.ts          entry point: export default CONFIG
eslint/
  config.ts               assembly: ignores → language → Prettier → themes → zones → oxlint's
  config.test.ts          completeness audit
  settings/               data tables (layers, kinds, verbs, globs, limits, dropped rules…)
  rules/<plugin>/<theme>.ts   one file per theme, one constant per file
  rules/<plugin>/all.ts   list of the plugin's themes
  scopes/                 path overrides (core, shell, modules, tests, tooling, barrels)
  plugin/                 custom plugin `local/*`: rules, utilities, tests
```

| Folder | Themes |
| --- | --- |
| `rules/core/` | variables, functions, complexity, control-flow, operators, objects, classes, async, errors, modules, naming, comments, regex, legacy |
| `rules/typescript/` | type-safety, strictness, type-declarations, type-imports, signatures, promises, classes, extensions, naming |
| `rules/jsdoc/` | presence, content, types, layout |
| `rules/imports/` | resolution, dependencies, style, default-exports, module-systems |
| `rules/unicorn/` | arrays, collections, iteration, strings, numbers, control-flow, functions, objects, errors, promises, modules, dom, web, platform, naming, misc |
| `rules/functional/` | immutability, purity, paradigm |
| `rules/stylistic/` | spacing, comments, prettier-owned |
| `rules/unused/` | unused |
| `rules/local/` | kinds, modules, documentation |

| Zone (`scopes/`) | Files | Effect |
| --- | --- | --- |
| `language.ts` | `**/*.ts` | typed parser, plugins, resolver |
| `pure-layers.ts` | `src/math … src/io` | forbids the DOM and the host |
| `dom-layers.ts` | `src/render`, `src/interaction`, `playground` | relaxes the pure statement rules |
| `modules.ts` | `src/`, `playground/` | one export per file, kebab-case file named after it; files and domain folders kept apart (`local/folder-layout`); no default nor optional parameter and documented type members in `src/` |
| `playground.ts` | `playground/index.ts` | load-time effects allowed |
| `barrels.ts` | `src/**/index.ts`, `eslint/**/index.ts`, `playground/*/**/index.ts` (not the entry); `eslint/rules/*/all.ts` | barrels: `local/barrel-exports` and any number of imports; `all.ts` aggregators: any number of imports only |
| `tooling.ts` | `eslint/`, root `*.config.ts`, `*.setup.ts`, `*.test.ts`, `tests/`, `docs/.vitepress/`, `.claude/hooks/`, `scripts/` | Node.js, dev dependencies, no `@kind`; in hooks and scripts run by Node.js (`node-run`), imports name the `.ts` file and no barrel is asked |
| `tests.ts` | `**/*.test.ts` | literal numbers, free length, no `@kind` |
| `oxlint-owned.ts` | `**/*.ts`, last | turns off every rule `.oxlintrc.json` names and every dropped rule |

The `eslint/` folder itself keeps kebab-case files exporting one constant each; the one-export rule targets the library and the playground.

## Changing a rule

1. Find the theme: `grep -rn '"rule-name"' eslint/rules`.
2. Change the value **in that file only**; if it becomes `off`, write the reason as a comment above it.
3. A value taken from a table (`GLOBAL_FUNCTION_CEILING`, `PROCEDURE_VERBS`…) is changed in `eslint/settings/`.
4. A path-specific exception goes to `eslint/scopes/`, never to `rules/`.
5. `npm run check`.

After a plugin upgrade, `npm run test` lists the new rules to decide: add them to the right theme.

## One-off exceptions

`// oxlint-disable-next-line rule -- justification` for a rule oxlint runs, `// eslint-disable-next-line rule -- justification` for one ESLint still runs (ESLint ignores oxlint's directives and reports its own unused ones; oxlint reads both, so its report of unused directives waits until ESLint leaves) — the justification is mandatory (`unicorn/no-abusive-eslint-disable`). Each directive has its row in the [register of exceptions](./exceptions.md), in the same change (ADR-0066).

## Known autofix pitfall

`unicorn/prefer-import-meta-properties` replaces `fileURLToPath(new URL(".", import.meta.url))` (trailing slash) with `import.meta.dirname` (no trailing slash): concatenated paths break. Always build paths with `join()`.

## Custom rules (`local/*`)

Code: `eslint/plugin/rules/`, one test file per rule.

### `local/require-kind`

Every top-level function carries exactly one known `@kind`. ADR-0011.

### `local/kind-in-layer`

The `@kind` is allowed in the file's layer (`eslint/settings/layers.ts`).

### `local/kind-limits`

Lines, statements, complexity and depth bounded by `@kind` (`eslint/settings/kinds.ts`).

### `local/kind-naming`

Name fitting the kind: verb for `procedure`, conversion for `format`, no vague verb for `math`/`geometry`/`domain`, predicate prefix ⇔ boolean return.

### `local/see-references`

Every function has a `@see`; every `@see` points to an existing, **verified** reference (`REF-*`, `DERIV-*`, `ADR-NNNN`, `docs/…md`).

### `local/one-export-per-file`

One exported value per module, the file named in kebab-case after it; a type may sit next to it only if it is part of its signature. ADR-0019.

### `local/folder-has-index`

Every code folder holds an `index.ts`. ADR-0012.

### `local/folder-layout`

In `src/` and `playground/`: a folder holds modules or domain folders, never both (only the barrel sits beside folders); a folder past 15 modules is reported unless `eslint/settings/folders.ts` lists it, with its reason in [files.md](./files.md#folders-past-15-modules). ADR-0043.

### `local/barrel-exports`

An `index.ts` contains only `export * from "./x";`, one per module, sorted. Autofixed.

### `local/canonical-import-path`

Shortest legal import path: the file within the same folder, the folder otherwise, never `index` nor `.ts`. Autofixed.
