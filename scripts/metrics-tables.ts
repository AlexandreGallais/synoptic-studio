// Statistics and Markdown tables of the metrics report (docs/tooling/metrics.md): commands grouped
// by what they run, counted and timed.

/** A timed tool call, as the tables read it. */
export type TimedCall = {
  readonly durationMs: number;
  readonly isOk: boolean;
  readonly summary: string;
  readonly tool: string;
};

/** Statistics of one group of calls. */
export type CallStats = {
  readonly count: number;
  readonly failures: number;
  readonly key: string;
  readonly maxMs: number;
  readonly medianMs: number;
  readonly totalMs: number;
};

/** Words naming a command and its subcommand: `git commit`, `npx vitest`, `npm install`. */
const TWO_WORDS = 2;

/** Words naming a command, a subcommand and its target: `npm run check`, `gh pr checks`. */
const THREE_WORDS = 3;

/** Commands named by more than their first word, and how many words name them. */
const COMMAND_WORDS = new Map([
  ["gh", THREE_WORDS],
  ["git", TWO_WORDS],
  ["node", TWO_WORDS],
  ["npm", THREE_WORDS],
  ["npx", TWO_WORDS],
]);

/** Milliseconds per second and per minute. */
const SECOND = 1000;

/** Seconds per minute. */
const MINUTE_SECONDS = 60;

/**
 * What a shell command runs, without its directory change, variables, flags and arguments.
 *
 * @param command - shell command as the agent wrote it
 * @returns e.g. `npm run check`, `git commit`, `gh pr checks`, `grep`
 */
export function commandKey(command: string): string {
  return (
    command
      .split(/&&|\|\||;|\n/u)
      .map((segment) => segmentKey(segment.trim()))
      .find((key) => key !== "") ?? ""
  );
}

/**
 * What one segment of a shell command runs.
 *
 * @param segment - text between `&&`, `;` or line breaks
 * @returns its command words, or an empty string for a directory change or variables only
 */
function segmentKey(segment: string): string {
  const words = segment.split(/\s+/u).filter((word) => /^[^-"'$][^=]*$/u.test(word));
  const first = words[0] ?? "";

  return first === "cd" ? "" : words.slice(0, wordCount(words)).join(" ");
}

/**
 * How many words name a command.
 *
 * @param words - the command's words, flags and variables left out
 * @returns 1, 2 or 3; `npm` takes three words only before `run`
 */
function wordCount(words: readonly string[]): number {
  const first = words[0] ?? "";

  return first === "npm" && words[1] !== "run" ? TWO_WORDS : (COMMAND_WORDS.get(first) ?? 1);
}

/**
 * Median of a list of numbers.
 *
 * @param values - numbers, in any order
 * @returns the middle value, the mean of the two middle values for an even count, 0 when empty
 */
export function median(values: readonly number[]): number {
  const sorted = values.toSorted((left, right) => left - right);
  const middle = Math.floor(sorted.length / 2);

  return sorted.length % 2 === 1
    ? (sorted[middle] ?? 0)
    : ((sorted[middle - 1] ?? 0) + (sorted[middle] ?? 0)) / 2;
}

/**
 * Duration written for a person.
 *
 * @param milliseconds - length of time to write
 * @returns `850 ms`, `12.4 s` or `3 min 05 s`
 */
export function formatDuration(milliseconds: number): string {
  const seconds = milliseconds / SECOND;

  if (seconds < 1) {
    return `${String(Math.round(milliseconds))} ms`;
  }

  if (seconds < MINUTE_SECONDS) {
    return `${seconds.toFixed(1)} s`;
  }

  const minutes = Math.floor(seconds / MINUTE_SECONDS);

  return `${String(minutes)} min ${String(Math.round(seconds % MINUTE_SECONDS)).padStart(2, "0")} s`;
}

/**
 * Statistics of calls grouped by a key.
 *
 * @param calls - timed calls
 * @param keyOf - group of a call
 * @returns one entry per group, in no particular order
 */
export function callStats(
  calls: readonly TimedCall[],
  keyOf: (call: TimedCall) => string,
): readonly CallStats[] {
  return [...Map.groupBy(calls, keyOf)].map(([key, group]) => {
    const durations = group.map((call) => call.durationMs);

    return {
      count: group.length,
      failures: group.filter((call) => !call.isOk).length,
      key,
      maxMs: Math.max(0, ...durations),
      medianMs: median(durations),
      totalMs: durations.reduce((sum, duration) => sum + duration, 0),
    };
  });
}

/**
 * One row of a Markdown table.
 *
 * @param cells - cell texts; `|` is escaped
 * @returns the row's line
 */
function tableRow(cells: readonly string[]): string {
  return `| ${cells.map((cell) => cell.replaceAll("|", String.raw`\|`)).join(" | ")} |`;
}

/**
 * Markdown table with a header row.
 *
 * @param headers - column titles
 * @param rows - cells, one list per row
 * @returns the table's lines joined
 */
export function markdownTable(headers: readonly string[], rows: readonly string[][]): string {
  return [headers, headers.map(() => "---"), ...rows].map((cells) => tableRow(cells)).join("\n");
}

/**
 * Table of call statistics.
 *
 * @param stats - statistics of each group
 * @returns a Markdown table: group, count, total, median, longest, failures
 */
export function statsTable(stats: readonly CallStats[]): string {
  return markdownTable(
    ["Command", "Count", "Total", "Median", "Longest", "Failed"],
    stats.map((entry) => [
      `\`${entry.key}\``,
      String(entry.count),
      formatDuration(entry.totalMs),
      formatDuration(entry.medianMs),
      formatDuration(entry.maxMs),
      String(entry.failures),
    ]),
  );
}
