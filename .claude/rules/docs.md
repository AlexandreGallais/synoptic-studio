---
paths:
  - "**/*.md"
---

# Markdown pitfalls

How a page is written: `docs/conventions/writing.md`. What breaks a page in the site's build or the checks:

- A `|` inside a table cell, even in code, is escaped `\|`: it would end the cell.
- Never two curly braces in a row: VitePress evaluates them.
- A front-matter value holding `: ` is quoted (`tests/backlog.test.ts`).
- A folder is linked as `./folder/`, never `./folder/README.md`, and a local address (`http://localhost:5173/…`) or a template (`_*.md`, unpublished) is named in code font, never linked: each is a dead link in the docs build.
- A backlog status is changed by `npm run backlog:status`, which writes the item, its index and its parent's table together (`tests/backlog.test.ts` checks the three).
- A reference gets its row in `docs/references.md`, `[verified] YYYY-MM-DD`, before it is cited.
- Commit body lines are at most 100 characters.
