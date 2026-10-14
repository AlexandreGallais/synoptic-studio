import { describe, expect, it } from "vitest";

import { startedCalls, turnTime } from "./metrics-time";

describe("metrics-time", () => {
  it("keeps the main agent's calls that have a start, in order, background ones left out", () => {
    const records = [
      { agent: "main", durationMs: 5, inputChars: 10, startedAt: 2000, turnStart: 1 },
      { agent: "auditor", durationMs: 5, startedAt: 1500, turnStart: 1 },
      { agent: "main", durationMs: 5, startedAt: 0, turnStart: 1 },
      { agent: "main", durationMs: 5, startedAt: 1100, turnStart: 0 },
      { agent: "main", durationMs: 5, isBackground: true, startedAt: 1200, turnStart: 1 },
      { agent: "main", durationMs: 7, inputChars: 3, startedAt: 1000, turnStart: 1 },
    ];

    expect(startedCalls(records).map((call) => call.startedAt)).toEqual([1000, 2000]);
  });

  it("splits a turn between its tools and the model's gaps, a pause over 5 minutes left out", () => {
    // Calls at 0 (1 s), 3 s (2 s), then 10 min later: gaps 2 s, then a pause.
    const calls = [
      { durationMs: 1000, inputChars: 100, startedAt: 0, turnStart: 0 },
      { durationMs: 2000, inputChars: 50, startedAt: 3000, turnStart: 0 },
      { durationMs: 500, inputChars: 5, startedAt: 605_000, turnStart: 0 },
    ];

    expect(turnTime(calls)).toEqual({
      calls: 3,
      charsWritten: 155,
      modelMs: 2000,
      toolMs: 3500,
      turnStart: 0,
    });
  });
});
