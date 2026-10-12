// PostToolUse and PostToolUseFailure hook (RET-001 metrics, docs/tooling/metrics.md): appends one
// line per tool call — what ran, how long it took, whether it failed — to the metrics log, a JSON
// Lines file inside the Git directory, shared by every worktree and never committed. Runs in the
// background (`async`): it never slows the agent down. `npm run metrics` reads it.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the hook command passes it.

import { execFileSync } from "node:child_process";
import { appendFileSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { isAbsolute, join, relative } from "node:path";

/** Longest summary kept for one call: a long heredoc is cut, its length is kept. */
const SUMMARY_LENGTH = 300;

/** Input keys naming what a tool acted on, in order of preference. */
const SUBJECT_KEYS = ["command", "file_path", "notebook_path", "pattern", "url", "query", "skill"];

/** One tool call, as written to the metrics log. */
export type ToolCallRecord = {
  readonly agent: string;
  readonly branch: string;
  readonly description: string;
  readonly durationMs: number | undefined;
  readonly error: string;
  readonly isOk: boolean;
  readonly session: string;
  readonly summary: string;
  readonly time: string;
  readonly tool: string;
  readonly toolUseId: string;
};

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * String property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a string, else an empty string
 */
function text(value: unknown, key: string): string {
  const property = field(value, key);

  return typeof property === "string" ? property : "";
}

/**
 * What a tool call acted on: its command, file, pattern, address, query or skill.
 *
 * @param tool - tool name (`Bash`, `Read`, `Agent`…)
 * @param input - the call's `tool_input`
 * @param cwd - working directory of the session, to shorten file paths
 * @returns the first non-empty one, a file path relative to `cwd`; for a subagent, its type and task
 */
function subject(tool: string, input: unknown, cwd: string): string {
  if (tool === "Agent") {
    return `${text(input, "subagent_type")}: ${text(input, "description")}`;
  }

  const value = SUBJECT_KEYS.map((key) => text(input, key)).find((found) => found !== "") ?? "";

  return cwd !== "" && isAbsolute(value) ? relative(cwd, value) : value;
}

/**
 * Short summary of what a tool call did, on one line.
 *
 * @param tool - tool name (`Bash`, `Read`, `Agent`…)
 * @param input - the call's `tool_input`
 * @param cwd - working directory of the session, to shorten file paths
 * @returns at most `SUMMARY_LENGTH` characters, followed by the full length when cut
 */
export function toolSummary(tool: string, input: unknown, cwd: string): string {
  const summary = subject(tool, input, cwd).replaceAll(/\s+/gu, " ");

  return summary.length > SUMMARY_LENGTH
    ? `${summary.slice(0, SUMMARY_LENGTH)}… (${String(summary.length)} characters)`
    : summary;
}

/**
 * Record of one tool call, from the hook input.
 *
 * @param input - parsed hook JSON of `PostToolUse` or `PostToolUseFailure`
 * @param time - when the hook ran, ISO 8601
 * @param branch - Git branch checked out in the session's working directory
 * @returns the record to append to the metrics log
 */
export function toolCallRecord(input: unknown, time: string, branch: string): ToolCallRecord {
  const tool = text(input, "tool_name");
  const toolInput = field(input, "tool_input");
  const duration = field(input, "duration_ms");
  const isFailure = text(input, "hook_event_name") === "PostToolUseFailure";
  const agent = text(input, "agent_type");

  return {
    agent: agent === "" ? "main" : agent,
    branch,
    description: text(toolInput, "description"),
    durationMs: typeof duration === "number" ? duration : undefined,
    error: isFailure ? (text(input, "error").split("\n", 1)[0] ?? "") : "",
    isOk: !isFailure,
    session: text(input, "session_id"),
    summary: toolSummary(tool, toolInput, text(input, "cwd")),
    time,
    tool,
    toolUseId: text(input, "tool_use_id"),
  };
}

/**
 * Output of a Git command run in a directory.
 *
 * @param cwd - directory to run it in
 * @param gitArguments - what follows `git`, e.g. `branch --show-current`
 * @returns its trimmed standard output, or an empty string when Git fails
 */
function git(cwd: string, gitArguments: readonly string[]): string {
  try {
    return execFileSync("git", gitArguments, { cwd, encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}

if (import.meta.main) {
  const input: unknown = JSON.parse(readFileSync(0, "utf8"));
  const sessionFolder = text(input, "cwd");
  const cwd = sessionFolder === "" ? process.cwd() : sessionFolder;
  const common = git(cwd, ["rev-parse", "--path-format=absolute", "--git-common-dir"]);

  if (common !== "") {
    const folder = join(common, "metrics");
    const time = Temporal.Now.instant().toString();
    const record = toolCallRecord(input, time, git(cwd, ["branch", "--show-current"]));

    mkdirSync(folder, { recursive: true });
    appendFileSync(join(folder, "tool-calls.jsonl"), `${JSON.stringify(record)}\n`);
    // The call has ended: the status line no longer shows it (track-tool-call hook).
    rmSync(join(folder, "running", `${record.toolUseId}.json`), { force: true });
  }
}
