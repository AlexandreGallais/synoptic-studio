// Where a turn's time goes (RET-002, docs/tooling/metrics.md): for the main agent's calls whose
// start the `track-tool-call` hook recorded, the time the tools ran, and the time between one
// call's end and the next one's start — the model reading, thinking and writing the next call —
// with the characters it wrote. Calls logged before RET-002 have no start and are left out.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { count, field, text } from "../.claude/hooks/status-json.ts";

import { formatDuration, markdownTable } from "./metrics-tables.ts";
import { toolUsesTable, toolsTable, usesSince } from "./metrics-tools.ts";
import { savedTurns, turnsTable } from "./turns.ts";

/** A main-agent call with its start, as the metrics log holds it. */
export type StartedCall = {
  readonly durationMs: number;
  readonly inputChars: number;
  readonly startedAt: number;
  readonly turnStart: number;
};

/** One turn's time, split between the tools and the model. */
export type TurnTime = {
  readonly calls: number;
  readonly charsWritten: number;
  readonly modelMs: number;
  readonly toolMs: number;
  readonly turnStart: number;
};

/** Percent of a whole. */
const PERCENT = 100;

/** Characters of an ISO 8601 instant kept: the date, hours and minutes. */
const MINUTE_TEXT = 16;

/** Turns shown in the time table. */
const TURNS_SHOWN = 10;

/** Gaps longer than this are a pause of the person, not the model at work: 5 minutes. */
const PAUSE_MS = 300_000;

/**
 * The main-agent calls of the metrics log that have a start.
 *
 * @param records - parsed lines of the metrics log
 * @returns those calls, oldest first
 */
export function startedCalls(records: readonly unknown[]): readonly StartedCall[] {
  return records
    .filter(
      (record) =>
        text(record, "agent") === "main" &&
        count(record, "startedAt") > 0 &&
        count(record, "turnStart") > 0 &&
        field(record, "isBackground") !== true,
    )
    .map((record) => ({
      durationMs: count(record, "durationMs"),
      inputChars: count(record, "inputChars"),
      startedAt: count(record, "startedAt"),
      turnStart: count(record, "turnStart"),
    }))
    .toSorted((left, right) => left.startedAt - right.startedAt);
}

/**
 * The time of one turn: its tools, and the model between them (gaps over 5 minutes left out).
 *
 * @param calls - the turn's calls, oldest first
 * @returns the turn's split
 */
export function turnTime(calls: readonly StartedCall[]): TurnTime {
  const gaps = calls.slice(1).map((call, index) => {
    const previous = calls[index];
    const gap =
      previous === undefined ? 0 : call.startedAt - (previous.startedAt + previous.durationMs);

    return gap > 0 && gap < PAUSE_MS ? gap : 0;
  });

  return {
    calls: calls.length,
    charsWritten: calls.reduce((sum, call) => sum + call.inputChars, 0),
    modelMs: gaps.reduce((sum, gap) => sum + gap, 0),
    toolMs: calls.reduce((sum, call) => sum + call.durationMs, 0),
    turnStart: calls[0]?.turnStart ?? 0,
  };
}

/**
 * The table of the latest turns' time.
 *
 * @param records - parsed lines of the metrics log
 * @param latest - turns shown
 * @returns a Markdown table, newest turn last
 */
function timeTable(records: readonly unknown[], latest: number): string {
  const calls = startedCalls(records);
  const starts = [...new Set(calls.map((call) => call.turnStart))].slice(-latest);
  const rows = starts.map((start) => {
    const turn = turnTime(calls.filter((call) => call.turnStart === start));
    const share = Math.round((PERCENT * turn.modelMs) / Math.max(1, turn.modelMs + turn.toolMs));

    return [
      Temporal.Instant.fromEpochMilliseconds(start).toString().slice(0, MINUTE_TEXT),
      String(turn.calls),
      formatDuration(turn.toolMs),
      formatDuration(turn.modelMs),
      `${String(share)} %`,
      turn.charsWritten.toLocaleString("en-US"),
    ];
  });

  return markdownTable(
    [
      "Turn (UTC)",
      "Calls",
      "Tools ran",
      "Model between calls",
      "Model share",
      "Characters written",
    ],
    rows,
  );
}

/**
 * The lines of the metrics log written by the `log-tool-call` hook.
 *
 * @returns them parsed, none when the log is missing
 */
export function loggedCalls(): readonly unknown[] {
  return metricsLines("tool-calls.jsonl");
}

/**
 * The lines of a JSON Lines file of the main checkout's `.git/metrics/`.
 *
 * @param file - its name: `tool-calls.jsonl`, `tool-uses.jsonl`
 * @returns them parsed, none when the file is missing
 */
function metricsLines(file: string): readonly unknown[] {
  const common = execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  }).trim();
  const log = join(common, "metrics", file);

  return existsSync(log)
    ? readFileSync(log, "utf8")
        .split("\n")
        .filter((line) => line.startsWith("{"))
        .map((line): unknown => JSON.parse(line))
    : [];
}

/**
 * The first moment a report covers, from its `--since <day>` option.
 *
 * @param options - the command's arguments
 * @returns the day's start, 00:00 UTC, in epoch milliseconds; 0 without the option
 */
export function sinceStart(options: readonly string[]): number {
  const index = options.indexOf("--since");

  return index === -1
    ? 0
    : Temporal.PlainDate.from(options[index + 1] ?? "")
        .toZonedDateTime("UTC")
        .toInstant().epochMilliseconds;
}

/**
 * The lines of the metrics log for calls that ended from a moment on.
 *
 * @param records - parsed lines of the metrics log
 * @param since - the moment, epoch milliseconds
 * @returns those lines, in order
 */
export function loggedSince(records: readonly unknown[], since: number): readonly unknown[] {
  return records.filter(
    (record) => Temporal.Instant.from(text(record, "time")).epochMilliseconds >= since,
  );
}

/**
 * The report's sections on turns and tools: each saved turn, where the latest turns' time went,
 * the agent's tools run through npm, then each Rust tool's recorded uses.
 *
 * @param since - the first moment covered, epoch milliseconds; 0 for every turn
 * @returns titles and tables, in order
 */
export function turnSections(since: number): readonly string[] {
  const calls = loggedSince(loggedCalls(), since);

  return [
    "## Turns",
    turnsTable(savedTurns().filter((turn) => turn.start >= since)),
    "## Where the time of the latest turns went",
    "Main agent only, calls timed by the hooks since RET-002: the tools running, and the model between two calls (reading, thinking, writing the next one; gaps over 5 minutes left out).",
    timeTable(calls, TURNS_SHOWN),
    "## The agent's tools",
    "Each tool of `docs/tooling/agent-tools.md` run through npm: how often, how long, how often it failed.",
    toolsTable(calls),
    "## Tools",
    "Each Rust tool of the workspace, as it records its own uses in `.git/metrics/tool-uses.jsonl`, whoever called it (EN-102): by tool and subcommand, a failure being an exit code other than 0.",
    toolUsesTable(usesSince(metricsLines("tool-uses.jsonl"), since)),
  ];
}
