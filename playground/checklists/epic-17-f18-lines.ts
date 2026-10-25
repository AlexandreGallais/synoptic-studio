import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E17's checklist for F18, A safe and cheap parallel run, and E15's leftovers.
 *
 * Prepared by `val:prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: mostly commands run in a terminal at the repository's root,
 * US-061 in the playground, and the four decisions to confirm.
 */
export const EPIC_17_F18_LINES: readonly ChecklistLine[] = [
  {
    feature: "F18",
    todo: "Terminal: npm run test in two terminals at once",
    see: "the second prints once “heavy lock held by process <pid> (npm run test in <folder>, for <n> s): waiting”, then runs when the first ends",
  },
  {
    feature: "F18",
    todo: "Chat: ask the agent to start four subagents at once",
    see: "three start; the fourth refused, “At most 3 subagents at once (ADR-0063): 3 running — <name> (<time>); …”",
  },
  {
    feature: "F18",
    todo: "Terminal: npm run worktree:new -- VAL-019, then npm run worktree:new -- US-61",
    see: "answered at once: “…/.claude/worktrees/val-019 already exists: remove it or name another story”, then “`US-61` is not a story id such as US-061”",
  },
  {
    feature: "F18",
    todo: "Terminal: npm run backlog:index, then git status",
    see: "no file changed: the stories index is exactly what the story files generate",
  },
  {
    feature: "F18",
    todo: "Decision to confirm (ADR-0068): git check-attr merge LOG-journal",
    see: "“merge: union”: two branches adding rows merge without conflict; the tables stay padded until F19's formatter",
  },
  {
    feature: "F18",
    todo: "Terminal: npm run metrics:session -- --all --markdown; during a run, the status line",
    see: "the time and tokens of the whole conversation, not of the last turn; a subagent in the background counted on the status line",
  },
  {
    feature: "F18",
    todo: "Terminal: npm run epic:check -- E17 2026-10-16 (it runs check:all, a few minutes)",
    see: "a table: the calls failed with “apart: <n> refused by the worktree isolation”, the calls lost at a force-quit, a failed check:all named by its tests",
  },
  {
    feature: "F18",
    todo: "Terminal: npm run mutate -- src/math/angle/central-angle.ts -- npx vitest run {test}",
    see: "three lines “caught …”, each a mutation the tests found, then “3 caught, 0 survived, 0 file(s) skipped”",
  },
  {
    feature: "F18",
    todo: "Chrome, Inter not installed: the Text tool, a click on the drawing, Installed fonts, allow; watch the counting line end",
    see: "the counting line ends with the faces refused, if any (“1 faces of the families read refused: 1 unreadable (…)”, US-062); the font list stays greyed, “Reading <family>, the default font: …”, then enabled, that family chosen (US-061)",
  },
  {
    feature: "F18",
    todo: "Decision to confirm (US-061): then open Weight and tick Embed fonts",
    see: "only the weights read again, none “its file not held”: a face refused at the second reading is dropped, a text asking it measured with the face held",
  },
  {
    feature: "F18",
    todo: "Decision to confirm (EN-067): npx vitest run src/io/font-face src/io/font-collection playground/fonts",
    see: "every test passed, none skipped: the collection, the 191 MB table and the second family are built from Inter's tables, on any system",
  },
  {
    feature: "F18",
    todo: "Decision to confirm (ADR-0067): at the release of 0.3, the pull requests on GitHub",
    see: "release-please's release pull request targets develop: merged first, then #259 develop → main; on main release-please only tags",
  },
];
