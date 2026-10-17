// PreToolUse hook (ADR-0021): denies shell commands the project forbids, whatever the instructions say.
// Input: the hook JSON on stdin. Output: a deny decision on stdout, or nothing to let the call proceed.

import { readFileSync } from "node:fs";

/** A forbidden command: what it matches and why it is denied. */
type Guard = {
  readonly pattern: RegExp;
  readonly reason: string;
};

/** `git` and its global options (`-C <path>`, `-c <name>=<value>`, `--no-pager`…), before a subcommand. */
const GIT = String.raw`^git(?:\s+(?:-[Cc]\s+(?:"[^"]*"|'[^']*'|\S+)|--[a-z-]+(?:=\S+)?))*\s+`;

/** Separators of a shell command's segments: `&&`, `||`, `;`, a pipe, a new line. */
const SEGMENTS = /&&|\|\||[;|\n]/u;

/** A folder inside a story's worktree (EN-052), its root captured. */
const IN_WORKTREE = /^(?<root>.*\/\.claude\/worktrees\/[^/]+)(?:\/|$)/u;

/** Vitest run by `npx` or from `node_modules/.bin`, its arguments captured. */
const VITEST = /^(?:npx\s+|(?:\.\/)?node_modules\/\.bin\/)vitest(?<rest>(?:\s.*)?)$/u;

/** Vitest's options that take a value as the next word, not a path of tests. */
const VITEST_VALUED = new Set([
  "--config",
  "--dir",
  "--environment",
  "--pool",
  "--project",
  "--reporter",
  "--root",
  "--shard",
  "--testNamePattern",
  "-c",
  "-r",
  "-t",
]);

/** The words a Vitest call takes for every test: none, or the current folder. */
const EVERY_TEST = new Set([".", "./"]);

/**
 * Whether a segment runs Vitest on every test, outside the heavy lock (EN-047): no test path, or
 * only the current folder, whatever its options (`--project x`, `--reporter dot`).
 *
 * @param segment - one segment of a shell command
 * @returns true when no word names a test file or folder other than `.`
 */
export function isFullVitestRun(segment: string): boolean {
  const rest = VITEST.exec(segment)?.groups?.["rest"];

  if (rest === undefined) {
    return false;
  }

  const words = rest.split(/\s+/u).filter((word) => word !== "" && word !== "run");
  const paths = words.filter(
    (word, index) =>
      !word.startsWith("-") && !VITEST_VALUED.has(words[index - 1] ?? "") && !EVERY_TEST.has(word),
  );

  return paths.length === 0;
}

/** Why a full Vitest run is denied. */
const FULL_VITEST =
  "A full Vitest run takes the heavy lock: run `npm run test`, or name the test files (ADR-0063).";

/** Commands denied in this repository, matched against each segment of a shell command. */
const GUARDS: readonly Guard[] = [
  {
    pattern: /^gh\s+pr\s+merge\b/u,
    reason:
      "Story pull requests are merged by the Product Owner, never by the agent (AGENTS.md, Never).",
  },
  {
    // `-n` is `git commit`'s short form of `--no-verify`, alone or among other short options.
    pattern: new RegExp(
      String.raw`${GIT}(?:commit\b.*\s(?:--no-verify\b|-[A-Za-z]*n[A-Za-z]*(?:\s|$))|push\b.*\s--no-verify\b)`,
      "u",
    ),
    reason:
      "The Git hooks (lint-staged, commitlint, check:all) are never bypassed: fix the cause instead.",
  },
  {
    // As a command's own setting, not as a word inside a quoted text or a file edit.
    pattern:
      /^(?:export\s+|env\s+)?(?:\w+=\S*\s+)*HUSKY=0\b|^git\s.*(?:-c\s+|config\b.*\s)core\.hooksPath\b/u,
    reason:
      "The Git hooks (lint-staged, commitlint, check:all) are never switched off: fix the cause instead.",
  },
  {
    // `+branch` in a refspec forces that one update.
    pattern: new RegExp(String.raw`${GIT}push\b(?=.*\s(?:--force|-f|\+\S+)(?:\s|$))`, "u"),
    reason: "Force pushes use --force-with-lease, and only on the agent's own story branch.",
  },
];

/**
 * Shell command without the bodies of its here-documents: their lines are data written to a file
 * or a pipe (a commit message, a test case), never run.
 *
 * @param command - shell command as the agent wrote it
 * @returns the command, each `<<WORD … WORD` body removed, the line opening it kept
 */
export function withoutHeredocs(command: string): string {
  return command.replaceAll(
    /(?<opening><<-?\s*(?<quote>['"]?)(?<word>\w+)\k<quote>[^\n]*)\n[\s\S]*?\n\s*\k<word>(?=\n|$)/gu,
    "$<opening>",
  );
}

/**
 * Reason for denying a shell command, if one of its segments is forbidden.
 *
 * @param command - shell command as the agent wrote it
 * @returns the reason of the first matching guard, or `undefined` when the command may run
 */
export function deniedReason(command: string): string | undefined {
  const segments = withoutHeredocs(command)
    .split(SEGMENTS)
    .map((segment) => segment.trim());

  const guarded = GUARDS.find((guard) =>
    segments.some((segment) => guard.pattern.test(segment)),
  )?.reason;

  // Vitest without a test path runs every test: the heavy lock is taken by `npm run test` (EN-047).
  return (
    guarded ?? (segments.some((segment) => isFullVitestRun(segment)) ? FULL_VITEST : undefined)
  );
}

/**
 * Reason for denying, in a worktree, a command Claude Code would refuse there: a Git command
 * chained with another, `cd` included (549 refusals in E15). The hint names the form that works.
 *
 * @param command - shell command as the agent wrote it
 * @param cwd - folder the command runs in
 * @returns a one-line hint, or `undefined` outside a worktree or for a command that fits
 */
export function worktreeReason(command: string, cwd: string): string | undefined {
  const root = IN_WORKTREE.exec(cwd)?.groups?.["root"];

  if (root === undefined) {
    return undefined;
  }

  const segments = withoutHeredocs(command)
    .split(SEGMENTS)
    .map((segment) => segment.trim())
    .filter((segment) => segment !== "");

  return segments.some((segment) => /^npm\s+run\s+golden:write\b/u.test(segment))
    ? "golden:write writes .git/metrics/golden.json, shared by every worktree: only the main thread runs it, in the main checkout."
    : chainedGitHint(segments, root);
}

/**
 * The hint for a Git command chained with another in a worktree: `cd` into the worktree first,
 * since a subagent's calls start in the main checkout, then the Git command alone (AUD-020).
 *
 * @param segments - the command's segments
 * @param root - the worktree's folder
 * @returns a one-line hint, or `undefined` when no Git command is chained
 */
function chainedGitHint(segments: readonly string[], root: string): string | undefined {
  const gitCommand = segments.find((segment) => /^git\s/u.test(segment));

  return gitCommand !== undefined && segments.length > 1
    ? `A worktree refuses a chained Git command: run \`cd "${root}" || exit 9\` then \`${gitCommand}\` alone.`
    : undefined;
}

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
export function field(value: unknown, key: string): unknown {
  if (typeof value !== "object" || value === null) {
    return undefined;
  }

  const property: unknown = Reflect.get(value, key);

  return property;
}

/**
 * Shell command carried by a hook input.
 *
 * @param input - parsed hook JSON
 * @returns `tool_input.command`, or an empty string when absent
 */
export function commandOf(input: unknown): string {
  const command = field(field(input, "tool_input"), "command");

  return typeof command === "string" ? command : "";
}

if (import.meta.main) {
  const input: unknown = JSON.parse(readFileSync(0, "utf8"));
  const cwd = field(input, "cwd");
  const reason =
    deniedReason(commandOf(input)) ??
    worktreeReason(commandOf(input), typeof cwd === "string" ? cwd : "");

  if (reason !== undefined) {
    const decision = {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: reason,
    };

    process.stdout.write(JSON.stringify({ hookSpecificOutput: decision }));
  }
}
