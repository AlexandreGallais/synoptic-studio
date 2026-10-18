import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E17's checklist for F22, One story at a time, the release by a button, its CI green.
 *
 * Prepared by `val-prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: commands run in a terminal at the repository's root, each
 * tried by VAL-023 on 2026-10-18, two lines left for the next review, where the CI of `develop` →
 * `main` and the button are first seen on GitHub, and the decisions of F22 to confirm.
 */
export const EPIC_17_F22_LINES: readonly ChecklistLine[] = [
  {
    feature: "F22",
    todo: 'Terminal: env -i PATH="$PATH" HOME="$HOME" GITHUB_ACTIONS=true npx vitest run tests/type-check-with-lint.test.ts -t F22.AC1',
    see: "1 passed: as on GitHub's runner and without the agent's variables, oxlint still prints the planted type error as planted.ts:2:9: error typescript(TS2322), since the test names --format agent; Decision to confirm (EN-116): the format named rather than picked from the environment",
  },
  {
    feature: "F22",
    todo: 'Terminal: CARGO_HOME="$(mktemp -d)" npx vitest run tests/rust-advisories.test.ts -t F22.AC1',
    see: "1 failed, as meant with no crate at hand: “cargo metadata failed (exit 101): error: no matching package named …”, Cargo's own message, no longer “Unexpected end of JSON input”; Decision to confirm (EN-116): the CI fetches every target's crates by cargo fetch --locked before check:all",
  },
  {
    feature: "F22",
    todo: "At the next review, on GitHub: the pull request develop → main, its Checks tab",
    see: "the CI green on ubuntu-latest, test and check:rust passed: [F19.AC6], [F19.AC9] and [F02.AC1], which failed 6 runs of 6 on #259, hold on the runner; pending until that review",
  },
  {
    feature: "F22",
    todo: "Terminal: node -p \"require('./package.json').version\", ls CHANGELOG.md, then grep -n 'if: github' .github/workflows/release.yml",
    see: "0.0.0-development; “No such file or directory”; then the release job only for workflow_dispatch on main, rust-cache and docs only for a push: a push to main deploys the site and makes no release",
  },
  {
    feature: "F22",
    todo: "Terminal: target/release/release notes > /tmp/notes.md, then git status",
    see: "57 lines grouped under ### Changed, ### Added, ### Removed and ### Fixed, each marked [draft] with its pull request, the commits behind each line on the terminal; the working tree clean; Decision to confirm (EN-117): release prepare is gone, the draft printed, no branch nor file made",
  },
  {
    feature: "F22",
    todo: "Terminal: printf '## 0.3.0\\n\\n### Fixed\\n\\n- Fix a thing\\n' > /tmp/bad.md, then target/release/release check /tmp/bad.md",
    see: "exit 1, line 1: “a release text holds no heading but its groups'”, line 5: “a change ends with its references, each a Markdown link”; Decision to confirm (EN-117): a body's release text ends at its first line ---, the evidence and the attribution after it left out of the notes",
  },
  {
    feature: "F22",
    todo: "Terminal: cargo test -q -p release f22_ac3_",
    see: "5 tests passed: tooling only since the last tag gives a patch with the notice of no change, no commit since it releases nothing, notes that disagree with the commits are refused; Decision to confirm (EN-117): only the library's and the playground's commits count, so a tooling commit marked ! is a patch",
  },
  {
    feature: "F22",
    todo: "Terminal: cargo test -q -p release dependabot",
    see: "1 test passed: a Dependabot pull request merged into main, its head branch dependabot/…, is left out of the notes; Decision to confirm (AUD-024): what such an update changes for a user is written in the next review's text",
  },
  {
    feature: "F22",
    todo: "Once develop → main is merged, on GitHub: Actions, Release, Run workflow on main; then press it again",
    see: "the release v0.3.0 on main's head, its notes the review's body up to its ---, the CI's test report attached, no commit on main; the second press releases nothing; pending until that review",
  },
  {
    feature: "F22",
    todo: 'Terminal: git log --first-parent --oneline origin/develop..origin/feature/f22-release-by-a-button --grep "Merge pull request", then grep -n -- --merge .github/workflows/story-automerge.yml',
    see: "five lines, #362 to #366, “Merge pull request #366 from …/docs/aud-024-audit-f22” first: each story of F22, then gh pr merge --auto --merge: every story into its feature branch by a merge commit, a feature or a REV into develop too",
  },
  {
    feature: "F22",
    todo: 'Terminal: grep -n "One agent at a time" AGENTS.md .claude/skills/run/SKILL.md, then cargo test -q -p worktree f22_ac4_',
    see: "a row of AGENTS.md and a rule of the run skill: one worker per story, never two stories, features or reviews at once; then 1 test passed: a story's worktree starts from its feature branch, a REV's from origin/develop",
  },
  {
    feature: "F22",
    todo: "Terminal: ls docs/backlog/_*.md, then npx vitest run tests/backlog.test.ts -t F22.AC5",
    see: "no template of an epic's closing, _review.md for the review at each return; then 4 tests passed: no epic asked for a closing nor holding one, at most 4 stories a feature and 3 tasks a story",
  },
];
