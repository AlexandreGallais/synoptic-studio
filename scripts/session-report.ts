// Report of one working session (docs/tooling/metrics.md): how long it took, how many tokens it
// used, how many tool calls and failures, which commits it made. The agent ends each session with
// it (CLAUDE.md, definition of done). Run by `npm run metrics:session [session id] [--all]
// [--markdown]`: without an id, the session whose transcript changed last; without `--all`, the
// current turn: from the prompt that started it.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

import { transcriptEvents, turnStartTime } from "./metrics-events.ts";
import { formatDuration } from "./metrics-tables.ts";
import { activeMs } from "./metrics-work.ts";

import type { TranscriptEvents } from "./metrics-events.ts";

/** Figures of one session. */
export type SessionFigures = {
  readonly activeMs: number;
  readonly cacheReadTokens: number;
  readonly cacheWriteTokens: number;
  readonly calls: number;
  readonly commits: readonly string[];
  readonly failures: number;
  readonly inputTokens: number;
  readonly outputTokens: number;
  readonly requests: number;
  readonly wallMs: number;
};

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** ANSI colors of the terminal report. */
const COLOR = {
  bold: "\u{1B}[1m",
  cyan: "\u{1B}[36m",
  dim: "\u{1B}[2m",
  green: "\u{1B}[32m",
  magenta: "\u{1B}[35m",
  reset: "\u{1B}[0m",
};

/** Spaces at least between a label and its value in the terminal report. */
const GAP = 4;

/** Token counts of a request, by kind. */
type TokenKind = "cacheRead" | "cacheWrite" | "input" | "output";

/**
 * Tokens of one kind over every request.
 *
 * @param events - requests and tool calls
 * @param kind - which count of `usage`
 * @returns their sum
 */
function tokenSum(events: TranscriptEvents, kind: TokenKind): number {
  return events.requests.reduce((total, request) => total + request.tokens[kind], 0);
}

/**
 * A count written with thousands separators.
 *
 * @param value - a whole number
 * @returns e.g. `41,141,108`
 */
function formatCount(value: number): string {
  return value.toLocaleString("en-US");
}

/**
 * Figures of a session from its events and the commits made meanwhile.
 *
 * @param events - requests and tool calls of the session and its subagents
 * @param commits - one line per commit made during the session
 * @returns its times, tokens, calls and commits
 */
export function sessionFigures(
  events: TranscriptEvents,
  commits: readonly string[],
): SessionFigures {
  const times = [
    ...events.requests.map((request) => request.time),
    ...events.calls.map((call) => call.end),
  ];

  return {
    activeMs: activeMs(times),
    cacheReadTokens: tokenSum(events, "cacheRead"),
    cacheWriteTokens: tokenSum(events, "cacheWrite"),
    calls: events.calls.length,
    commits,
    failures: events.calls.filter((call) => !call.isOk).length,
    inputTokens: tokenSum(events, "input"),
    outputTokens: tokenSum(events, "output"),
    requests: events.requests.length,
    wallMs: times.length === 0 ? 0 : Math.max(...times) - Math.min(...times),
  };
}

/**
 * Rows of the report: label and value.
 *
 * @param figures - figures of the session
 * @returns the rows, in reading order
 */
export function reportRows(figures: SessionFigures): readonly (readonly [string, string])[] {
  const total =
    figures.inputTokens + figures.cacheWriteTokens + figures.cacheReadTokens + figures.outputTokens;

  return [
    ["Duration", `${formatDuration(figures.wallMs)} (active ${formatDuration(figures.activeMs)})`],
    ["Tokens, total", formatCount(total)],
    ["  written by the model", formatCount(figures.outputTokens)],
    ["  read from the cache", formatCount(figures.cacheReadTokens)],
    ["  written to the cache", formatCount(figures.cacheWriteTokens)],
    ["  new input", formatCount(figures.inputTokens)],
    ["Model requests", formatCount(figures.requests)],
    ["Tool calls", `${formatCount(figures.calls)} (${formatCount(figures.failures)} failed)`],
    ["Commits", formatCount(figures.commits.length)],
  ];
}

/**
 * The report as a Markdown table, for a chat message.
 *
 * @param figures - figures of the session
 * @returns a table, then the commits
 */
export function markdownReport(figures: SessionFigures): string {
  const rows = reportRows(figures).map(([label, value]) => `| ${label.trim()} | ${value} |`);
  const commits = figures.commits.map((commit) => `- \`${commit}\``);

  return ["| Session | |", "| --- | --- |", ...rows, "", ...commits].join("\n");
}

/**
 * The report for a terminal, framed and colored.
 *
 * @param figures - figures of the session
 * @returns lines with ANSI colors
 */
export function terminalReport(figures: SessionFigures): string {
  const rows = reportRows(figures);
  const width = Math.max(...rows.map(([label, value]) => label.length + value.length)) + GAP;
  const rule = `${COLOR.dim}${"─".repeat(width)}${COLOR.reset}`;
  const lines = rows.map(([label, value]) => {
    const color = label.startsWith(" ") ? COLOR.dim : COLOR.cyan;

    return `${color}${label}${COLOR.reset}${" ".repeat(width - label.length - value.length)}${COLOR.bold}${value}${COLOR.reset}`;
  });
  const commits = figures.commits.map((commit) => `${COLOR.green}●${COLOR.reset} ${commit}`);

  return [
    `${COLOR.magenta}${COLOR.bold}Session report${COLOR.reset}`,
    rule,
    ...lines,
    rule,
    ...commits,
  ].join("\n");
}

/**
 * Transcript of a session, and those of its subagents.
 *
 * @param folder - transcripts folder of the repository
 * @param session - session id, or empty for the transcript changed last
 * @returns paths of the JSON Lines files
 */
function sessionFiles(folder: string, session: string): readonly string[] {
  const main = readdirSync(folder)
    .filter((name) => name.endsWith(".jsonl") && name.startsWith(session))
    .map((name) => join(folder, name))
    .toSorted((left, right) => statSync(right).mtimeMs - statSync(left).mtimeMs)[0];

  if (main === undefined) {
    return [];
  }

  const subagents = join(main.replace(/\.jsonl$/u, ""), "subagents");

  return [
    main,
    ...(existsSync(subagents)
      ? readdirSync(subagents)
          .filter((name) => name.endsWith(".jsonl"))
          .map((name) => join(subagents, name))
      : []),
  ];
}

/**
 * Events of a session, from a given time on.
 *
 * @param files - transcripts of the session and its subagents
 * @param from - epoch milliseconds
 * @returns requests and calls at or after `from`
 */
function eventsFrom(files: readonly string[], from: number): TranscriptEvents {
  const parsed = files.map((file) => transcriptEvents(readFileSync(file, "utf8")));

  return {
    calls: parsed.flatMap((entry) => entry.calls).filter((call) => call.start >= from),
    requests: parsed.flatMap((entry) => entry.requests).filter((request) => request.time >= from),
  };
}

if (import.meta.main) {
  const options = process.argv.slice(2);
  const session = options.find((option) => !option.startsWith("--")) ?? "";
  const folder = join(homedir(), ".claude", "projects", ROOT.replaceAll(/[^\dA-Za-z]/gu, "-"));
  const files = sessionFiles(folder, session);
  const from =
    options.includes("--all") || files[0] === undefined
      ? 0
      : turnStartTime(readFileSync(files[0], "utf8"));
  const events = eventsFrom(files, from);
  const start = Math.min(...events.requests.map((request) => request.time));
  const since = Temporal.Instant.fromEpochMilliseconds(
    Number.isFinite(start) ? start : 0,
  ).toString();
  const log = execFileSync(
    "git",
    ["log", "--all", "--reverse", `--since=${since}`, "--format=%h %s"],
    {
      cwd: ROOT,
      encoding: "utf8",
    },
  );
  const figures = sessionFigures(
    events,
    log.split("\n").filter((line) => line !== ""),
  );

  process.stdout.write(
    `${options.includes("--markdown") ? markdownReport(figures) : terminalReport(figures)}\n`,
  );
}
