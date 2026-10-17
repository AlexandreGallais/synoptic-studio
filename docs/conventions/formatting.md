# Formatting

Three tools, each with its own scope, **no duplicate** (ADR-0013; Prettier replaced by oxfmt in EN-073):

| Tool | Decides | File |
| --- | --- | --- |
| EditorConfig | charset, line ending, indentation, line length (100), trailing spaces, final newline | `.editorconfig` |
| The formatter (`tools/format`) | the whole layout of every file kind it knows: oxfmt, then Markdown tables without padding (ADR-0068, EN-073) | `.oxfmtrc.json` (Prettier's options, translated by `oxfmt --migrate=prettier`; `printWidth` 100) |
| ESLint | what the formatter does not decide: blank lines between statements, comments, import order, JSDoc layout, braces | `eslint/rules/stylistic/`, `eslint/rules/imports/style.ts`, `eslint/rules/jsdoc/layout.ts` |

## One formatter

One command formats every file the repository formats: `cargo run --release -q -p format -- --write|--check <targets…>`, run by `npm run format`, `format:check`, `fix`, `backlog:index` and `lint-staged`. It runs oxfmt 0.72.0 on the targets, then writes each Markdown table without padding.

| File kind | Formatted by |
| --- | --- |
| TypeScript, JavaScript (`.cjs`), JSON, YAML, TOML (`Cargo.toml`), CSS, Markdown | oxfmt, natively in Rust (`REF-OXFMT-LANGUAGES`) |
| HTML (`playground/index.html`) | oxfmt, through the Prettier it bundles (`REF-OXFMT-LANGUAGES`); no Prettier package in `package.json` |
| Markdown tables | oxfmt pads them, with no option against it; `tools/format` then removes the padding: each cell trimmed, one space around each `\|`, delimiter cells `---` with their colons |
| Rust | `cargo fmt` (`check:rust`) |
| SVG, XML, shell hooks, `LICENSE`, dotfiles | nothing, as before: neither Prettier nor oxfmt reads them |

- **Tables without padding**: a wider cell changes its own row only, so parallel branches no longer conflict on a table's widths (ADR-0068). Paragraphs and list items stay on the lines they were written on (`proseWrap: "preserve"`).
- **`--check`** changes nothing: oxfmt checks every file but Markdown; each Markdown file is formatted in a temporary copy, its tables unpadded, then compared with the file.
- **Targets**: the folders and root files named in `package.json` (EN-045: never `.`); `*` stands for the root's files, which the tool lists itself, since oxfmt reads `*` at any depth. Files that `.oxfmtrc.json`'s `ignorePatterns` name (plain paths only) are skipped by both steps.

`npm run fix` = the formatter, then oxlint, the project's linter and ESLint with `--fix`. After it, `npm run check` reports no layout issue. On commit, `lint-staged` (`.lintstagedrc.json`) runs the formatter and the three linters with `--fix` on staged TypeScript, the formatter and the project's linter with `--fix` on staged Markdown, the formatter on the other kinds above, and secretlint on every staged file.

## Blank lines (`@stylistic/padding-line-between-statements`)

Code reads in paragraphs: declarations, then actions, then the result.

- blank line **after** a group of `const` / `let` (not between two `const`);
- blank line **before** a `const` following a call;
- blank line **around** blocks (`if`, `for`, functions…) and multi-line expressions;
- blank line **before** `return` and before `export`;
- blank line after the imports.

```ts
const pathData = polygonToPathData(polygon);
const svg = createSvgElement(document, viewBox);

svg.append(createPathElement(document, pathData));
```

## Comments

- Above the line, never at the end of it (`no-inline-comments`, `@stylistic/line-comment-position`).
- No `TODO` / `FIXME` (`no-warning-comments`): ongoing work goes to `docs/`.

## Format on save

- **WebStorm / IntelliJ** (`.idea/`, in the repository): the Oxc plugin of the JetBrains Marketplace formats with oxfmt; _ESLint_ → automatic configuration + "Run eslint --fix on save". An editor formats with oxfmt alone: the tables it pads are unpadded at the commit by `lint-staged`.
- **VS Code** (`.vscode/`): recommended extensions `oxc.oxc-vscode`, `dbaeumer.vscode-eslint`, `editorconfig.editorconfig`; `formatOnSave` then `source.fixAll.eslint`.
- On save: unused imports disappear (`unused-imports/no-unused-imports`), mixed imports split into `import` + `import type`, import paths and barrels are fixed (`local/canonical-import-path`, `local/barrel-exports`).
