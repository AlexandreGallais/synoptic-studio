// Turns of work kept for later comparison (docs/tooling/metrics.md): each session report is saved
// as one line in the Git directory by `tools/metrics` (`turn_record`, EN-107), and the metrics
// report lists them all, so that a retrospective
// compares the turns of an epic — time, tokens, calls, failures, commits.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { count, text } from "./metrics-events.ts";
import { formatDuration, markdownTable } from "./metrics-tables.ts";

/** One turn of work, as saved. */
export type TurnRecord = {
  readonly activeMs: number;
  readonly calls: number;
  readonly commits: number;
  readonly failures: number;
  readonly outputTokens: number;
  readonly session: string;
  readonly start: number;
  readonly tokens: number;
  readonly wallMs: number;
};

/** Characters of an ISO 8601 instant kept: date, hours and minutes. */
const MINUTE_LENGTH = 16;

/** Characters of a session id shown. */
const SESSION_SHOWN = 8;

/**
 * A saved turn read back.
 *
 * @param record - parsed line of `turns.jsonl`
 * @returns the turn, a missing number read as 0 and a missing session as empty
 */
export function readTurn(record: unknown): TurnRecord {
  return {
    activeMs: count(record, "activeMs"),
    calls: count(record, "calls"),
    commits: count(record, "commits"),
    failures: count(record, "failures"),
    outputTokens: count(record, "outputTokens"),
    session: text(record, "session"),
    start: count(record, "start"),
    tokens: count(record, "tokens"),
    wallMs: count(record, "wallMs"),
  };
}

/**
 * Turns saved by the session reports.
 *
 * @returns each saved line, in saving order
 */
export function savedTurns(): readonly TurnRecord[] {
  const common = execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  }).trim();
  const file = join(common, "metrics", "turns.jsonl");

  return existsSync(file)
    ? readFileSync(file, "utf8")
        .split("\n")
        .filter((line) => line.startsWith("{"))
        .map((line) => readTurn(JSON.parse(line)))
    : [];
}

/**
 * Table of the turns saved, oldest first; a turn saved twice keeps its last figures.
 *
 * @param records - saved turns, in saving order
 * @returns a Markdown table: start, session, duration, active time, tokens, written, calls, failed, commits
 */
export function turnsTable(records: readonly TurnRecord[]): string {
  const turns = new Map(
    records.map((record) => [`${record.session} ${String(record.start)}`, record]),
  );
  const rows = turns
    .values()
    .toArray()
    .toSorted((left, right) => left.start - right.start)
    .map((turn) => [
      Temporal.Instant.fromEpochMilliseconds(turn.start).toString().slice(0, MINUTE_LENGTH),
      turn.session.slice(0, SESSION_SHOWN),
      formatDuration(turn.wallMs),
      formatDuration(turn.activeMs),
      turn.tokens.toLocaleString("en-US"),
      turn.outputTokens.toLocaleString("en-US"),
      String(turn.calls),
      String(turn.failures),
      String(turn.commits),
    ]);

  return markdownTable(
    [
      "Started (UTC)",
      "Session",
      "Duration",
      "Active",
      "Tokens",
      "Written",
      "Calls",
      "Failed",
      "Commits",
    ],
    rows,
  );
}
