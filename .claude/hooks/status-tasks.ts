// Background tasks of a session, for the status line (docs/tooling/metrics.md): which tool calls
// start one — a subagent, a monitor, a call run in the background — and which transcript lines end
// one: its notification, or an error result when it was refused before it started.

import { blocksOf, field, startOf, text, textOr } from "./status-json.ts";

/** A background task the agent started (a subagent, a command in the background), still running. */
export type BackgroundTask = {
  /** The id of the tool call that started it. */
  readonly id: string;
  /** A subagent, or a command (a shell command in the background, a monitor). */
  readonly kind: "agent" | "command";
  /** What it does: a subagent's type and task, a command's description. */
  readonly label: string;
  /** When it started, epoch milliseconds. */
  readonly start: number;
};

/** Three hours: a background task older than this is taken as lost, its end never arrived (the session was stopped). */
export const LOST_TASK_MS = 10_800_000;

/**
 * Whether a tool call runs in the background: every subagent, every monitor (both end with a
 * notification), and any call asked to run in the background.
 *
 * @param block - a `tool_use` block
 * @returns true when its end comes later, as a notification
 */
function isBackground(block: unknown): boolean {
  const name = text(block, "name");

  return (
    name === "Agent" ||
    name === "Monitor" ||
    field(field(block, "input"), "run_in_background") === true
  );
}

/** Characters of a command kept as its label when it has no description. */
const LABEL_LENGTH = 60;

/**
 * The background task a tool call starts.
 *
 * @param block - a `tool_use` block running in the background
 * @param start - when it started, epoch milliseconds
 * @returns its id, kind, label and start
 */
function taskOf(block: unknown, start: number): BackgroundTask {
  const input = field(block, "input");
  const isAgent = text(block, "name") === "Agent";
  const label = isAgent
    ? `${textOr(input, "subagent_type", "agent")}: ${text(input, "description")}`
    : textOr(input, "description", text(input, "command").slice(0, LABEL_LENGTH));

  return { id: text(block, "id"), kind: isAgent ? "agent" : "command", label, start };
}

/**
 * Background tasks a transcript line starts.
 *
 * @param line - parsed transcript line
 * @returns each subagent, monitor or background call it launches, with its start
 */
export function startedTasks(line: unknown): readonly BackgroundTask[] {
  return blocksOf(field(line, "message"))
    .filter((block) => text(block, "type") === "tool_use" && isBackground(block))
    .map((block) => taskOf(block, startOf(line)));
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
