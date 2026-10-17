import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E17's checklist for F20, A context diet for every agent.
 *
 * Prepared by `val:prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: commands run in a terminal at the repository's root, or what
 * to look at during a run, each tried by VAL-021 on 2026-10-18. Criterion 5, the progress page,
 * was dropped on 2026-10-17, so it has no line.
 */
export const EPIC_17_F20_LINES: readonly ChecklistLine[] = [
  {
    feature: "F20",
    todo: "Terminal: npm run context:size -- ~/.claude/projects/<this project's folder>/<a session's id>/subagents",
    see: "one row per subagent, then the median first request per type: general-purpose about 38,900 tokens with CLAUDE.md, AGENTS.md and MEMORY.md loaded; story-worker about 10,500, auditor about 8,500, verifier about 7,100, none of them loaded",
  },
  {
    feature: "F20",
    todo: "Terminal: grep '^model:' .claude/agents/*.md",
    see: "clerk.md on sonnet, the seven others on opus, each file's body giving why; Decision to confirm (EN-066): the story worker stays on Opus, the Sonnet trial of CHK-021 being too small to compare",
  },
  {
    feature: "F20",
    todo: "Chat, during a run: the report of a story worker as the main thread receives it",
    see: "above the report, “check-handback: <story> on <branch> pushed at <hash>; next: npm run story:finish -- <feature branch>”; a worker whose branch is not pushed, or whose report leaves a commit unnamed, is refused once and told what is missing",
  },
  {
    feature: "F20",
    todo: "Claude Code's status line during a run, then after a compaction or in a new session of the same run",
    see: "“run <duration> · <tokens> tokens · week <n> % used”, counted from the run's start, subagents included, never restarting until the run ends",
  },
  {
    feature: "F20",
    todo: "Terminal: npm run metrics:session -- --run",
    see: "one row “| <duration>, open | <tokens> | <input> | <cache written> | <cache read> | <output> |”: the run's totals, the same as the status line's",
  },
  {
    feature: "F20",
    todo: "Terminal: npm run run:should-stop",
    see: "“go: week <n> % used, the limit is 90 %” and exit 0 below 90 %, “stop” and exit 1 from 90 %; then the parked stories, if any",
  },
  {
    feature: "F20",
    todo: "Read .claude/skills/run/SKILL.md, sections A question parks its story, Pushes and Stop when",
    see: "a question parks its story, written in its file, is pushed once and the run goes on; no push for a merge nor a stop; a run stops at its scope's end, on a blocker it cannot get around, when told, or at 90 % of the week",
  },
  {
    feature: "F20",
    todo: "Terminal, in a story's worktree: npm run git:sync",
    see: "“<id>: <branch> rebased on origin/feature/<…>, <n> commit(s) ahead: …” in about 1 s: the fetch and the rebase in one call",
  },
  {
    feature: "F20",
    todo: "Terminal, in a story's worktree: npm run git:commit -- -F <message file> <the files changed>",
    see: "“<id>: <hash> <subject> on <branch>”: the files named staged and committed in one call; with --push, pushed too; never develop pushed",
  },
  {
    feature: "F20",
    todo: "Ask the agent to run curl -s https://www.sqlite.org/fts5.html",
    see: 'refused: “The library holds this page: call library_get with url "https://www.sqlite.org/fts5.html", then library_read the sections needed”; the page then read from the library in about 0.1 s instead of 6 s',
  },
  {
    feature: "F20",
    todo: "Terminal: npm run library -- stats",
    see: "a table “### Projects”, one row per project that asked the library, “SVG editor” once F20 is on develop and the library's server restarted; the uses before it under “(not recorded)”",
  },
  {
    feature: "F20",
    todo: "Terminal: cat docs/conventions/*.md | wc -m, then npx vitest run tests/backlog.test.ts -t 'F20.AC8]'",
    see: "about 151,600 characters, 65,900 tokens at 2.30 characters a token, against 78,300 before EN-097; then 1 test passed: each of the four rewrites measured and its rules traced",
  },
];
