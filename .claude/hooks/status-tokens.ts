// Exact tokens of the current turn for the status line (docs/tooling/metrics.md): from the person's
// prompt to the agent's last answer, like the session report. Summed from the transcript
// incrementally, a small cache in the Git directory keeping the byte offset already read.

import { existsSync, readFileSync, writeFileSync } from "node:fs";

/** A background task the agent started (a subagent, a command in the background), still running. */
type BackgroundTask = {
  /** The id of the tool call that started it. */
  readonly id: string;
  /** When it started, epoch milliseconds. */
  readonly start: number;
};

/** Tokens of the current turn, and where the reading of the transcript stopped. */
export type TokenTally = {
  /** When the agent ended its turn, epoch milliseconds; 0 while it works. */
  readonly end: number;
  /** Ids of the tool calls whose result or end arrived in the lines just read. */
  readonly finished: readonly string[];
  /** Whether the agent ended its turn: the next prompt starts a new one. */
  readonly isWaiting: boolean;
  readonly lastId: string;
  readonly offset: number;
  readonly output: number;
  /** Background tasks still running: the agent waits for them even after its turn ended. */
  readonly pending: readonly BackgroundTask[];
  /** When the turn started, epoch milliseconds; 0 before the first prompt. */
  readonly start: number;
  readonly total: number;
};

/** A tally before any line. */
export const EMPTY_TALLY: TokenTally = {
  end: 0,
  finished: [],
  isWaiting: true,
  lastId: "",
  offset: 0,
  output: 0,
  pending: [],
  start: 0,
  total: 0,
};

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
export function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * String property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a string, else an empty string
 */
export function text(value: unknown, key: string): string {
  const property = field(value, key);

  return typeof property === "string" ? property : "";
}

/**
 * Number property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a number, else 0
 */
export function count(value: unknown, key: string): number {
  const property = field(value, key);

  return typeof property === "number" ? property : 0;
}

/**
 * Whether a transcript line holds text, not only tool results.
 *
 * @param line - parsed transcript line
 * @returns true for a string content or a text block
 */
function hasText(line: unknown): boolean {
  const content = field(field(line, "message"), "content");

  return (
    typeof content === "string" ||
    (Array.isArray(content) && content.some((block) => text(block, "type") === "text"))
  );
}

/**
 * Whether a transcript line is a prompt typed by the person, not a tool result, a reminder, nor
 * the summary written after the conversation memory is compacted, nor a background task's
 * notification: they continue the turn.
 *
 * @param line - parsed transcript line
 * @returns true for a `user` line holding text
 */
function isPrompt(line: unknown): boolean {
  return (
    text(line, "type") === "user" &&
    field(line, "isMeta") !== true &&
    field(line, "isCompactSummary") !== true &&
    text(line, "promptSource") !== "system" &&
    hasText(line)
  );
}

/**
 * Tally after a request's usage; a request written over several consecutive lines is counted
 * once, by its message id.
 *
 * @param tally - tokens of the turn so far
 * @param message - message of a transcript line
 * @returns the tally with the request added
 */
function withUsage(tally: TokenTally, message: unknown): TokenTally {
  const usage = field(message, "usage");
  const id = String(field(message, "id"));

  if (usage === undefined || id === tally.lastId) {
    return tally;
  }

  const all = ["input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens"]
    .map((key) => count(usage, key))
    .reduce((left, right) => left + right, count(usage, "output_tokens"));

  return {
    ...tally,
    lastId: id,
    output: tally.output + count(usage, "output_tokens"),
    total: tally.total + all,
  };
}

/**
 * When a transcript line was written: a prompt sent, or the agent's last answer.
 *
 * @param line - parsed transcript line
 * @returns its timestamp, epoch milliseconds, or 0 when it has none
 */
function startOf(line: unknown): number {
  const time = text(line, "timestamp");

  return time === "" ? 0 : Temporal.Instant.from(time).epochMilliseconds;
}

/**
 * Tally once the agent works again after the end of its turn: a new turn at a prompt, the same
 * turn resumed at an answer that no prompt asked (a background task's notification woke it).
 *
 * @param tally - tally of a turn that ended
 * @param line - parsed transcript line
 * @returns the tally of the turn at work, or unchanged for any other line
 */
function resumed(tally: TokenTally, line: unknown): TokenTally {
  if (isPrompt(line)) {
    return { ...tally, end: 0, isWaiting: false, output: 0, start: startOf(line), total: 0 };
  }

  return text(line, "type") === "assistant" ? { ...tally, end: 0, isWaiting: false } : tally;
}

/** Three hours: a background task older than this is taken as lost, its end never arrived (the session was stopped). */
const LOST_TASK_MS = 10_800_000;

/**
 * Blocks of a message's content.
 *
 * @param message - message of a transcript line
 * @returns its blocks, none for a text content
 */
function blocksOf(message: unknown): readonly unknown[] {
  const content = field(message, "content");

  return Array.isArray(content) ? content : [];
}

/**
 * Background tasks a transcript line starts: every subagent (they always run in the background),
 * and any tool call asked to run in the background.
 *
 * @param line - parsed transcript line
 * @returns each subagent or background call it launches, with its start
 */
function startedTasks(line: unknown): readonly BackgroundTask[] {
  return blocksOf(field(line, "message"))
    .filter(
      (block) =>
        text(block, "type") === "tool_use" &&
        (text(block, "name") === "Agent" ||
          field(field(block, "input"), "run_in_background") === true),
    )
    .map((block) => ({ id: text(block, "id"), start: startOf(line) }));
}

/**
 * Ids of the tool calls a transcript line ends: a tool result, or a background task's notification
 * (`<task-notification>`, naming the call that started it).
 *
 * @param line - parsed transcript line
 * @param raw - the same line, as written
 * @returns the ids of the calls whose result or end it brings
 */
function endedIds(line: unknown, raw: string): readonly string[] {
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

/**
 * Background tasks after one transcript line: those it starts added, those notified as ended
 * removed, those older than `LOST_TASK_MS` dropped.
 *
 * @param tally - tally before the line
 * @param line - parsed transcript line
 * @param raw - the same line, as written
 * @returns the tally with its tasks and the calls ended
 */
function withTasks(tally: TokenTally, line: unknown, raw: string): TokenTally {
  const ended = endedIds(line, raw);
  const notified = raw.includes("<task-notification>") ? ended : [];
  const time = startOf(line);
  const pending = [...tally.pending, ...startedTasks(line)].filter(
    (task) => !notified.includes(task.id) && (time === 0 || time - task.start < LOST_TASK_MS),
  );

  return { ...tally, finished: [...tally.finished, ...ended], pending };
}

/**
 * Tally after one transcript line: a prompt after the agent ended its turn starts a new turn;
 * messages sent while the agent works stay inside the turn; an answer after the end of the turn
 * without a prompt (the agent woken by a background task's notification) resumes that turn.
 *
 * @param tally - tokens of the turn so far
 * @param line - parsed transcript line
 * @param raw - the same line, as written
 * @returns the new tally
 */
function withLine(tally: TokenTally, line: unknown, raw: string): TokenTally {
  const message = field(line, "message");
  const started = tally.isWaiting ? resumed(tally, line) : tally;
  const counted = withUsage(withTasks(started, line, raw), message);

  return text(message, "stop_reason") === "end_turn"
    ? { ...counted, end: startOf(line), isWaiting: true }
    : counted;
}

/**
 * Tally updated with new transcript lines.
 *
 * @param tally - tokens of the turn so far
 * @param lines - complete lines appended to the transcript since `tally.offset`
 * @returns the new tally, its offset moved past `lines`
 */
export function addTokens(tally: TokenTally, lines: string): TokenTally {
  return lines
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .reduce<TokenTally>(
      (sum, raw) => {
        const line: unknown = JSON.parse(raw);

        return withLine(sum, line, raw);
      },
      {
        ...tally,
        finished: [],
        offset: tally.offset + Buffer.byteLength(lines),
      },
    );
}

/**
 * Tally kept by the previous call, when it was for the same transcript.
 *
 * @param cached - parsed cache file
 * @param transcript - path of the session's transcript
 * @returns that tally, or an empty one from the start of the file
 */
export function cachedTally(cached: unknown, transcript: string): TokenTally {
  const tally = field(cached, "tally");

  const pending = field(tally, "pending");

  return field(cached, "transcript") === transcript
    ? {
        end: count(tally, "end"),
        finished: [],
        isWaiting: field(tally, "isWaiting") !== false,
        lastId: text(tally, "lastId"),
        offset: count(tally, "offset"),
        output: count(tally, "output"),
        pending: (Array.isArray(pending) ? pending : []).map((task) => ({
          id: text(task, "id"),
          start: count(task, "start"),
        })),
        start: count(tally, "start"),
        total: count(tally, "total"),
      }
    : EMPTY_TALLY;
}

/**
 * Tally of a transcript, read from where the cache stopped.
 *
 * @param transcript - path of the session's transcript
 * @param cacheFile - JSON file holding the tally of the previous call
 * @returns the updated tally, also written to the cache
 */
export function turnTokens(transcript: string, cacheFile: string): TokenTally {
  const cached: unknown = existsSync(cacheFile) ? JSON.parse(readFileSync(cacheFile, "utf8")) : {};
  const start = cachedTally(cached, transcript);
  const content = existsSync(transcript)
    ? readFileSync(transcript).subarray(start.offset)
    : Buffer.alloc(0);
  const complete = content.subarray(0, content.lastIndexOf("\n") + 1).toString("utf8");
  const tally = addTokens(start, complete);

  writeFileSync(cacheFile, JSON.stringify({ tally, transcript }));

  return tally;
}
