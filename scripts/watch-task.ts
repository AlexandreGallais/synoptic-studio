// Watchdog of a background task (docs/tooling/metrics.md): run under Claude Code's Monitor, it
// follows the task's output file and prints a line only when the agent must look — the output
// stalled, came back, matched the end pattern, or the watch ran out of time — so that a stuck task
// is seen within minutes instead of waited on.
//   node --harmony-temporal scripts/watch-task.ts <output file> [--stall 120] [--until <start of a line>] [--max 1800]

import { readFileSync } from "node:fs";

/** What the watchdog knows of the output file. */
export type Watch = {
  /** When the output last changed, epoch milliseconds. */
  readonly changedAt: number;
  /** Whether a stall was already reported. */
  readonly isStalled: boolean;
  /** Size of the output, bytes. */
  readonly size: number;
};

/** What to look for. */
export type WatchRules = {
  /** Output silent this long, milliseconds: a stall. */
  readonly stallMs: number;
  /** A line starting with it ends the watch; plain text, not a pattern (CodeQL: no pattern from the command line). */
  readonly until: string;
};

/** A step of the watch: the new state, and the line to print, possibly empty. */
export type WatchStep = {
  readonly isDone: boolean;
  readonly message: string;
  readonly watch: Watch;
};

/** Line Claude Code appends to a background task's output when the task ends. */
const EXITED = /^\[exited with code \d+\]$/u;

/** Seconds between two looks at the file. */
const PERIOD_S = 5;

/** Milliseconds in a second. */
const SECOND_MS = 1000;

/** Default silence before a stall, seconds. */
const STALL_S = 120;

/** Default length of the watch, seconds: Monitor's longest. */
const MAX_S = 1800;

/**
 * Last non-empty line of a text.
 *
 * @param text - output so far
 * @returns that line, or an empty string
 */
export function lastLine(text: string): string {
  return (
    text
      .split("\n")
      .map((line) => line.trim())
      .findLast((line) => line !== "") ?? ""
  );
}

/** The output at one look. */
export type Look = {
  /** When the look happens, epoch milliseconds. */
  readonly now: number;
  /** The output so far. */
  readonly text: string;
};

/**
 * Step after the output changed: done when a new line matches the end pattern or says the task
 * exited, back after a stall.
 *
 * @param watch - state after the previous look
 * @param look - the output now
 * @param until - start of the line that ends the watch, empty for none
 * @returns the new state and the line to print
 */
function changedStep(watch: Watch, look: Look, until: string): WatchStep {
  const last = lastLine(look.text);
  const added = Buffer.from(look.text)
    .subarray(Math.max(watch.size, 0))
    .toString("utf8")
    .split("\n");
  const end =
    added.find((line) => (until !== "" && line.startsWith(until)) || EXITED.test(line)) ?? "";
  const isDone = end !== "";
  const messages = [isDone ? `done: ${end}` : "", watch.isStalled ? `resumed: ${last}` : ""];

  return {
    isDone,
    message: messages.find((message) => message !== "") ?? "",
    watch: { changedAt: look.now, isStalled: false, size: Buffer.byteLength(look.text) },
  };
}

/**
 * One look at the output.
 *
 * @param watch - state after the previous look
 * @param look - the output now
 * @param rules - stall length and end pattern
 * @returns the new state and the line to print
 */
export function watchStep(watch: Watch, look: Look, rules: WatchRules): WatchStep {
  if (Buffer.byteLength(look.text) !== watch.size) {
    return changedStep(watch, look, rules.until);
  }

  const silentMs = look.now - watch.changedAt;
  const isNewStall = !watch.isStalled && silentMs >= rules.stallMs;
  const seconds = String(Math.round(silentMs / SECOND_MS));

  return {
    isDone: false,
    message: isNewStall
      ? `stalled: no output for ${seconds} s — last line: ${lastLine(look.text)}`
      : "",
    watch: { ...watch, isStalled: watch.isStalled || isNewStall },
  };
}

/**
 * Text of a file, empty when it does not exist yet.
 *
 * @param file - path
 * @returns its content
 */
function readText(file: string): string {
  try {
    return readFileSync(file, "utf8");
  } catch {
    return "";
  }
}

/**
 * Value following an option on the command line.
 *
 * @param options - command-line arguments
 * @param name - option, e.g. `--stall`
 * @param fallback - value when the option is absent
 * @returns the value, or the fallback
 */
function option(options: readonly string[], name: string, fallback: string): string {
  const index = options.indexOf(name);

  return index === -1 ? fallback : (options[index + 1] ?? fallback);
}

if (import.meta.main) {
  const options = process.argv.slice(2);
  const file = options[0] ?? "";
  const rules = {
    stallMs: Number(option(options, "--stall", String(STALL_S))) * SECOND_MS,
    until: option(options, "--until", ""),
  };
  const start = Temporal.Now.instant().epochMilliseconds;
  const endAt = start + Number(option(options, "--max", String(MAX_S))) * SECOND_MS;
  const state = { watch: { changedAt: start, isStalled: false, size: -1 } };
  const timer = setInterval(() => {
    const look = { now: Temporal.Now.instant().epochMilliseconds, text: readText(file) };
    const step = watchStep(state.watch, look, rules);
    const isOver = look.now >= endAt && !step.isDone;
    const lines = [step.message, isOver ? `watch over: ${lastLine(look.text)}` : ""];

    state.watch = step.watch;

    process.stdout.write(
      lines
        .filter((line) => line !== "")
        .map((line) => `${line}\n`)
        .join(""),
    );

    if (isOver || step.isDone) {
      clearInterval(timer);
    }
  }, PERIOD_S * SECOND_MS);
}
