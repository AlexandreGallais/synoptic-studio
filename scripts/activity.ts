// What the agents are doing (RET-002, the agent's tools in docs/tooling/agent-tools.md): the
// calls running now, the latest calls of the main agent and of each subagent, and the end of the
// output of the background commands of the last hour. For the Product Owner, who cannot see a
// subagent's work until its final message. Run by `npm run activity`, `-- 30` for more calls.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import { count, text, textOr } from "../.claude/hooks/status-json.ts";

/** A tool call as the metrics log holds it, the fields shown here. */
export type LoggedCall = {
  readonly agent: string;
  readonly description: string;
  readonly durationMs: number;
  readonly isOk: boolean;
  readonly summary: string;
  readonly time: string;
  readonly tool: string;
};

/** Characters of a command shown when it has no description. */
const COMMAND_SHOWN = 80;

/** Characters of a running call's record shown. */
const RECORD_SHOWN = 160;

/** Milliseconds per second. */
const SECOND = 1000;

/** Where the clock time sits in an ISO 8601 instant: `hh:mm:ss`. */
const CLOCK = { end: 19, start: 11 };

/** Lines of a background command's output shown. */
const OUTPUT_LINES = 5;

/** Background commands older than this are left out: one hour. */
const RECENT_MS = 3_600_000;

/**
 * One line for a call.
 *
 * @param call - a logged call
 * @returns `21:04:05 auditor Bash · List the files (1.2 s)`, `✗` before a failed one
 */
export function callLine(call: LoggedCall): string {
  const what = call.description === "" ? call.summary.slice(0, COMMAND_SHOWN) : call.description;
  const seconds = (call.durationMs / SECOND).toFixed(1);
  const clock = call.time.slice(CLOCK.start, CLOCK.end);

  return `${call.isOk ? " " : "✗"} ${clock} ${call.agent} ${call.tool} · ${what} (${seconds} s)`;
}

/**
 * The latest calls of each agent, the main one first.
 *
 * @param calls - logged calls, oldest first
 * @param kept - calls kept per agent
 * @returns for each agent, its latest calls, oldest first
 */
export function latestByAgent(
  calls: readonly LoggedCall[],
  kept: number,
): ReadonlyMap<string, readonly LoggedCall[]> {
  const names = new Set(["main", ...calls.map((call) => call.agent)]);
  const agents = names.values().toArray();

  return new Map(
    agents.map((agent) => [agent, calls.filter((call) => call.agent === agent).slice(-kept)]),
  );
}

/**
 * A logged call read from one line of the metrics log.
 *
 * @param line - one JSON line
 * @returns the fields shown, empty or 0 where missing
 */
function loggedCall(line: string): LoggedCall {
  const record: unknown = JSON.parse(line);

  return {
    agent: textOr(record, "agent", "main"),
    description: text(record, "description"),
    durationMs: count(record, "durationMs"),
    isOk: typeof record !== "object" || record === null || Reflect.get(record, "isOk") !== false,
    summary: text(record, "summary"),
    time: text(record, "time"),
    tool: text(record, "tool"),
  };
}

/**
 * Output files of background commands written in the last hour, newest first.
 *
 * @param now - time now, epoch milliseconds
 * @returns their paths
 */
function recentOutputs(now: number): readonly string[] {
  const project = process.cwd().replaceAll(/[/ ]/gu, "-");
  const user = execFileSync("id", ["-u"], { encoding: "utf8" }).trim();
  const root = join("/private/tmp", `claude-${user}`, project);

  return existsSync(root)
    ? readdirSync(root)
        .map((session) => join(root, session, "tasks"))
        .filter((folder) => existsSync(folder))
        .flatMap((folder) => readdirSync(folder).map((name) => join(folder, name)))
        .filter((path) => path.endsWith(".output") && now - statSync(path).mtimeMs < RECENT_MS)
        .toSorted((left, right) => statSync(right).mtimeMs - statSync(left).mtimeMs)
    : [];
}

if (import.meta.main) {
  const kept = Number(process.argv[2] ?? "10");
  const common = execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  }).trim();
  const log = join(common, "metrics", "tool-calls.jsonl");
  const running = join(common, "metrics", "running");
  const now = Temporal.Now.instant().epochMilliseconds;
  const calls = existsSync(log)
    ? readFileSync(log, "utf8")
        .split("\n")
        .filter((line) => line.startsWith("{"))
        .map((line) => loggedCall(line))
    : [];
  const lines = [
    "Running now:",
    ...(existsSync(running) ? readdirSync(running) : []).map(
      (name) => `  ${readFileSync(join(running, name), "utf8").slice(0, RECORD_SHOWN)}`,
    ),
    ...[...latestByAgent(calls, kept)].flatMap(([agent, latest]) => [
      `Latest calls of ${agent}:`,
      ...latest.map((call) => `  ${callLine(call)}`),
    ]),
    ...recentOutputs(now).flatMap((path) => [
      `Background output ${path}:`,
      ...readFileSync(path, "utf8")
        .trimEnd()
        .split("\n")
        .slice(-OUTPUT_LINES)
        .map((line) => `  ${line}`),
    ]),
  ];

  process.stdout.write(`${lines.join("\n")}\n`);
}
