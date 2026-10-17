import { describe, expect, it } from "vitest";

import {
  markdownReport,
  reportRows,
  reportedEvents,
  sessionFigures,
  terminalReport,
} from "./session-report";

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

/**
 * Transcript line of a request: an answer and its tokens.
 *
 * @param id - message id
 * @param time - ISO 8601 timestamp
 * @param stop - why the answer stopped: `end_turn` ends the agent's turn
 * @returns one JSON line, 1 input token and 10 written
 */
function answer(id: string, time: string, stop: string): string {
  return `{"type": "assistant", "sessionId": "s", "timestamp": "${time}", "message": {"id": "${id}", "stop_reason": "${stop}", "usage": {"input_tokens": 1, "output_tokens": 10}}}`;
}

describe("reportedEvents", () => {
  // Three turns, each opened by a prompt of the Product Owner and closed by the agent; a subagent
  // answered during the first one.
  const main = [
    '{"type": "user", "timestamp": "2026-10-15T08:00:00Z", "message": {"content": "first"}}',
    answer("a", "2026-10-15T08:01:00Z", "end_turn"),
    '{"type": "user", "timestamp": "2026-10-15T09:00:00Z", "message": {"content": "second"}}',
    answer("b", "2026-10-15T09:10:00Z", "end_turn"),
    '{"type": "user", "timestamp": "2026-10-15T18:00:00Z", "message": {"content": "third"}}',
    answer("c", "2026-10-15T18:00:02Z", "end_turn"),
  ].join("\n");
  const subagent = answer("s1", "2026-10-15T08:00:30Z", "tool_use");

  it("[F18.AC9] counts every turn of the session with --all, the subagents' requests included", () => {
    const figures = sessionFigures(reportedEvents([main, subagent], true), []);

    // Requests a, s1, b and c: 4 × 11 tokens; from 08:00:30 to 18:00:02.
    expect(figures.requests).toBe(4);
    expect(figures.inputTokens + figures.outputTokens).toBe(44);
    expect(figures.wallMs).toBe(35_972_000);
  });

  it("counts the current turn only without --all: the 2.8 s seen for 10 h 40 of work", () => {
    const figures = sessionFigures(reportedEvents([main, subagent], false), []);

    // Request c alone, from the prompt at 18:00:00: one request, no duration.
    expect(figures.requests).toBe(1);
    expect(figures.wallMs).toBe(0);
    expect(reportedEvents([], false)).toEqual({ calls: [], requests: [] });
  });
});
