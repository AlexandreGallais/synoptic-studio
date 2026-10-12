// Exact tokens of a session for the status line (docs/tooling/metrics.md): summed from its
// transcript incrementally, a small cache in the Git directory keeping the byte offset already read.

import { existsSync, readFileSync, writeFileSync } from "node:fs";

/** Tokens summed so far over a transcript, and where the reading stopped. */
export type TokenTally = {
  readonly lastId: string;
  readonly offset: number;
  readonly output: number;
  readonly total: number;
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
 * Tally updated with new transcript lines; a request written over several consecutive lines is
 * counted once, by its message id.
 *
 * @param tally - tokens summed so far
 * @param lines - complete lines appended to the transcript since `tally.offset`
 * @returns the new tally, its offset moved past `lines`
 */
export function addTokens(tally: TokenTally, lines: string): TokenTally {
  return lines
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .map((line): unknown => field(JSON.parse(line), "message"))
    .filter((message) => field(message, "usage") !== undefined)
    .reduce<TokenTally>(
      (sum, message) => {
        const id = String(field(message, "id"));
        const usage = field(message, "usage");
        const isNew = id !== sum.lastId;
        const all = ["input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens"]
          .map((key) => count(usage, key))
          .reduce((left, right) => left + right, count(usage, "output_tokens"));

        return {
          lastId: id,
          offset: sum.offset,
          output: sum.output + (isNew ? count(usage, "output_tokens") : 0),
          total: sum.total + (isNew ? all : 0),
        };
      },
      { ...tally, offset: tally.offset + Buffer.byteLength(lines) },
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
  const lastId = field(tally, "lastId");

  return field(cached, "transcript") === transcript
    ? {
        lastId: typeof lastId === "string" ? lastId : "",
        offset: count(tally, "offset"),
        output: count(tally, "output"),
        total: count(tally, "total"),
      }
    : { lastId: "", offset: 0, output: 0, total: 0 };
}

/**
 * Tally of a transcript, read from where the cache stopped.
 *
 * @param transcript - path of the session's transcript
 * @param cacheFile - JSON file holding the tally of the previous call
 * @returns the updated tally, also written to the cache
 */
export function sessionTokens(transcript: string, cacheFile: string): TokenTally {
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
