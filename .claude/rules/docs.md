---
paths:
  - "**/*.md"
---

# Markdown pitfalls

- Style: `docs/conventions/writing.md`. English only, except the glossary's French column.
- A `|` inside a table cell, even in code, must be escaped `\|`.
- Never write double curly braces: VitePress evaluates them.
- Front-matter values containing `: ` are quoted (`backlog.test.ts` checks it).
- Link folders as `./folder/`, not `./folder/README.md` (VitePress dead links). Templates start with `_` and are not published: never link to them (dead link in the docs build), name them in code font.
- Backlog: a status change is written in the item file, its folder index and its parent's table (`backlog.test.ts` checks all three).
- A reference is cited only after it was read: add its row to `docs/references.md` with `[verified] YYYY-MM-DD` first.
- Derivations use the commit scope `docs(geometry)`; body lines ≤ 100 characters.
- A local address (`http://localhost:5173/...`) goes in code font, never as a link: the docs build reports it as a dead link (US-018).
