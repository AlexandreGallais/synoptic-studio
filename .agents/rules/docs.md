---
paths:
  - "**/*.md"
---

# Markdown

The docs are the Product Owner's, written only at a review; working facts go into the records. How a page is written: the records' `conventions`, area `writing`, and `word_list`. What breaks a page in the site's build or the checks:

- A `|` inside a table cell, even in code, is escaped `\|`.
- Never two curly braces in a row: VitePress evaluates them.
- A front-matter value holding `: ` is quoted.
- A folder is linked as `./folder/`, never `./folder/README.md`; a local address (`http://localhost:5173/…`) or an unpublished template (`_*.md`) is named in code font, never linked.
- A heading a link targets has no `'`, `/`, `_`, `.` nor `—`, and no leading digit, so that it keeps one anchor on GitHub and on the site (`local/anchor-slugs`).
- A source is cited by its `REF-*` of the records, never a path into them.
