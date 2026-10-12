---
paths:
  - "eslint/**"
  - "*.config.ts"
  - "*.test.ts"
  - "package.json"
  - ".github/**"
  - ".claude/**"
---

# Tooling pitfalls

- **TypeScript is pinned to 6.0** (`typescript-eslint` does not support 7); **VitePress is on 2.0 alpha** (1.6 bundles a vulnerable Vite 5). Change either only with an ADR.
- Any new dev package: a row in `docs/tooling/dependencies.md` (tested), its release notes read, the latest version at least 7 days old, `npm audit` clean. A new GitHub Action or Node.js bump: `npm run deps:tools` checks them too.
- A plugin upgrade makes `eslint/config.test.ts` fail until its new rules are decided in `eslint/rules/`: intended.
- `unicorn/prefer-import-meta-properties` turns `new URL(".", import.meta.url)` into `import.meta.dirname` (no trailing slash): build paths with `join()`.
- A test that reads files is mutation-checked once: break its input on purpose and see it fail.
- A tooling change that makes existing files invalid is committed together with their fix.
- Hooks in `.claude/hooks/` and `scripts/` run with `node` (type stripping): relative imports name the `.ts` file (lint scope `node-run`), exported pure functions tested, side effects under `if (import.meta.main)`. Temporal needs `node --harmony-temporal` on Node.js 24.
- Stylelint is planned for SCSS but not installed (`braces` advisory GHSA-vfj7-8cjw-p6xm): see `docs/tooling/versions-and-security.md`.
- The `guard-bash` hook skips the bodies of here-documents (`<<'EOF' … EOF`), which are data; a guarded command written elsewhere on the line (inside `-m "…"`) is still read: write such text in a here-document.
- Delete a remote branch with `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`: `git push --delete` runs the pre-push hook (`check:all`, about 50 s) each time.
- Vite's `?raw` imports are typed by `vite/client` (tsconfig `types`); never add a `.d.ts` file for it.
- Commit scopes are a closed list (`commitlint.config.ts`): `docs/process/` changes use `docs(docs)`, not `docs(process)` — a refused commit leaves its files staged for the next one (F07).
- The shell is zsh: an unquoted `$ids` holding several lines is **one** word, so `for id in $ids` runs once with all of them (2026-10-13: a cleanup loop sent malformed requests for 7 minutes). Read lines with `while read -r id; do …; done <<< "$ids"` in a `bash` script, never hide the errors of a loop (`2>/dev/null`), and make every loop progress or stop: a round that changes nothing ends it and prints the error.
