import { describe, expect, it } from "vitest";

import { agentTool, toolsTable } from "./metrics-tools";

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
});
