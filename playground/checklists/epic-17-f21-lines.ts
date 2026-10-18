import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E17's checklist for F21, Agent tools and the library server in Rust.
 *
 * Prepared by `val:prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: commands run in a terminal at the repository's root or in a
 * story's worktree, each tried by VAL-022 on 2026-10-18, and the five decisions of F21 to
 * confirm.
 */
export const EPIC_17_F21_LINES: readonly ChecklistLine[] = [
  {
    feature: "F21",
    todo: 'Terminal: ls .claude/hooks, then grep -o \'"command": "[^"]*\' .claude/settings.json',
    see: "“No such file or directory”, then each hook naming target/release/<binary> under the project's folder: guard-bash, limit-agents, library-pages, check-handback, track-tool-call, session-state, log-tool-call; the last line, cut at its quote, is the status line's, which runs target/release/status-line; Decision to confirm (ADR-0078): the hooks, the Git hooks and the agent call each binary by its path, about 3 ms a hook instead of 47 ms through cargo run, npm run kept for people",
  },
  {
    feature: "F21",
    todo: 'Ask the agent, in a story\'s worktree, to run cd "<worktree>" || exit 9 then git log --oneline -3 | head -3, then the same with git status; ls',
    see: "the first goes through, three commits listed; the second is refused: “A worktree refuses a chained Git command”, with the form to run; Decision to confirm (EN-104): one Git command piped into read-only filters (head, tail, grep, wc, cut, sort, uniq, sed -n, awk) goes through, a second command never",
  },
  {
    feature: "F21",
    todo: "Terminal: npm run metrics, then its section Tools",
    see: "one row per tool and subcommand with its calls, failures, median and slowest: guard-bash about 61,000 calls, 0 failed, 0 ms median; no row for run-totals status; Decision to confirm (EN-102): the status line's poll, once a second during a run, is not recorded, about 18,000 lines a run of 5 hours saved",
  },
  {
    feature: "F21",
    todo: "Terminal: ls scripts, then grep -n '\"node ' package.json",
    see: "default-font-metrics.ts and its test only, then one line, font:metrics: every other script and hook is a Rust binary of tools/; that one runs the library itself",
  },
  {
    feature: "F21",
    todo: "Terminal: npm run -s library -- search fts5",
    see: "the sections of the stored pages that hold the word, SQLite's FTS5 page first, read by the Rust command line from the store the rmcp server uses, ~/.local/share/agent-library/library.sqlite, in about 5 ms by its path; Decision to confirm (ADR-0077): the store is named by AGENT_LIBRARY_FILE when set, never LIBRARY_PATH, which Apple's python3 sets for the C toolchain",
  },
  {
    feature: "F21",
    todo: "Terminal: node -p \"Object.keys(require('./package.json').devDependencies).length\", then npm run deps:unused",
    see: "13 dev packages, against 38 at E17's start, each kept with its reason in F21's table; then nothing printed and exit 0: no unused file, export nor package",
  },
  {
    feature: "F21",
    todo: "Terminal: printf 'Fixed the thing\\n' > /tmp/msg, then target/release/git-hooks commit-msg /tmp/msg",
    see: "“✖ subject may not be empty [subject-empty]”, “✖ type may not be empty [type-empty]”, “found 2 problems, 0 warnings” and exit 1: commitlint's checks, run by our own binary from .githooks/; a commit of one Markdown file takes about 356 ms with its hooks instead of 716 ms",
  },
  {
    feature: "F21",
    todo: "Terminal: time npm run -s lint:secrets",
    see: "nothing printed, exit 0, in about 0.27 s through npm and 0.11 s by its path, against 0.97 s for secretlint: every tracked file scanned for secrets by secretlint's 27 rules, ported",
  },
  {
    feature: "F21",
    todo: "Terminal: git ls-files | grep -ciE 'husky|lint-staged|commitlint|knip|release-please|npm-check-updates|validate-branch-name'",
    see: "0: no file of the rewritten packages left; ESLint's and secretlint's names stay only where EN-113 kept them with their reason (the rules accounted for, the preset's cases)",
  },
  {
    feature: "F21",
    todo: "Terminal, in a fresh clone of develop once F21 is merged: npm install, then npm run release:prepare, then target/release/release check",
    see: "“v0.2.0 → 0.3.0 on release/0.3.0: 57 line(s) drafted in CHANGELOG.md”, then each [draft] line named until it is rewritten; Decision to confirm (ADR-0079): the version and the changelog are one commit on a release branch from develop, its pull request into main merged by you, the tag and the GitHub release made by our tool, no commit after the merge",
  },
  {
    feature: "F21",
    todo: "Terminal: npm run -s page -- REF-CC-HOOKS",
    see: "the table's header and the one row of REF-CC-HOOKS, about 2,600 characters of the 212,000 of docs/references.md, nothing else",
  },
  {
    feature: "F21",
    todo: "Terminal: npm run check:all, then cargo test -q -p repo-tools f21_ac9_",
    see: "every check ✓ in about 75 s with nothing changed since the last run, against 170 to 217 s at E17's start; then the tests passed: a lane that writes [core], [user], [commit], [url], [include], [includeIf], [extensions] or a remote's URL into .git/config fails naming its lines",
  },
];
