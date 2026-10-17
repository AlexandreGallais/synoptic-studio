// Status line of Claude Code (docs/tooling/metrics.md), three lines: where the work stands
// (epic, feature, story, step); the current turn, from the person's prompt (active or silent,
// time, exact tokens), the run's duration and tokens while a `/run` is open (EN-092), how full
// the conversation memory is and the model; the tool call running, if any. Input: the status line
// JSON on stdin (`REF-CC-STATUSLINE`).

// Temporal runs behind `--harmony-temporal` in Node.js 24: the status line command passes it.

import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { join } from "node:path";

import { count, field } from "./status-json.ts";
import { backgroundLines, placeOf, readRunningCall, runningLine } from "./status-place.ts";
import { runPart } from "./status-run.ts";
import { turnTokens } from "./status-tokens.ts";
import { usageSnapshot } from "./status-usage.ts";

import type { RunningCall } from "./status-place.ts";
import type { TokenTally } from "./status-tokens.ts";

/** What the status line shows. */
export type Status = {
  /** One line per background task: subagent or command, what it does now. */
  readonly background: readonly string[];
  /** Share of the context window used, in percent. */
  readonly contextPercent: number;
  /** Time since the current turn started. */
  readonly durationMs: number;
  /** Name of the model. */
  readonly model: string;
  /** Epic, feature and story of the branch checked out. */
  readonly place: string;
  /** The open run's part, `run 2 h 05 · 48,123,456 tokens`, empty outside a run (EN-092). */
  readonly run: string;
  /** Whether that call has run for more than 5 minutes: shown in red, to be checked. */
  readonly isRunningLong: boolean;
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
  magenta: "\u{1B}[35m",
  red: "\u{1B}[31m",
  reset: "\u{1B}[0m",
  yellow: "\u{1B}[33m",
};

/** A call running longer than this, milliseconds, is shown in red: look at what it does. */
const LONG_CALL_MS = 300_000;

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
 * Duration to the second, so that the turn's clock moves at each refresh.
 *
 * @param milliseconds - length of time to write
 * @returns `45 s`, `12 min 05 s` or `2 h 05 min 09 s`
 */
export function clockDuration(milliseconds: number): string {
  const seconds = Math.floor(milliseconds / SECOND);
  const minutes = Math.floor(seconds / SIXTY);
  const secondsText = `${String(seconds % SIXTY).padStart(2, "0")} s`;

  if (minutes < 1) {
    return `${String(seconds)} s`;
  }

  return minutes < SIXTY
    ? `${String(minutes)} min ${secondsText}`
    : `${String(Math.floor(minutes / SIXTY))} h ${String(minutes % SIXTY).padStart(2, "0")} min ${secondsText}`;
}

/**
 * Whether the person's turn has come: the agent ended its turn and waits for no background task.
 *
 * @param tally - tokens, times and background tasks of the turn
 * @returns true when the agent waits for the person
 */
export function isTurnDone(tally: TokenTally): boolean {
  return tally.isWaiting && tally.start !== 0 && tally.pending.length === 0;
}

/**
 * A number of things, the noun in the plural when needed.
 *
 * @param amount - how many
 * @param noun - the noun in the singular
 * @returns `1 agent`, `2 agents`
 */
function counted(amount: number, noun: string): string {
  return `${String(amount)} ${noun}${amount === 1 ? "" : "s"}`;
}

/**
 * The background tasks in a few words.
 *
 * @param tally - the turn's tally
 * @returns `2 agents, 1 command`, empty when nothing runs in the background
 */
export function backgroundText(tally: TokenTally): string {
  const agents = tally.pending.filter((task) => task.kind === "agent").length;
  const commands = tally.pending.filter((task) => task.kind === "command").length;

  return [
    agents === 0 ? "" : counted(agents, "agent"),
    commands === 0 ? "" : counted(commands, "command"),
  ]
    .filter((part) => part !== "")
    .join(", ");
}

/**
 * Activity, in four states: the agent works on the main thread (with or without tasks in the
 * background), or its turn is over and only background tasks run (the chat is free for the
 * person's messages); a working agent silent for a while is shown as such.
 *
 * @param silentMs - time since the transcript last changed
 * @param tally - the turn's tally
 * @returns the colored activity
 */
function activityOf(silentMs: number, tally: TokenTally): string {
  const background = backgroundText(tally);

  if (background !== "" && tally.isWaiting) {
    return `${COLOR.cyan}◌ chat free · ${background} in the background${COLOR.reset}`;
  }

  const also = background === "" ? "" : ` · ${background} in the background`;

  return silentMs < IDLE_MS
    ? `${COLOR.green}● working${also}${COLOR.reset}`
    : `${COLOR.yellow}◌ silent for ${shortDuration(silentMs)}${also}${COLOR.reset}`;
}

/**
 * The turn's line: activity, time, tokens, how full the conversation memory is, model.
 *
 * @param status - what the status line shows
 * @returns one line with ANSI colors
 */
function sessionLine(status: Status): string {
  const { contextPercent, durationMs, model, run, silentMs, tally } = status;
  const isDone = isTurnDone(tally);

  return [
    isDone ? `${COLOR.dim}◌ your turn${COLOR.reset}` : activityOf(silentMs, tally),
    `⏱ ${isDone ? "last turn" : "turn"} ${clockDuration(durationMs)}`,
    `${tally.total.toLocaleString("en-US")} tokens this turn (${tally.output.toLocaleString("en-US")} written)`,
    ...(run === "" ? [] : [`${COLOR.magenta}${run}${COLOR.reset}`]),
    `conversation memory ${String(Math.round(contextPercent))} % full`,
    `${COLOR.dim}${model}${COLOR.reset}`,
  ].join(SEPARATOR);
}

/**
 * The status line's text.
 *
 * @param status - what the line shows
 * @returns two lines or more with ANSI colors: place and step, session, the main thread's running
 * call, then one line per background task
 */
export function statusText(status: Status): string {
  const place = `${COLOR.bold}${status.place}${COLOR.reset}`;
  // No step written: the place alone, never a word that reads as a state ("waiting", RET-003).
  const head = status.step === "" ? place : [place, status.step].join(SEPARATOR);
  const color = status.isRunningLong ? COLOR.red : COLOR.cyan;
  const running = status.running === "" ? [] : [`${color}${status.running}${COLOR.reset}`];
  const background = status.background.map((line) => `${COLOR.dim}${line}${COLOR.reset}`);

  return [head, sessionLine(status), ...running, ...background].join("\n");
}

/**
 * The session's state that each tool call's record copies (`now.json`, read by the
 * `log-tool-call` hook): written at each refresh, so that reading it costs the hook nothing.
 *
 * @param status - what the status line shows
 * @param now - time now, epoch milliseconds
 * @returns the snapshot to write
 */
export function sessionSnapshot(status: Status, now: number): Readonly<Record<string, unknown>> {
  const agents = status.tally.pending.filter((task) => task.kind === "agent").length;

  return {
    contextPercent: status.contextPercent,
    isWaiting: status.tally.isWaiting,
    model: status.model,
    pendingAgents: agents,
    pendingCommands: status.tally.pending.filter((task) => task.kind === "command").length,
    time: now,
    turnOutputTokens: status.tally.output,
    turnStart: status.tally.start,
    turnTokens: status.tally.total,
  };
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
 * @returns each call still running, with the name of its file
 */
function runningCalls(folder: string): readonly (readonly [string, RunningCall])[] {
  return existsSync(folder)
    ? readdirSync(folder)
        .filter((name) => name.endsWith(".json"))
        .map((name) => {
          const json: unknown = JSON.parse(readFileSync(join(folder, name), "utf8"));

          return [name, readRunningCall(json)] as const;
        })
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
  const running = join(folder, "running");

  // A call whose result arrived is over, even if its end was never recorded (a scheduled wake-up).
  for (const id of tally.finished) {
    rmSync(join(running, `${id}.json`), { force: true });
  }

  // A main call started before the turn never finished (an interrupted turn): its file is removed;
  // a subagent's call may outlive the turn that launched the subagent.
  const stale = runningCalls(running).filter(
    ([, call]) => call.agent === "main" && call.start < tally.start,
  );

  for (const [name] of stale) {
    rmSync(join(running, name), { force: true });
  }

  const all = runningCalls(running).map(([, call]) => call);
  const calls = tally.isWaiting
    ? []
    : all.filter((call) => call.agent === "main" && call.start >= tally.start);
  const status: Status = {
    background: backgroundLines(tally.pending, all, { duration: shortDuration, now }),
    contextPercent: count(field(input, "context_window"), "used_percentage"),
    // Once the agent ended its turn, the clock stops at its last answer.
    durationMs:
      tally.start === 0
        ? 0
        : (isTurnDone(tally) && tally.end !== 0 ? tally.end : now) - tally.start,
    model: typeof model === "string" ? model : "",
    place: placeOf(git(["branch", "--show-current"]), backlog),
    run: runPart(root, textOf(join(folder, "run.json"))),
    isRunningLong: calls.some((call) => now - call.start > LONG_CALL_MS),
    running: runningLine(calls, now, shortDuration),
    silentMs: existsSync(transcript) ? now - statSync(transcript).mtimeMs : 0,
    step: textOf(join(folder, "step")),
    tally,
  };

  writeFileSync(join(folder, "now.json"), JSON.stringify(sessionSnapshot(status, now)));

  const usage = usageSnapshot(input, now);

  if (usage !== undefined) {
    writeFileSync(join(folder, "usage.json"), JSON.stringify(usage));
  }

  process.stdout.write(statusText(status));
}
