# Lint

Three linters run one after the other until F19 ends (`npm run lint`): **oxlint** first, with its native rules and `tsgolint`'s type-aware ones (`.oxlintrc.json`, written by `@oxlint/migrate` from `eslint/`, EN-074), then **the project's linter** (`tools/lint`, Rust, EN-075), with the project's own rules and those oxlint lacks, then **ESLint**, for the rules not yet moved, which F19 rewrites in the project's linter before ESLint leaves (EN-085). Markdown pages are the project's linter's alone since EN-077: ESLint reads no page. Each rule runs in one linter only: the last entry of ESLint's configuration (`eslint/scopes/oxlint-owned.ts`) turns off every rule `.oxlintrc.json` names and every rule the triage dropped (`eslint/settings/dropped-rules.ts`; the reasons in research note 0023), and `tests/oxlint-config.test.ts` checks it, with the scopes carried as `overrides`; a rule moved to the project's linter is `off` in its theme, with a comment naming `tools/lint`.

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

| Zone (`scopes/`) | Files | Effect |
| --- | --- | --- |
| `language.ts` | `**/*.ts` | typed parser, plugins, resolver |
| `pure-layers.ts` | `src/math … src/io` | forbids the DOM and the host |
| `dom-layers.ts` | `src/render`, `src/interaction`, `playground` | relaxes the pure statement rules |
| `modules.ts` | `src/` | documented type members |
| `playground.ts` | `playground/index.ts` | load-time effects allowed |
| `barrels.ts` | `src/**/index.ts`, `eslint/**/index.ts`, `playground/*/**/index.ts` (not the entry); `eslint/rules/*/all.ts` | barrels: any number of imports; `all.ts` aggregators: any number of imports only |
| `tooling.ts` | `eslint/`, root `*.config.ts`, `*.setup.ts`, `*.test.ts`, `tests/`, `docs/.vitepress/`, `.claude/hooks/`, `scripts/` | Node.js, dev dependencies |
| `tests.ts` | `**/*.test.ts` | literal numbers, free length |
| `oxlint-owned.ts` | `**/*.ts`, last | turns off every rule `.oxlintrc.json` names and every dropped rule |

The `eslint/` folder itself keeps kebab-case files exporting one constant each; the one-export rule targets the library and the playground.

## Changing a rule

1. Find the theme: `grep -rn '"rule-name"' eslint/rules`.
2. Change the value **in that file only**; if it becomes `off`, write the reason as a comment above it.
3. A value taken from a table is changed in `eslint/settings/` (`GLOBAL_FUNCTION_CEILING`…) or, for the project's linter, in `tools/lint/src/settings.rs` (`PROCEDURE_VERBS`, `KIND_LIMITS`…).
4. A path-specific exception goes to `eslint/scopes/`, never to `rules/`.
5. `npm run check`.

After a plugin upgrade, `npm run test` lists the new rules to decide: add them to the right theme.

## One-off exceptions

`// oxlint-disable-next-line rule -- justification` for a rule oxlint runs, `// lint-disable-next-line rule -- justification` for one the project's linter runs (it reports a directive that silences nothing or gives no justification), `// eslint-disable-next-line rule -- justification` for one ESLint still runs (ESLint ignores oxlint's directives and reports its own unused ones; oxlint reads both, so its report of unused directives waits until ESLint leaves) — the justification is mandatory (`unicorn/no-abusive-eslint-disable`). Each directive has its row in the [register of exceptions](./exceptions.md), in the same change (ADR-0066).

## Known autofix pitfall

`unicorn/prefer-import-meta-properties` replaces `fileURLToPath(new URL(".", import.meta.url))` (trailing slash) with `import.meta.dirname` (no trailing slash): concatenated paths break. Always build paths with `join()`.

## The project's linter (`tools/lint`)

A crate of the Rust workspace on Oxc's crates (`oxc_parser`, `oxc_ast`, `oxc_ast_visit`, `oxc_semantic`, `oxc_allocator`; EN-075, research note 0023): `cargo run --release -q -p lint -- [--fix] <targets>`, on oxlint's targets. It prints each finding as `file:line:column rule message`, then a count, and fails on any.

- **Each file once**: read, parsed with its comments, its semantic model built, then every rule its scopes turn on runs over that one AST — a rule reads the whole file (`check_file`) or each node of the linter's single walk (`check_node`); the files are spread over the machine's threads, and what the rules read beside the file (folder listings, the bibliography, the ADRs) is read once per run.
- **One rule, one module** of `tools/lint/src/rules/`, its tests beside it, from the cases of the rule it carries and of its documentation; registered in `rules::ALL`. Adding a rule: the module, its line in `ALL`, its name in `ON_EVERYWHERE` or a scope of `tools/lint/src/scopes.rs`, its section below, its rule `off` in ESLint.
- **Markdown pages** (EN-077): each page of the targets but `CHANGELOG.md` is parsed once by the crate `markdown` (`markdown-rs`, `REF-MARKDOWN-RS`) into an mdast tree, the one `@eslint/markdown` reads, with YAML front matter and GFM; the Markdown rules of `tools/lint/src/markdown/rules/` read that tree and the text under it.
- **Scopes**: `tools/lint/src/scopes.rs` carries, in ESLint's order, the scopes of `eslint/scopes/` that set its rules; the project's values (kinds, layers, verbs, reference sources, the folder signal) are in `tools/lint/src/settings.rs`.
- **Measured** on 2026-10-17: 0.05 to 0.06 s for the 1,535 files of `npm run lint`'s targets, all its rules on (0.11 s of CPU over the machine's threads).

### `no-restricted-syntax`

No `enum`, no `for…in`, no getter or setter, no `export * as name`, no dynamic import; in `src/`, no default nor optional parameter (ADR-0016). The spread of an accumulator in a fold is oxlint's `oxc/no-accumulating-spread` (EN-061, EN-075).

### `require-atomic-updates`

In an `async` function or a generator, no assignment to a variable, or to a property through a variable, read before an `await` or a `yield` and not since (ESLint's rule, `allowProperties: false`).

### `functional/no-return-void`

A function of the functional core returns a value: no `void`, `undefined` or `null` return type; without a written type, the body is read (no type checker).

### `@typescript-eslint/naming-convention`

Each name's casing by its role, with the project's options ([naming](./naming.md)), sorted and matched as typescript-eslint does: the first option that applies is the only one checked. Its `types` (a module constant holding a function, a boolean's prefix) are read from the written annotation, or else the initializer: a call's result is not read, and such a name takes the next option (EN-076, `REF-TSESLINT-NAMING-CONVENTION`).

### `unicorn/no-non-function-verb-prefix`

A variable or a parameter named `getX`, `setX`, `unsetX`, `deleteX`, `addX`, `removeX`, `destroyX` or `createX` holds a function. Whether it can be called is read from its written type or its initializer; an unknown value is not reported, as the rule reported nothing without types (EN-076, `REF-UNICORN-NON-FUNCTION-VERB-PREFIX`).

### `functional/immutable-data`

In the functional core, no existing object or array is changed: no assignment, `delete`, `++` or `--` on a member; no `copyWithin`, `fill`, `pop`, `push`, `reverse`, `shift`, `sort`, `splice`, `unshift`, `add`, `clear`, `delete` or `set` called by name, whatever the object; no `Object.assign`, `defineProperty`, `defineProperties` or `setPrototypeOf` on an existing object.

### `functional/no-mixed-types`

A type holds only data or only functions; a member is a function when it is a method, a call or construct signature, or a property whose written type is a function type.

### `local/require-kind`

Every top-level function carries exactly one known `@kind`. ADR-0011.

### `local/kind-in-layer`

The `@kind` is allowed in the file's layer (`tools/lint/src/settings.rs`).

### `local/kind-limits`

Lines, statements, complexity and depth bounded by `@kind` (`tools/lint/src/settings.rs`).

### `local/kind-naming`

Name fitting the kind: verb for `procedure`, conversion for `format`, no vague verb for `math`/`geometry`/`domain`, predicate prefix ⇔ boolean return.

### `local/see-references`

Every function has a `@see`; every `@see` points to an existing, **verified** reference (`REF-*`, `DERIV-*`, `ADR-NNNN`, `docs/…md`).

### `local/one-export-per-file`

One exported value per module, the file named in kebab-case after it; a type may sit next to it only if it is part of its signature. ADR-0019.

### `local/folder-has-index`

Every code folder holds an `index.ts`. ADR-0012.

### `local/folder-layout`

In `src/` and `playground/`: a folder holds modules or domain folders, never both (only the barrel sits beside folders); a folder past 15 modules is reported unless `tools/lint/src/settings.rs` lists it, with its reason in [files.md](./files.md#folders-past-15-modules). ADR-0043.

### `local/barrel-exports`

An `index.ts` contains only `export * from "./x";`, one per module, sorted. Autofixed.

### `local/canonical-import-path`

Shortest legal import path: the file within the same folder, the folder otherwise, never `index` nor `.ts`. Autofixed.

### Markdown rules

On every Markdown page but `CHANGELOG.md` (EN-077), with `@eslint/markdown`'s names, options and messages (`REF-ESLINT-MARKDOWN-RULES`), so that a finding reads as it did in ESLint. Autofixed: `no-bare-urls`, `no-missing-atx-heading-space`, `no-reversed-media-syntax`, `no-space-in-emphasis`.

| Rule | A page… |
| --- | --- |
| `markdown/fenced-code-language` | names the language of each fenced code block |
| `markdown/fenced-code-meta` | writes no metadata after a fence's language (`never`) |
| `markdown/heading-increment` | goes down one heading level at a time; front matter holds no title |
| `markdown/no-bare-urls` | writes a URL as `<URL>` or `[text](URL)` |
| `markdown/no-duplicate-definitions` | defines a link label or a footnote once (`[//]: #` comments apart) |
| `markdown/no-duplicate-headings` | gives distinct titles to the headings under one parent (`checkSiblingsOnly`) |
| `markdown/no-empty-definitions` | gives each definition a URL other than `#`, each footnote content |
| `markdown/no-empty-images` | gives each image a source other than `#` |
| `markdown/no-empty-links` | gives each link a destination other than `#` |
| `markdown/no-html` | holds no HTML element but `<span>` (`<span v-pre>` shows double curly braces on the site) |
| `markdown/no-invalid-label-refs` | writes no reference with a blank label, `[text][ ]` |
| `markdown/no-missing-atx-heading-space` | puts a space after a heading's hashes |
| `markdown/no-missing-label-refs` | defines each reference it uses; GitHub's alerts and `[verified]`, `[unverified]` apart |
| `markdown/no-missing-link-fragments` | links `#fragment` only to a heading (GitHub's anchor), a `{#id}`, an HTML `id` or `name`, `#top` or `#L12` |
| `markdown/no-multiple-h1` | has one first-level title, Markdown or `<h1>` |
| `markdown/no-reversed-media-syntax` | writes `[label](URL)`, never `(label)[URL]` |
| `markdown/no-space-in-emphasis` | puts no space inside emphasis markers, `** bold **` |
| `markdown/no-unused-definitions` | uses each definition and footnote it holds |
| `markdown/require-alt-text` | gives each image a text alternative; `alt=""` or `aria-hidden="true"` for a decorative one |
| `markdown/table-column-count` | gives each table row as many cells as its header (`checkMissingCells`) |
| `local/image-alt-text` | keeps a text alternative under 155 characters, never opening with "Image of" ([writing.md](./writing.md), `REF-GOOGLE-IMAGES`) |

`markdown/no-reference-like-urls` is dropped: the pages link inline, reference definitions appear on one copied page (`.github/CODE_OF_CONDUCT.md`), and a link whose URL is a label is a dead link, which VitePress's build refuses in `docs/`. None of the 21 rules is the formatter's: oxfmt leaves `#Title`, `* text *`, a reversed link, a bare URL and an extra table cell as they are (checked on 2026-10-17).
