// Report of one working session (docs/tooling/metrics.md): how long it took, how many tokens it
// used, how many tool calls and failures, which commits it made. The agent ends each session with
// it (CLAUDE.md, with `--all`). Run by `npm run metrics:session [session id] [--all]
// [--markdown]`: without an id, the session whose transcript changed last; without `--all`, the
// current turn: from the prompt that started it.

import { execFileSync } from "node:child_process";
import {
  appendFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

import { transcriptEvents, turnStartTime } from "./metrics-events.ts";
import { formatDuration } from "./metrics-tables.ts";
import { activeMs } from "./metrics-work.ts";
import { turnRecord } from "./turns.ts";

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

/** Commits listed by name in a report; beyond, only their count. */
const COMMITS_LISTED = 8;

/**
 * Rows of the report: label and value, the essentials only.
 *
 * @param figures - figures of the session
 * @returns duration, tokens, tokens written, tool calls, commits
 */
export function reportRows(figures: SessionFigures): readonly (readonly [string, string])[] {
  const total =
    figures.inputTokens + figures.cacheWriteTokens + figures.cacheReadTokens + figures.outputTokens;

  return [
    ["Duration", `${formatDuration(figures.wallMs)} (active ${formatDuration(figures.activeMs)})`],
    ["Tokens", formatCount(total)],
    ["Written", formatCount(figures.outputTokens)],
    ["Tool calls", `${formatCount(figures.calls)} (${formatCount(figures.failures)} failed)`],
    ["Commits", formatCount(figures.commits.length)],
  ];
}

/**
 * Commits of a report: each one when few, else none (the count is in the table).
 *
 * @param commits - commits made during the session
 * @returns the commits to list
 */
function listedCommits(commits: readonly string[]): readonly string[] {
  return commits.length <= COMMITS_LISTED ? commits : [];
}

/**
 * The report as a one-row Markdown table, for a chat message.
 *
 * @param figures - figures of the session
 * @returns the table, then the commits when they are few
 */
export function markdownReport(figures: SessionFigures): string {
  const rows = reportRows(figures);
  const commits = listedCommits(figures.commits).map((commit) => `- \`${commit}\``);

  return [
    `| ${rows.map(([label]) => label).join(" | ")} |`,
    `| ${rows.map(() => "---").join(" | ")} |`,
    `| ${rows.map(([, value]) => value).join(" | ")} |`,
    ...(commits.length === 0 ? [] : ["", ...commits]),
  ].join("\n");
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
  const lines = rows.map(
    ([label, value]) =>
      `${COLOR.cyan}${label}${COLOR.reset}${" ".repeat(width - label.length - value.length)}${COLOR.bold}${value}${COLOR.reset}`,
  );
  const commits = listedCommits(figures.commits).map(
    (commit) => `${COLOR.green}●${COLOR.reset} ${commit}`,
  );

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
 * Events a report counts: every turn of the session with `--all`, else the current turn only,
 * from the prompt that started it (the main transcript's last turn).
 *
 * @param transcripts - text of the session's transcript, then those of its subagents
 * @param isAll - whether `--all` was given
 * @returns requests and calls of the turns counted
 */
export function reportedEvents(transcripts: readonly string[], isAll: boolean): TranscriptEvents {
  const [main] = transcripts;
  const from = isAll || main === undefined ? 0 : turnStartTime(main);
  const parsed = transcripts.map((transcript) => transcriptEvents(transcript));

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
  const events = reportedEvents(
    files.map((file) => readFileSync(file, "utf8")),
    options.includes("--all"),
  );
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

  // Saved for the retrospective's comparison of turns (npm run metrics), unless --all.
  if (!options.includes("--all") && Number.isFinite(start)) {
    const common = execFileSync(
      "git",
      ["rev-parse", "--path-format=absolute", "--git-common-dir"],
      {
        cwd: ROOT,
        encoding: "utf8",
      },
    ).trim();
    const record = turnRecord(figures, start, events.requests[0]?.session ?? "");

    mkdirSync(join(common, "metrics"), { recursive: true });
    appendFileSync(join(common, "metrics", "turns.jsonl"), `${JSON.stringify(record)}\n`);
  }

  process.stdout.write(
    `${options.includes("--markdown") ? markdownReport(figures) : terminalReport(figures)}\n`,
  );
}
