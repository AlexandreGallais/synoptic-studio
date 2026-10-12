import { describe, expect, it } from "vitest";

import { markdownReport, reportRows, sessionFigures, terminalReport } from "./session-report";

/** Two requests and one failed tool call, 10 minutes apart. */
const EVENTS = {
  calls: [
    {
      branch: "develop",
      end: 600_000,
      id: "t1",
      isOk: false,
      session: "s",
      start: 300_000,
      summary: "npm run check",
      tool: "Bash",
    },
  ],
  requests: [
    {
      branch: "develop",
      id: "m1",
      session: "s",
      time: 0,
      tokens: { cacheRead: 1000, cacheWrite: 200, input: 3, output: 50 },
    },
    {
      branch: "develop",
      id: "m2",
      session: "s",
      time: 60_000,
      tokens: { cacheRead: 2000, cacheWrite: 0, input: 1, output: 25 },
    },
  ],
};

describe("session report", () => {
  const figures = sessionFigures(EVENTS, ["abc1234 docs: x"]);

  it("sums tokens by kind and times the session", () => {
    expect(figures).toEqual({
      // Gaps 60 s and 540 s, the second capped at 5 minutes: 60 + 300 = 360 s.
      activeMs: 360_000,
      cacheReadTokens: 3000,
      cacheWriteTokens: 200,
      calls: 1,
      commits: ["abc1234 docs: x"],
      failures: 1,
      inputTokens: 4,
      outputTokens: 75,
      requests: 2,
      wallMs: 600_000,
    });
  });

  it("lists the total of every kind of token", () => {
    // 4 + 200 + 3000 + 75 = 3279.
    expect(reportRows(figures)[1]).toEqual(["Tokens", "3,279"]);
  });

  it("writes a Markdown table and the commits", () => {
    const report = markdownReport(figures);

    expect(report.split("\n").slice(0, 3)).toEqual([
      "| Duration | Tokens | Written | Tool calls | Commits |",
      "| --- | --- | --- | --- | --- |",
      "| 10 min 00 s (active 6 min 00 s) | 3,279 | 75 | 1 (1 failed) | 1 |",
    ]);

    expect(report).toContain("- `abc1234 docs: x`");
  });

  it("frames the terminal report", () => {
    expect(terminalReport(figures)).toContain("Session report");
  });
});
