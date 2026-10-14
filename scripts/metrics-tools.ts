// The agent's own tools in the metrics (RET-002, docs/tooling/agent-tools.md): how often each one
// ran, how long it took and how often it failed, from the hook's log, so that each retrospective
// sees whether a tool is used, slow or crashing, and which ones deserve Rust.

import { count, field, text } from "../.claude/hooks/status-json.ts";

import { callStats, statsTable } from "./metrics-tables.ts";

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
