# ADR-0013 — Prettier next to ESLint, not inside it

**Status**: Accepted

## Context

The Product Owner asked for Prettier, EditorConfig and ESLint working together, so that one command fixes everything.

## Decision

- **`eslint-config-prettier`** turns off the ESLint layout rules that conflict with Prettier.
- **No `eslint-plugin-prettier`**: Prettier's documentation advises against it (slower, formatting warnings in the editor, needless complexity) and recommends running Prettier separately (`REF-PRETTIER-LINTERS`).
- A single command remains: `npm run fix` = `prettier --write . && eslint --fix .`. Order matters: Prettier formats, ESLint adds what Prettier does not decide.
- **EditorConfig first**: `.editorconfig` holds `end_of_line`, `indent_style`, `indent_size`, `max_line_length` (read by Prettier, `REF-PRETTIER-CONFIG`); `.prettierrc.json` only holds options without an EditorConfig equivalent. No duplicate.
- ESLint adds the layout Prettier does not decide: blank lines between statements (`@stylistic/padding-line-between-statements`), comment style, import order, JSDoc layout.
- Only layout rule re-enabled after `eslint-config-prettier`: `curly: "all"`, compatible (documented "special rule").

## Consequences

- `npm run check` runs `prettier --check` first.
- The audit test checks that no other rule turned off by Prettier is re-enabled.

## References

`REF-PRETTIER-LINTERS`, `REF-PRETTIER-CONFIG`
