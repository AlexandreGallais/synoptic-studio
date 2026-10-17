import { describe, expect, it } from "vitest";

import { loggedSince, sinceStart, startedCalls, turnTime } from "./metrics-time";

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

  it("[F18.AC11] starts a report at 00:00 UTC of its --since day, at 0 without it", () => {
    // 2026-10-16 is 20 742 days after 1970-01-01: 20 742 × 86 400 000 ms = 1 792 108 800 000 ms.
    expect(sinceStart(["--since", "2026-10-16"])).toBe(1_792_108_800_000);
    expect(sinceStart([])).toBe(0);
    expect(() => sinceStart(["--since"])).toThrow(RangeError);
  });

  it("[F18.AC11] keeps the logged calls that ended from that moment on", () => {
    const records = [
      { time: "2026-10-15T23:59:59Z", tool: "Bash" },
      { time: "2026-10-16T00:00:00Z", tool: "Read" },
      { time: "2026-10-17T08:00:00Z", tool: "Edit" },
    ];

    expect(loggedSince(records, 1_792_108_800_000)).toEqual(records.slice(1));
    expect(loggedSince(records, 0)).toEqual(records);
  });
});
