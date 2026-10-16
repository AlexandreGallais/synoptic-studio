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
    // Vitest with options only runs every test: the heavy lock is taken by `npm run test` (EN-047).
    pattern: /^npx\s+vitest(?:\s+run)?(?:\s+--?[\w-]+(?:=\S+)?)*\s*$/u,
    reason:
      "A full Vitest run takes the heavy lock: run `npm run test`, or name the test files (ADR-0063).",
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
    .split(/&&|\|\||[;|\n]/u)
    .map((segment) => segment.trim());

  return GUARDS.find((guard) => segments.some((segment) => guard.pattern.test(segment)))?.reason;
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
  const reason = deniedReason(commandOf(input));

  if (reason !== undefined) {
    const decision = {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: reason,
    };

    process.stdout.write(JSON.stringify({ hookSpecificOutput: decision }));
  }
}
