// Metrics report of the agent's work (docs/tooling/metrics.md): reads the Claude Code transcripts
// of this repository and the log of the `log-tool-call` hook, prints a Markdown report — sessions,
// commands by count and by time, slowest calls, time and tokens per story, feature and task.
// Run by `npm run metrics` (Temporal: `node --harmony-temporal`).

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

import { libraryPath, openLibrary } from "./library-store.ts";
import { usageReport } from "./library-usage.ts";
import { field, text, transcriptEvents } from "./metrics-events.ts";
import {
  callStats,
  commandKey,
  formatDuration,
  markdownTable,
  statsTable,
} from "./metrics-tables.ts";
import { taskCommits, taskOf, unitOf, workStats } from "./metrics-work.ts";
import { savedTurns, turnsTable } from "./turns.ts";

import type { ModelRequest, ToolCall } from "./metrics-events.ts";
import type { TimedCall } from "./metrics-tables.ts";
import type { Moment, WorkStats } from "./metrics-work.ts";

/** A tool call with its duration, exact (hook) or estimated (transcript times). */
type Timed = TimedCall & ToolCall & { readonly isExact: boolean };

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Rows shown in each ranked table. */
const TOP = 20;

/** Characters of a call's summary shown in the table of slowest calls. */
const SUMMARY_SHOWN = 80;

/** Characters of an ISO 8601 instant that give its day (`2026-10-12`). */
const DAY_LENGTH = 10;

/** Characters of a session id shown, enough to find its transcript. */
const SESSION_SHOWN = 8;

/**
 * Lines of a text file that hold JSON objects.
 *
 * @param path - JSON Lines file
 * @returns each object line parsed
 */
function jsonLines(path: string): readonly unknown[] {
  return readFileSync(path, "utf8")
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .map((line): unknown => JSON.parse(line));
}

/**
 * Claude Code transcripts of this repository: the sessions and their subagents.
 *
 * @returns paths of the JSON Lines files
 */
function transcriptFiles(): readonly string[] {
  const folder = join(homedir(), ".claude", "projects", ROOT.replaceAll(/[^\dA-Za-z]/gu, "-"));

  return existsSync(folder)
    ? readdirSync(folder, { encoding: "utf8", recursive: true })
        .filter((name) => name.endsWith(".jsonl"))
        .map((name) => join(folder, name))
    : [];
}

/**
 * Exact durations written by the `log-tool-call` hook, by tool call.
 *
 * @returns milliseconds by tool use id
 */
function hookDurations(): ReadonlyMap<string, number> {
  const common = execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  }).trim();
  const log = join(common, "metrics", "tool-calls.jsonl");

  return new Map(
    (existsSync(log) ? jsonLines(log) : [])
      .filter((record) => typeof field(record, "durationMs") === "number")
      .map((record) => [text(record, "toolUseId"), Number(field(record, "durationMs"))]),
  );
}

/**
 * Moment of work of a model request.
 *
 * @param request - a request and its tokens
 * @returns the moment, with every token it processed and those it wrote
 */
function requestMoment(request: ModelRequest): Moment {
  const { cacheRead, cacheWrite, input, output } = request.tokens;

  return {
    branch: request.branch,
    durationMs: 0,
    isFailure: false,
    isToolCall: false,
    outputTokens: output,
    session: request.session,
    time: request.time,
    tokens: cacheRead + cacheWrite + input + output,
  };
}

/**
 * Moment of work of a tool call.
 *
 * @param call - a timed call
 * @returns the moment, at the call's end
 */
function callMoment(call: Timed): Moment {
  return {
    branch: call.branch,
    durationMs: call.durationMs,
    isFailure: !call.isOk,
    isToolCall: true,
    outputTokens: 0,
    session: call.session,
    time: call.end,
    tokens: 0,
  };
}

/**
 * Table of work totals, longest active time first.
 *
 * @param title - name of the first column
 * @param stats - totals of each unit
 * @returns a Markdown table
 */
function workTable(title: string, stats: readonly WorkStats[]): string {
  return markdownTable(
    [title, "Active time", "Output tokens", "All tokens", "Tool calls", "Tool time", "Failed"],
    stats
      .toSorted((left, right) => right.activeMs - left.activeMs)
      .slice(0, TOP)
      .map((entry) => [
        entry.key,
        formatDuration(entry.activeMs),
        entry.outputTokens.toLocaleString("en-US"),
        entry.tokens.toLocaleString("en-US"),
        String(entry.calls),
        formatDuration(entry.toolMs),
        String(entry.failures),
      ]),
  );
}

/**
 * Feature of each story, from the story file names (`E01-F04-US-018-….md`).
 *
 * @returns feature ids by story id
 */
function storyFeatures(): ReadonlyMap<string, string> {
  return new Map(
    readdirSync(join(ROOT, "docs", "backlog", "stories")).flatMap((name) => {
      const match = /^E\d{2}-(?<feature>F\d{2})-(?<story>[A-Z]+-\d{3})-/u.exec(name)?.groups;

      return match === undefined ? [] : [[match["story"] ?? "", match["feature"] ?? ""] as const];
    }),
  );
}

/**
 * Sections of the report about commands.
 *
 * @param calls - timed tool calls
 * @returns Markdown sections
 */
function commandSections(calls: readonly Timed[]): readonly string[] {
  const byCommand = callStats(
    calls.filter((call) => call.tool === "Bash"),
    (call) => commandKey(call.summary),
  );
  const slowest = calls.toSorted((left, right) => right.durationMs - left.durationMs).slice(0, TOP);

  return [
    "## Commands by count",
    statsTable(byCommand.toSorted((left, right) => right.count - left.count).slice(0, TOP)),
    "## Commands by total time",
    statsTable(byCommand.toSorted((left, right) => right.totalMs - left.totalMs).slice(0, TOP)),
    "## Slowest calls",
    markdownTable(
      ["Duration", "Exact", "Tool", "Branch", "What"],
      slowest.map((call) => [
        formatDuration(call.durationMs),
        call.isExact ? "yes" : "no",
        call.tool,
        call.branch,
        `\`${call.summary.slice(0, SUMMARY_SHOWN)}\``,
      ]),
    ),
  ];
}

/**
 * Session of a moment, named by the day it started.
 *
 * @param moment - a request or a tool call
 * @param starts - first moment of each session, epoch milliseconds
 * @returns `2026-10-12 168220a7`
 */
function sessionKey(moment: Moment, starts: ReadonlyMap<string, number>): string {
  const start = Temporal.Instant.fromEpochMilliseconds(starts.get(moment.session) ?? 0);

  return `${start.toString().slice(0, DAY_LENGTH)} ${moment.session.slice(0, SESSION_SHOWN)}`;
}

/**
 * Sections of the report about units of work.
 *
 * @param moments - requests and calls as moments of work, oldest first
 * @returns Markdown sections
 */
function workSections(moments: readonly Moment[]): readonly string[] {
  const commits = taskCommits(
    execFileSync("git", ["log", "--all", "--format=%H%x1f%aI%x1f%s%x1f%b%x1e"], {
      cwd: ROOT,
      encoding: "utf8",
    }),
  );
  const features = storyFeatures();
  const starts = new Map(moments.toReversed().map((moment) => [moment.session, moment.time]));

  return [
    "## Sessions",
    workTable(
      "Started, session",
      workStats(moments, (moment) => sessionKey(moment, starts)),
    ),
    "## Features",
    workTable(
      "Feature",
      workStats(moments, (moment) => features.get(unitOf(moment.branch)) ?? unitOf(moment.branch)),
    ),
    "## Stories and branches",
    workTable(
      "Unit",
      workStats(moments, (moment) => unitOf(moment.branch)),
    ),
    "## Tasks, by commit",
    workTable(
      "Task",
      workStats(moments, (moment) => taskOf(moment, commits)),
    ),
  ];
}

if (import.meta.main) {
  const exact = hookDurations();
  const parsed = transcriptFiles().map((file) => transcriptEvents(readFileSync(file, "utf8")));
  const requests = parsed.flatMap((entry) => entry.requests);
  const calls: readonly Timed[] = parsed
    .flatMap((entry) => entry.calls)
    .map((call) => ({
      ...call,
      durationMs: exact.get(call.id) ?? call.end - call.start,
      isExact: exact.has(call.id),
    }));
  const moments = [
    ...requests.map((request) => requestMoment(request)),
    ...calls.map((call) => callMoment(call)),
  ].toSorted((left, right) => left.time - right.time);
  const exactCount = calls.filter((call) => call.isExact).length;
  const summary = [
    `${String(requests.length)} model requests, ${String(calls.length)} tool calls`,
    `(${String(exactCount)} timed exactly by the hook; the others from transcript times, which also`,
    "count the time spent writing the command). Active time: the gaps between moments of work,",
    "each capped at 5 minutes.",
  ].join(" ");

  const library = openLibrary(libraryPath());
  const sections = [
    "# Metrics of the agent's work",
    summary,
    ...commandSections(calls),
    ...workSections(moments),
    "## Turns",
    turnsTable(savedTurns()),
    "## Sources",
    usageReport(library, 0),
  ];

  process.stdout.write(`${sections.join("\n\n")}\n`);
}
