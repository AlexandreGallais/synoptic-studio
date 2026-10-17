import { describe, expect, it } from "vitest";

import { agentTool, toolUseKey, toolUsesTable, toolsTable, usesSince } from "./metrics-tools";

describe("metrics-tools", () => {
  it("finds the agent's tool a command runs, flags and directory change included", () => {
    expect(agentTool("cd \"/repo\" && npm run -s replace <<'EOF'")).toBe("replace");
    expect(agentTool("npm run backlog:status -- RET-002 done")).toBe("backlog:status");
    expect(agentTool("npm run test")).toBe("");
    expect(agentTool("ls")).toBe("");
  });

  it("counts each tool's runs and failures, the most used first", () => {
    const records = [
      { durationMs: 100, isOk: true, summary: "npm run -s replace", tool: "Bash" },
      { durationMs: 300, isOk: false, summary: "npm run -s replace", tool: "Bash" },
      { durationMs: 50, isOk: true, summary: "npm run activity", tool: "Bash" },
      { durationMs: 9, isOk: true, summary: "npm run -s replace", tool: "Read" },
    ];
    const rows = toolsTable(records).split("\n");

    expect(rows[2]).toMatch(/^\| `replace` \| 2 \| .* \| 1 \|$/u);
    expect(rows[3]).toMatch(/^\| `activity` \| 1 \|/u);
  });

  it("[F21.AC7] groups the recorded uses by tool and subcommand: calls, failures, median, slowest", () => {
    const records = [
      { durationMs: 10, exitCode: 0, subcommand: "--check", tool: "format" },
      { durationMs: 100, exitCode: 0, subcommand: "sync", tool: "git-sequences" },
      { durationMs: 30, exitCode: 1, subcommand: "--check", tool: "format" },
      { durationMs: 5, exitCode: 0, subcommand: "", tool: "lint" },
      { durationMs: 20, exitCode: 0, subcommand: "--check", tool: "format" },
      { durationMs: 300, exitCode: 0, subcommand: "sync", tool: "git-sequences" },
      { durationMs: 40, exitCode: 2, subcommand: "--write", tool: "format" },
    ].map((use) => ({ ...use, start: "2026-10-18T10:00:00.000Z", worktree: "main" }));

    // format --check: 10, 30, 20 → median 20, slowest 30, one exit 1. git-sequences sync: 100 and
    // 300 → median (100 + 300) / 2 = 200. Ties of one call sorted by name: --write before lint.
    expect(toolUsesTable(records).split("\n")).toEqual([
      "| Tool | Calls | Failed | Median | Slowest |",
      "| --- | --- | --- | --- | --- |",
      "| `format --check` | 3 | 1 | 20 ms | 30 ms |",
      "| `git-sequences sync` | 2 | 0 | 200 ms | 300 ms |",
      "| `format --write` | 1 | 1 | 40 ms | 40 ms |",
      "| `lint` | 1 | 0 | 5 ms | 5 ms |",
    ]);

    expect(toolUsesTable([]).split("\n")).toHaveLength(2);
  });

  it("[F21.AC7] keeps the uses started from the report's first moment on", () => {
    const records = [
      { start: "2026-10-15T23:59:59.999Z", tool: "lint" },
      { start: "2026-10-16T00:00:00.000Z", tool: "format" },
    ];

    // 2026-10-16T00:00Z is 1 792 108 800 000 ms (metrics-time's test).
    expect(usesSince(records, 1_792_108_800_000)).toEqual(records.slice(1));
    expect(usesSince(records, 0)).toEqual(records);
    expect(toolUseKey({ subcommand: "", tool: "lint" })).toBe("lint");
  });
});
