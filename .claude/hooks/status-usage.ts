// The subscription's usage that a run reads to stop in time (`.git/metrics/usage.json`, EN-092):
// the weekly and five-hour windows of `rate_limits` in the status line's input
// (`REF-CC-STATUSLINE`).

import { field } from "./status-json.ts";

/** One window of the subscription's rate limit: the share used and when it resets. */
export type UsageWindow = {
  /** Share of the window used, in percent, from 0 to 100. */
  readonly usedPercent: number;
  /** When the window resets, an ISO 8601 instant. */
  readonly resetsAt: string;
};

/** What `usage.json` holds. */
export type UsageSnapshot = {
  /** The rolling five-hour window, absent when Claude Code did not send it. */
  readonly fiveHour?: UsageWindow;
  /** The weekly window, absent when Claude Code did not send it. */
  readonly sevenDay?: UsageWindow;
  /** When it was read, epoch milliseconds. */
  readonly time: number;
};

/** Milliseconds per second. */
const SECOND = 1000;

/**
 * One window of `rate_limits`.
 *
 * @param limits - the input's `rate_limits` object
 * @param key - `five_hour` or `seven_day`
 * @returns the window, or `undefined` when Claude Code did not send it
 */
function usageWindow(limits: unknown, key: string): UsageWindow | undefined {
  const window = field(limits, key);
  const used = field(window, "used_percentage");
  const resets = field(window, "resets_at");

  return typeof used === "number" && typeof resets === "number"
    ? {
        resetsAt: Temporal.Instant.fromEpochMilliseconds(resets * SECOND).toString(),
        usedPercent: used,
      }
    : undefined;
}

/**
 * The subscription's usage, sent by Claude Code to Pro and Max subscribers after the session's
 * first response.
 *
 * @param input - the status line's JSON input
 * @param now - time now, epoch milliseconds
 * @returns the snapshot to write, or `undefined` when no window was sent (the last one is kept)
 */
export function usageSnapshot(input: unknown, now: number): UsageSnapshot | undefined {
  const limits = field(input, "rate_limits");
  const fiveHour = usageWindow(limits, "five_hour");
  const sevenDay = usageWindow(limits, "seven_day");

  if (fiveHour === undefined && sevenDay === undefined) {
    return undefined;
  }

  return {
    ...(fiveHour !== undefined && { fiveHour }),
    ...(sevenDay !== undefined && { sevenDay }),
    time: now,
  };
}
