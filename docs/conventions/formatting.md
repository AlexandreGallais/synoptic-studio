# Formatting

Three tools, each with its own scope, **no duplicate** (ADR-0013):

| Tool         | Decides                                                                                                     | File                                                                                       |
| ------------ | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| EditorConfig | charset, line ending, indentation, line length (100), trailing spaces, final newline                        | `.editorconfig`                                                                            |
| Prettier     | the whole code layout (also reads `.editorconfig`)                                                          | `.prettierrc.json` (options without an EditorConfig equivalent)                            |
| ESLint       | what Prettier does not decide: blank lines between statements, comments, import order, JSDoc layout, braces | `eslint/rules/stylistic/`, `eslint/rules/imports/style.ts`, `eslint/rules/jsdoc/layout.ts` |

## One command

`npm run fix` = `prettier --write .` then `eslint --fix .`. After it, `npm run check` reports no layout issue. On commit, `lint-staged` (`.lintstagedrc.json`) runs Prettier and ESLint `--fix` on staged TypeScript and Markdown, Prettier on JSON, HTML, CSS and YAML, and secretlint on every staged file.

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

- **WebStorm / IntelliJ** (`.idea/`, in the repository): _Prettier_ → "Run on save" (automatic configuration); _ESLint_ → automatic configuration + "Run eslint --fix on save".
- **VS Code** (`.vscode/`): recommended extensions `esbenp.prettier-vscode`, `dbaeumer.vscode-eslint`, `editorconfig.editorconfig`; `formatOnSave` then `source.fixAll.eslint`.
- On save: unused imports disappear (`unused-imports/no-unused-imports`), mixed imports split into `import` + `import type`, import paths and barrels are fixed (`local/canonical-import-path`, `local/barrel-exports`).
