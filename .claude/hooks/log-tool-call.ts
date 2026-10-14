// PostToolUse and PostToolUseFailure hook (RET-001 metrics, docs/tooling/metrics.md): appends one
// line per tool call — what ran, who ran it, when it started and ended, the size of what it read
// and returned, whether it failed, and the session's state at that moment as the status line last
// saw it (tokens of the turn, conversation memory, background tasks, step) — to the metrics log,
// a JSON Lines file inside the Git directory, shared by every worktree and never committed. Runs
// in the background (`async`): it never slows the agent down. `npm run metrics` reads it.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the hook command passes it.

import { execFileSync } from "node:child_process";
import { appendFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { isAbsolute, join, relative } from "node:path";

import { count, field, text, textOr } from "./status-json.ts";

/** Longest summary kept for one call: a long heredoc is cut, its length is kept. */
const SUMMARY_LENGTH = 300;

/** Input keys naming what a tool acted on, in order of preference. */
const SUBJECT_KEYS = ["command", "file_path", "notebook_path", "pattern", "url", "query", "skill"];

/** One tool call, as written to the metrics log. */
export type ToolCallRecord = SessionMoment & {
  readonly agent: string;
  /** The subagent's own id, empty for the main agent. */
  readonly agentId: string;
  readonly branch: string;
  readonly description: string;
  readonly durationMs: number | undefined;
  readonly error: string;
  /** Characters of the call's input: what the agent wrote to make it. */
  readonly inputChars: number;
  /** Whether it runs in the background: a subagent, a monitor, a command sent there. */
  readonly isBackground: boolean;
  readonly isOk: boolean;
  /** Characters of the call's result: what the agent reads back. */
  readonly outputChars: number;
  readonly permissionMode: string;
  readonly session: string;
  /** When it started, epoch milliseconds, from the `track-tool-call` hook; 0 when unknown. */
  readonly startedAt: number;
  /** What the agent said it was doing (`.git/metrics/step`). */
  readonly step: string;
  readonly summary: string;
  /** When it ended, ISO 8601. */
  readonly time: string;
  readonly tool: string;
  readonly toolUseId: string;
};

/** The session's state when a call ends, as the status line last wrote it (`now.json`). */
export type SessionMoment = {
  readonly contextPercent: number;
  readonly model: string;
  readonly pendingAgents: number;
  readonly pendingCommands: number;
  /** Age of that state when the call ended, milliseconds. */
  readonly snapshotAgeMs: number;
  /** Time since the turn started, milliseconds. */
  readonly turnElapsedMs: number;
  readonly turnOutputTokens: number;
  readonly turnStart: number;
  readonly turnTokens: number;
};

/** Where and when a call ends, read by the hook beside its input. */
export type CallContext = {
  readonly branch: string;
  /** Content of `now.json`, written by the status line. */
  readonly snapshot: unknown;
  readonly startedAt: number;
  readonly step: string;
  /** When the hook runs, epoch milliseconds. */
  readonly time: number;
};

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
 * Characters of a JSON value written out.
 *
 * @param value - parsed JSON, possibly missing
 * @returns the length of its JSON text, 0 when missing
 */
function sizeOf(value: unknown): number {
  return value === undefined ? 0 : JSON.stringify(value).length;
}

/**
 * The session's state when a call ends, from the status line's last snapshot.
 *
 * @param snapshot - content of `now.json`, possibly empty
 * @param time - when the call ended, epoch milliseconds
 * @returns tokens of the turn, memory, background tasks, all 0 when the snapshot is missing
 */
export function sessionMoment(snapshot: unknown, time: number): SessionMoment {
  const turnStart = count(snapshot, "turnStart");
  const taken = count(snapshot, "time");

  return {
    contextPercent: count(snapshot, "contextPercent"),
    model: text(snapshot, "model"),
    pendingAgents: count(snapshot, "pendingAgents"),
    pendingCommands: count(snapshot, "pendingCommands"),
    snapshotAgeMs: taken === 0 ? 0 : time - taken,
    turnElapsedMs: turnStart === 0 ? 0 : time - turnStart,
    turnOutputTokens: count(snapshot, "turnOutputTokens"),
    turnStart,
    turnTokens: count(snapshot, "turnTokens"),
  };
}

/**
 * Whether a call runs in the background: a subagent, a monitor, or a call sent there.
 *
 * @param tool - `Bash`, `Agent`, `Monitor`…
 * @param toolInput - its `tool_input`, where `run_in_background` is read
 * @returns true when its end comes as a notification
 */
function isBackgroundCall(tool: string, toolInput: unknown): boolean {
  return tool === "Agent" || tool === "Monitor" || field(toolInput, "run_in_background") === true;
}

/**
 * First line of a failed call's error.
 *
 * @param input - parsed hook JSON
 * @returns that line, empty for a call that succeeded
 */
function errorLine(input: unknown): string {
  return text(input, "hook_event_name") === "PostToolUseFailure"
    ? (text(input, "error").split("\n", 1)[0] ?? "")
    : "";
}

/**
 * Record of one tool call, from the hook input.
 *
 * @param input - parsed hook JSON of `PostToolUse` or `PostToolUseFailure`
 * @param context - branch, step, start, time and the status line's snapshot
 * @returns the record to append to the metrics log
 */
export function toolCallRecord(input: unknown, context: CallContext): ToolCallRecord {
  const tool = text(input, "tool_name");
  const toolInput = field(input, "tool_input");
  const duration = field(input, "duration_ms");

  return {
    ...sessionMoment(context.snapshot, context.time),
    agent: textOr(input, "agent_type", "main"),
    agentId: text(input, "agent_id"),
    branch: context.branch,
    description: text(toolInput, "description"),
    durationMs: typeof duration === "number" ? duration : undefined,
    error: errorLine(input),
    inputChars: sizeOf(toolInput),
    isBackground: isBackgroundCall(tool, toolInput),
    isOk: text(input, "hook_event_name") !== "PostToolUseFailure",
    outputChars: sizeOf(field(input, "tool_response")),
    permissionMode: text(input, "permission_mode"),
    session: text(input, "session_id"),
    startedAt: context.startedAt,
    step: context.step,
    summary: toolSummary(tool, toolInput, text(input, "cwd")),
    time: Temporal.Instant.fromEpochMilliseconds(context.time).toString(),
    tool,
    toolUseId: text(input, "tool_use_id"),
  };
}

/**
 * Parsed JSON of a file, or `undefined` when it is missing or unreadable.
 *
 * @param path - file to read
 * @returns its content
 */
function jsonOf(path: string): unknown {
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return undefined;
  }
}

/**
 * The step the agent wrote, empty when there is none.
 *
 * @param path - `.git/metrics/step`
 * @returns its first line
 */
function readStep(path: string): string {
  const content = existsSync(path) ? readFileSync(path, "utf8") : "";

  return content.split("\n", 1)[0] ?? "";
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
    const running = join(folder, "running", `${text(input, "tool_use_id")}.json`);
    const record = toolCallRecord(input, {
      branch: git(cwd, ["branch", "--show-current"]),
      snapshot: jsonOf(join(folder, "now.json")),
      startedAt: count(jsonOf(running), "start"),
      step: readStep(join(folder, "step")),
      time: Temporal.Now.instant().epochMilliseconds,
    });

    mkdirSync(folder, { recursive: true });
    appendFileSync(join(folder, "tool-calls.jsonl"), `${JSON.stringify(record)}\n`);
    // The call has ended: the status line no longer shows it (track-tool-call hook).
    rmSync(join(folder, "running", `${record.toolUseId}.json`), { force: true });
  }
}
