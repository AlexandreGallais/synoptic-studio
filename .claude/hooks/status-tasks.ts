// Background tasks of a session, for the status line (docs/tooling/metrics.md): which tool calls
// start one — a subagent, a monitor, a call run in the background, a message that resumes a
// subagent — and which transcript lines end one: its notification, an error result when it was
// refused before it started, or a subagent's result when it ran in the foreground.

import { blocksOf, field, startOf, text, textOr } from "./status-json.ts";

/** A background task the agent started (a subagent, a command in the background), still running. */
export type BackgroundTask = {
  /** The id of the tool call that started it. */
  readonly id: string;

  /**
   * A subagent, a command (a shell command in the background, a monitor), or a message to a
   * subagent whose result is still to come: it becomes a subagent if that result resumes one.
   */
  readonly kind: "agent" | "command" | "message";
  /** What it does: a subagent's type and task, a command's description, a message's summary. */
  readonly label: string;
  /** When it started, epoch milliseconds. */
  readonly start: number;
};

/** Three hours: a background task older than this is taken as lost, its end never arrived (the session was stopped). */
export const LOST_TASK_MS = 10_800_000;

/**
 * Whether a tool call may run in the background: every subagent, every monitor (both end with a
 * notification), any call asked to run in the background, and a message to a subagent, which
 * resumes it in the background when it had stopped.
 *
 * @param block - a `tool_use` block
 * @returns true when its end may come later, as a notification
 */
function isBackground(block: unknown): boolean {
  const name = text(block, "name");

  return (
    name === "Agent" ||
    name === "Monitor" ||
    name === "SendMessage" ||
    field(field(block, "input"), "run_in_background") === true
  );
}

/** Characters of a command kept as its label when it has no description. */
const LABEL_LENGTH = 60;

/**
 * What a background task does.
 *
 * @param name - the tool's name
 * @param input - the tool call's input
 * @returns a subagent's type and task, a message's summary, a command's description
 */
function labelOf(name: string, input: unknown): string {
  if (name === "Agent") {
    return `${textOr(input, "subagent_type", "agent")}: ${text(input, "description")}`;
  }

  return name === "SendMessage"
    ? `resumed: ${textOr(input, "summary", text(input, "to"))}`
    : textOr(input, "description", text(input, "command").slice(0, LABEL_LENGTH));
}

/**
 * The background task a tool call starts.
 *
 * @param block - a `tool_use` block running in the background
 * @param start - when it started, epoch milliseconds
 * @returns its id, kind, label and start
 */
function taskOf(block: unknown, start: number): BackgroundTask {
  const name = text(block, "name");
  const kinds: Readonly<Record<string, BackgroundTask["kind"]>> = {
    Agent: "agent",
    SendMessage: "message",
  };

  return {
    id: text(block, "id"),
    kind: kinds[name] ?? "command",
    label: labelOf(name, field(block, "input")),
    start,
  };
}

/**
 * Background tasks a transcript line starts.
 *
 * @param line - parsed transcript line
 * @returns each subagent, monitor, background call or message it launches, with its start
 */
export function startedTasks(line: unknown): readonly BackgroundTask[] {
  return blocksOf(field(line, "message"))
    .filter((block) => text(block, "type") === "tool_use" && isBackground(block))
    .map((block) => taskOf(block, startOf(line)));
}

/**
 * Ids of the messages a transcript line answers by resuming a subagent: a subagent that had
 * stopped runs again in the background, and its end is notified under the message's call id
 * (seen 2026-10-15: "Resuming agent …", then the notification 15 minutes later). A message to a
 * subagent still running is only queued: its result resumes nothing.
 *
 * @param line - parsed transcript line
 * @returns the ids of the calls whose result names a resumed subagent
 */
export function resumedIds(line: unknown): readonly string[] {
  return blocksOf(field(line, "message"))
    .filter(
      (block) =>
        text(block, "type") === "tool_result" &&
        JSON.stringify(field(block, "content") ?? "").includes("resumedAgentId"),
    )
    .map((block) => text(block, "tool_use_id"));
}

/**
 * Whether a tool result launches a subagent in the background, as every `Agent` result of the
 * transcripts did up to 2026-10-17: `toolUseResult` holds `status: "async_launched"` and
 * `isAsync: true`, and the text opens with "Async agent launched successfully" (one line of
 * 2026-10-16 holds the text without `toolUseResult`).
 *
 * @param line - parsed transcript line
 * @param block - one of its `tool_result` blocks
 * @returns true when the subagent runs on, its end to be notified
 */
function isLaunchedInBackground(line: unknown, block: unknown): boolean {
  const result = field(line, "toolUseResult");

  return (
    text(result, "status") === "async_launched" ||
    field(result, "isAsync") === true ||
    JSON.stringify(field(block, "content") ?? "").includes("Async agent launched")
  );
}

/**
 * Ids of the calls a transcript line answers with a result that is neither an error nor a launch
 * in the background: a subagent run in the foreground ends with its result (`status:
 * "completed"`, the hooks reference), never with a notification (AUD-020).
 *
 * @param line - parsed transcript line
 * @returns the ids of the calls it completes
 */
export function completedIds(line: unknown): readonly string[] {
  return blocksOf(field(line, "message"))
    .filter(
      (block) =>
        text(block, "type") === "tool_result" &&
        field(block, "is_error") !== true &&
        !isLaunchedInBackground(line, block),
    )
    .map((block) => text(block, "tool_use_id"));
}

/**
 * Ids of the calls a transcript line answers with an error: a call refused or failed before it
 * started (permission denied, the safety check unavailable, an invalid input) never sends the
 * notification of a background task's end.
 *
 * @param line - parsed transcript line
 * @returns the ids of the calls whose result is an error
 */
export function failedIds(line: unknown): readonly string[] {
  return blocksOf(field(line, "message"))
    .filter((block) => text(block, "type") === "tool_result" && field(block, "is_error") === true)
    .map((block) => text(block, "tool_use_id"));
}

/**
 * Ids of the tool calls a transcript line ends: a tool result, or a background task's notification
 * (`<task-notification>`, naming the call that started it).
 *
 * @param line - parsed transcript line
 * @param raw - the same line, as written
 * @returns the ids of the calls whose result or end it brings
 */
export function endedIds(line: unknown, raw: string): readonly string[] {
  const results = blocksOf(field(line, "message"))
    .filter((block) => text(block, "type") === "tool_result")
    .map((block) => text(block, "tool_use_id"));
  const notified = raw.includes("<task-notification>")
    ? raw
        .matchAll(/<tool-use-id>(?<id>[\w-]+)<\/tool-use-id>/gu)
        .map((match) => match.groups?.["id"] ?? "")
        .toArray()
    : [];

  return [...results, ...notified];
}
