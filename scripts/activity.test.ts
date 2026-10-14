import { describe, expect, it } from "vitest";

import { callLine, latestByAgent } from "./activity";

/** A call of the main agent, before a test changes some of it. */
const CALL = {
  agent: "main",
  description: "List the files",
  durationMs: 1234,
  isOk: true,
  summary: "ls",
  time: "2026-10-14T21:04:05.123Z",
  tool: "Bash",
};

describe("activity", () => {
  it("writes a call on one line, its time, agent, tool, description and duration", () => {
    expect(callLine(CALL)).toBe("  21:04:05 main Bash · List the files (1.2 s)");

    expect(callLine({ ...CALL, description: "", isOk: false })).toBe(
      "✗ 21:04:05 main Bash · ls (1.2 s)",
    );
  });

  it("keeps the latest calls of each agent, the main agent first even without calls", () => {
    const calls = [
      { ...CALL, agent: "auditor", summary: "a" },
      { ...CALL, agent: "auditor", summary: "b" },
      { ...CALL, agent: "auditor", summary: "c" },
    ];
    const latest = latestByAgent(calls, 2);

    expect(latest.keys().toArray()).toEqual(["main", "auditor"]);
    expect(latest.get("auditor")?.map((call) => call.summary)).toEqual(["b", "c"]);
  });
});
