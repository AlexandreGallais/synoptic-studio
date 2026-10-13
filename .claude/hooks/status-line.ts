// Status line of Claude Code (docs/tooling/metrics.md), three lines: where the work stands
// (epic, feature, story, step); the current turn, from the person's prompt (active or silent,
// time, exact tokens), with how full the conversation memory is and the model; the tool call running, if any. Input: the status line JSON on
// stdin (`REF-CC-STATUSLINE`).

// Temporal runs behind `--harmony-temporal` in Node.js 24: the status line command passes it.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import { placeOf, readRunningCall, runningLine } from "./status-place.ts";
import { count, field, turnTokens } from "./status-tokens.ts";

import type { RunningCall } from "./status-place.ts";
import type { TokenTally } from "./status-tokens.ts";

/** What the status line shows. */
export type Status = {
  /** Share of the context window used, in percent. */
  readonly contextPercent: number;
  /** Time since the current turn started. */
  readonly durationMs: number;
  /** Name of the model. */
  readonly model: string;
  /** Epic, feature and story of the branch checked out. */
  readonly place: string;
  /** Line of the tool call running, possibly empty. */
  readonly running: string;
  /** Time since the transcript last changed. */
  readonly silentMs: number;
  /** What the agent says it is doing, possibly empty. */
  readonly step: string;
  /** Exact tokens of the session. */
  readonly tally: TokenTally;
};

/** Milliseconds per second. */
const SECOND = 1000;

/** Seconds per minute and minutes per hour. */
const SIXTY = 60;

/** Silence after which the session is shown as silent (2 minutes). */
const IDLE_MS = 120_000;

/** ANSI colors of the status line. */
const COLOR = {
  bold: "\u{1B}[1m",
  cyan: "\u{1B}[36m",
  dim: "\u{1B}[2m",
  green: "\u{1B}[32m",
  reset: "\u{1B}[0m",
  yellow: "\u{1B}[33m",
};

/** Separator between the parts of a line. */
const SEPARATOR = `${COLOR.dim} · ${COLOR.reset}`;

/**
 * Duration written short.
 *
 * @param milliseconds - length of time to write
 * @returns `45 s`, `12 min` or `2 h 05`
 */
export function shortDuration(milliseconds: number): string {
  const minutes = Math.floor(milliseconds / SECOND / SIXTY);

  if (minutes < 1) {
    return `${String(Math.round(milliseconds / SECOND))} s`;
  }

  return minutes < SIXTY
    ? `${String(minutes)} min`
    : `${String(Math.floor(minutes / SIXTY))} h ${String(minutes % SIXTY).padStart(2, "0")}`;
}

/**
 * The turn's line: activity, time, tokens, how full the conversation memory is, model.
 *
 * @param status - what the status line shows
 * @returns one line with ANSI colors
 */
function sessionLine(status: Status): string {
  const { contextPercent, durationMs, model, silentMs, tally } = status;
  const activity =
    silentMs < IDLE_MS
      ? `${COLOR.green}● active${COLOR.reset}`
      : `${COLOR.yellow}◌ silent for ${shortDuration(silentMs)}${COLOR.reset}`;

  return [
    activity,
    `⏱ turn ${shortDuration(durationMs)}`,
    `${tally.total.toLocaleString("en-US")} tokens this turn (${tally.output.toLocaleString("en-US")} written)`,
    `conversation memory ${String(Math.round(contextPercent))} % full`,
    `${COLOR.dim}${model}${COLOR.reset}`,
  ].join(SEPARATOR);
}

/**
 * The status line's text.
 *
 * @param status - what the line shows
 * @returns two or three lines with ANSI colors: place and step, session, running call
 */
export function statusText(status: Status): string {
  const place = `${COLOR.bold}${status.place}${COLOR.reset}`;
  const step = status.step === "" ? "waiting" : status.step;
  const running = status.running === "" ? [] : [`${COLOR.cyan}${status.running}${COLOR.reset}`];

  return [[place, step].join(SEPARATOR), sessionLine(status), ...running].join("\n");
}

/**
 * Text of a file, empty when it is missing.
 *
 * @param path - file to read
 * @returns its trimmed text
 */
function textOf(path: string): string {
  return existsSync(path) ? readFileSync(path, "utf8").trim() : "";
}

/**
 * Calls running, as the `track-tool-call` hook wrote them.
 *
 * @param folder - `metrics/running` of the Git directory
 * @returns each call still running
 */
function runningCalls(folder: string): readonly RunningCall[] {
  return existsSync(folder)
    ? readdirSync(folder)
        .filter((name) => name.endsWith(".json"))
        .map((name) => readFileSync(join(folder, name), "utf8"))
        .map((json) => readRunningCall(JSON.parse(json)))
    : [];
}

/**
 * Output of a Git command.
 *
 * @param gitArguments - what follows `git`, e.g. `branch --show-current`
 * @returns its trimmed standard output
 */
function git(gitArguments: readonly string[]): string {
  return execFileSync("git", gitArguments, { encoding: "utf8" }).trim();
}

if (import.meta.main) {
  const input: unknown = JSON.parse(readFileSync(0, "utf8"));
  const path = field(input, "transcript_path");
  const transcript = typeof path === "string" ? path : "";
  const folder = join(git(["rev-parse", "--path-format=absolute", "--git-common-dir"]), "metrics");
  const root = git(["rev-parse", "--show-toplevel"]);
  const backlog = ["stories", "features"].flatMap((name) =>
    readdirSync(join(root, "docs", "backlog", name)),
  );
  const now = Temporal.Now.instant().epochMilliseconds;
  const model = field(field(input, "model"), "display_name");

  mkdirSync(folder, { recursive: true });

  const tally = turnTokens(transcript, join(folder, "status-line-turn.json"));
  const status: Status = {
    contextPercent: count(field(input, "context_window"), "used_percentage"),
    durationMs: tally.start === 0 ? 0 : now - tally.start,
    model: typeof model === "string" ? model : "",
    place: placeOf(git(["branch", "--show-current"]), backlog),
    // A call started before the turn was interrupted and never finished: not shown.
    running: runningLine(
      runningCalls(join(folder, "running")).filter((call) => call.start >= tally.start),
      now,
      shortDuration,
    ),
    silentMs: existsSync(transcript) ? now - statSync(transcript).mtimeMs : 0,
    step: textOf(join(folder, "step")),
    tally,
  };

  process.stdout.write(statusText(status));
}
