// The agent's own tools in the metrics (RET-002, docs/tooling/agent-tools.md): how often each one
// ran, how long it took and how often it failed, from the hook's log, so that each retrospective
// sees whether a tool is used, slow or crashing, and which ones deserve Rust; and each Rust tool's
// own record of its uses, `tool-uses.jsonl` (EN-102, docs/tooling/metrics.md).

import { count, field, text } from "../.claude/hooks/status-json.ts";

import { callStats, formatDuration, markdownTable, statsTable } from "./metrics-tables.ts";

import type { TimedCall } from "./metrics-tables.ts";

/** The npm scripts that are the agent's tools. */
const AGENT_TOOLS = new Set([
  "activity",
  "backlog:status",
  "check:all",
  "epic:check",
  "golden:check",
  "golden:write",
  "metrics",
  "metrics:session",
  "records:save",
  "replace",
  "story:finish",
  "branches:clean",
  "library",
]);

/**
 * The agent's tool a shell command runs, if any.
 *
 * @param command - the command as logged
 * @returns the tool's name, empty when it runs none
 */
export function agentTool(command: string): string {
  const match = /\bnpm run (?:-s |--silent )?(?<name>[\w:]+)/u.exec(command);
  const name = match?.groups?.["name"] ?? "";

  return AGENT_TOOLS.has(name) ? name : "";
}

/**
 * What a recorded use of a tool is grouped under: the tool, then its subcommand when it has one.
 *
 * @param record - one line of `tool-uses.jsonl`
 * @returns `git-sequences sync`, `lint`
 */
export function toolUseKey(record: unknown): string {
  return [text(record, "tool"), text(record, "subcommand")].filter((word) => word !== "").join(" ");
}

/**
 * The recorded uses that started from a moment on.
 *
 * @param records - parsed lines of `tool-uses.jsonl`
 * @param since - the moment, epoch milliseconds; 0 for every use
 * @returns those lines, in order
 */
export function usesSince(records: readonly unknown[], since: number): readonly unknown[] {
  return since === 0
    ? records
    : records.filter(
        (record) => Temporal.Instant.from(text(record, "start")).epochMilliseconds >= since,
      );
}

/**
 * The table of each tool's recorded uses (EN-102): calls, failures, median and slowest durations.
 *
 * @param records - parsed lines of `tool-uses.jsonl`
 * @returns a Markdown table, the most called first, then by name; a failure is a code other than 0
 */
export function toolUsesTable(records: readonly unknown[]): string {
  const uses: readonly TimedCall[] = records.map((record) => ({
    durationMs: count(record, "durationMs"),
    isOk: count(record, "exitCode") === 0,
    summary: toolUseKey(record),
    tool: text(record, "tool"),
  }));
  const stats = callStats(uses, (use) => use.summary).toSorted((left, right) =>
    right.count === left.count ? left.key.localeCompare(right.key) : right.count - left.count,
  );

  return markdownTable(
    ["Tool", "Calls", "Failed", "Median", "Slowest"],
    stats.map((entry) => [
      `\`${entry.key}\``,
      String(entry.count),
      String(entry.failures),
      formatDuration(entry.medianMs),
      formatDuration(entry.maxMs),
    ]),
  );
}

/**
 * The table of the agent's tools: count, total, median, longest, failed.
 *
 * @param records - parsed lines of the hook's log
 * @returns a Markdown table, the most used first
 */
export function toolsTable(records: readonly unknown[]): string {
  const calls: readonly TimedCall[] = records
    .filter(
      (record) => text(record, "tool") === "Bash" && agentTool(text(record, "summary")) !== "",
    )
    .map((record) => ({
      durationMs: count(record, "durationMs"),
      isOk: field(record, "isOk") !== false,
      summary: text(record, "summary"),
      tool: "Bash",
    }));

  return statsTable(
    callStats(calls, (call) => agentTool(call.summary)).toSorted(
      (left, right) => right.count - left.count,
    ),
  );
}
