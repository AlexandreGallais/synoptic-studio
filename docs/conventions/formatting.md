# Formatting

Layout is written by tools, never chosen by hand nor discussed in a review. Each tool owns one part of it, so that no two tools fight over a line (ADR-0075):

| Tool | Decides | File |
| --- | --- | --- |
| EditorConfig | charset, line ending, indentation, line length (100), trailing spaces, final newline | `.editorconfig` |
| the formatter (`tools/format`) | the whole layout of every file kind it knows: oxfmt, then Markdown tables without padding (ADR-0068); the order of imports (`sortImports`, [imports](./imports.md#order)) | `.oxfmtrc.json` (`printWidth` 100) |
| the linters | what the formatter leaves: blank lines between statements, comments, import members, JSDoc layout, braces | `.oxlintrc.json`, `tools/lint` |

The reasons for each setting, measured on this repository's edits, are in [rules](./rules.md#the-formatters-settings).

## The formatter

One tool formats every file: `tools/format`, run with `--write` by `npm run fix` and with `--check`, which changes nothing, by `npm run format:check`. It runs oxfmt on its targets, then rewrites each Markdown table without padding.

| File kind | Formatted by |
| --- | --- |
| TypeScript, JavaScript (`.cjs`), JSON, YAML, TOML, CSS, Markdown | oxfmt (`REF-OXFMT-LANGUAGES`) |
| HTML (`playground/index.html`) | oxfmt, through the Prettier it bundles; no Prettier package in `package.json` |
| Markdown tables | oxfmt pads them, then `tools/format` removes the padding: each cell trimmed, one space around each `\|` |
| Rust | `cargo fmt` (`check:rust`) |
| SVG, XML, shell hooks, `LICENSE`, dotfiles | nothing |

- **Tables without padding**: a wider cell changes its own row only, so parallel branches do not conflict over a table's widths (ADR-0068).
- **Prose stays on its lines**: a paragraph or a list item stays on the lines it was written on (`proseWrap: "preserve"`), so a reflow never touches lines nobody edited.
- **Targets are named**: the folders and root files listed in `package.json`, never `.`, which would walk into the worktrees of `.claude/worktrees/` and the build outputs (EN-045). `.oxfmtrc.json`'s `ignorePatterns` holds plain paths only.

Run `npm run fix` before each commit: it runs the formatter, then oxlint and the project's linter with `--fix`. After it, `npm run format:check` finds no layout issue. At the commit, the pre-commit hook runs the same tools on the staged files, and the secret scan on each of them ([hooks](../tooling/git-workflow.md#hooks-githooks)).

## Blank lines (`@stylistic/padding-line-between-statements`)

Code reads in paragraphs: declarations, then actions, then the result. The autofix writes the blank lines:

- after a group of `const` or `let`, but not between two `const`;
- before a declaration that follows a call;
- around blocks (`if`, `for`, functions…) and multi-line expressions;
- before `return` and before `export`;
- after the imports.

```ts
const pathData = polygonToPathData(polygon);
const svg = createSvgElement(document, viewBox);

svg.append(createPathElement(document, pathData));
```

## Comments

- Write a comment above its line, never at its end (`no-inline-comments`): it is read before the code it explains.
- Write no `TODO` or `FIXME` (`no-warning-comments`): open work goes to the backlog, where it is found again.

## Format on save

- **WebStorm and IntelliJ** (`.idea/`, in the repository): the Oxc plugin from the JetBrains Marketplace formats with oxfmt and shows oxlint's findings. The editor alone pads tables; the pre-commit hook unpads them at the commit.
- **VS Code** (`.vscode/`): the recommended extensions are `oxc.oxc-vscode`, `editorconfig.editorconfig` and TypeScript 7's `TypeScriptTeam.native-preview`; `formatOnSave`, then `source.fixAll.oxc`; `js/ts.experimental.useTsgo` on, and `js/ts.tsdk.path` set to the installed TypeScript 7, `./node_modules/typescript` (`REF-TS7-VSCODE`, `tests/typescript-7.test.ts`, EN-121).
- On save, unused imports disappear and mixed imports split into `import` and `import type`. Import paths and barrels are fixed by `npm run fix` (`local/canonical-import-path`, `local/barrel-exports`).
