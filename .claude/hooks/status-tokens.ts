// Exact tokens of the current turn for the status line (docs/tooling/metrics.md): from the person's
// prompt to the agent's last answer, like the session report. Summed from the transcript
// incrementally, a small cache in the Git directory keeping the byte offset already read.

import { existsSync, readFileSync, writeFileSync } from "node:fs";

/** Tokens of the current turn, and where the reading of the transcript stopped. */
export type TokenTally = {
  /** Whether the agent ended its turn: the next prompt starts a new one. */
  readonly isWaiting: boolean;
  readonly lastId: string;
  readonly offset: number;
  readonly output: number;
  /** When the turn started, epoch milliseconds; 0 before the first prompt. */
  readonly start: number;
  readonly total: number;
};

/** A tally before any line. */
export const EMPTY_TALLY: TokenTally = {
  isWaiting: true,
  lastId: "",
  offset: 0,
  output: 0,
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
 * the summary written after the conversation memory is compacted (it continues the turn).
 *
 * @param line - parsed transcript line
 * @returns true for a `user` line holding text
 */
function isPrompt(line: unknown): boolean {
  return (
    text(line, "type") === "user" &&
    field(line, "isMeta") !== true &&
    field(line, "isCompactSummary") !== true &&
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
 * When a prompt was sent.
 *
 * @param line - parsed transcript line of a prompt
 * @returns its timestamp, epoch milliseconds, or 0 when it has none
 */
function startOf(line: unknown): number {
  const time = text(line, "timestamp");

  return time === "" ? 0 : Temporal.Instant.from(time).epochMilliseconds;
}

/**
 * Tally after one transcript line: a prompt after the agent ended its turn starts a new turn;
 * messages sent while the agent works stay inside the turn.
 *
 * @param tally - tokens of the turn so far
 * @param line - parsed transcript line
 * @returns the new tally
 */
function withLine(tally: TokenTally, line: unknown): TokenTally {
  const message = field(line, "message");
  const started =
    tally.isWaiting && isPrompt(line)
      ? { ...tally, isWaiting: false, output: 0, start: startOf(line), total: 0 }
      : tally;
  const counted = withUsage(started, message);

  return text(message, "stop_reason") === "end_turn" ? { ...counted, isWaiting: true } : counted;
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
    .map((line): unknown => JSON.parse(line))
    .reduce<TokenTally>((sum, line) => withLine(sum, line), {
      ...tally,
      offset: tally.offset + Buffer.byteLength(lines),
    });
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

  return field(cached, "transcript") === transcript
    ? {
        isWaiting: field(tally, "isWaiting") !== false,
        lastId: text(tally, "lastId"),
        offset: count(tally, "offset"),
        output: count(tally, "output"),
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
