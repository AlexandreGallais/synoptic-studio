import { describe, expect, it } from "vitest";

import { IDLE_MS, activeMs, taskCommits, taskIds, taskOf, unitOf, workStats } from "./metrics-work";

import type { Moment } from "./metrics-work";

/**
 * Moment of work for the tests.
 *
 * @param time - epoch milliseconds
 * @param branch - Git branch
 * @returns a tool call of 100 ms, or a request of 10 tokens when `isToolCall` is false
 */
function moment(time: number, branch: string): Moment {
  return {
    branch,
    durationMs: 100,
    isFailure: false,
    isToolCall: true,
    outputTokens: 1,
    session: "s",
    time,
    tokens: 10,
  };
}

describe("unitOf", () => {
  it("reads the story, else the feature, else keeps the branch", () => {
    expect(unitOf("feat/us-018-turn-a-shape")).toBe("US-018");
    expect(unitOf("docs/ret-001-retrospective-e01")).toBe("RET-001");
    expect(unitOf("feature/f04-free-rotation")).toBe("F04");
    expect(unitOf("main")).toBe("main");
    expect(unitOf("")).toBe("(no branch)");
  });
});

describe("activeMs", () => {
  it("adds the gaps between moments, each capped at the idle limit", () => {
    // Gaps: 1000 (kept), then IDLE_MS + 5000 (capped at IDLE_MS).
    expect(activeMs([1000 + IDLE_MS + 5000 + 1000, 0, 1000])).toBe(1000 + IDLE_MS);
    expect(activeMs([42])).toBe(0);
  });
});

describe("taskIds", () => {
  it("reads the task ids of the Refs lines, nothing else", () => {
    expect(taskIds("Body.\n\nRefs: US-011.T2, ADR-0007\nRefs: EN-001.T1\nCloses: US-011")).toEqual([
      "US-011.T2",
      "EN-001.T1",
    ]);
  });
});

describe("taskCommits", () => {
  it("keeps task commits once, oldest first", () => {
    const log = [
      ["bbb", "2026-10-12T10:00:00+02:00", "second", "Refs: US-001.T2"],
      ["aaa", "2026-10-12T09:00:00+02:00", "first", "Refs: US-001.T1"],
      ["ccc", "2026-10-12T09:00:00+02:00", "first", "Refs: US-001.T1"],
      ["ddd", "2026-10-12T11:00:00+02:00", "docs", "No reference"],
    ]
      .map((fields) => fields.join("\u{1F}"))
      .join("\u{1E}\n");

    expect(taskCommits(log).map((commit) => [commit.subject, commit.time])).toEqual([
      // 09:00 at UTC+2 is 07:00 UTC.
      ["first", Temporal.Instant.from("2026-10-12T07:00:00Z").epochMilliseconds],
      ["second", Temporal.Instant.from("2026-10-12T08:00:00Z").epochMilliseconds],
    ]);
  });
});

describe("taskOf and workStats", () => {
  const commits = [
    { hash: "aaaaaaa1", subject: "first", tasks: ["US-001.T1"], time: 2000 },
    { hash: "bbbbbbb2", subject: "second", tasks: ["US-001.T2"], time: 5000 },
  ];

  it("gives a moment to the next task commit of its story", () => {
    expect(taskOf(moment(1000, "feat/us-001-x"), commits)).toBe("US-001.T1 aaaaaaa first");
    expect(taskOf(moment(3000, "feat/us-001-x"), commits)).toBe("US-001.T2 bbbbbbb second");
    expect(taskOf(moment(6000, "feat/us-001-x"), commits)).toBe("US-001 (no task commit)");
    expect(taskOf(moment(1000, "main"), commits)).toBe("main (no task commit)");
  });

  it("sums the moments of each unit", () => {
    const moments = [moment(0, "feat/us-001-x"), moment(1000, "feat/us-001-x"), moment(0, "main")];

    expect(workStats(moments, (entry) => unitOf(entry.branch))).toEqual([
      {
        activeMs: 1000,
        calls: 2,
        failures: 0,
        key: "US-001",
        outputTokens: 2,
        tokens: 20,
        toolMs: 200,
      },
      { activeMs: 0, calls: 1, failures: 0, key: "main", outputTokens: 1, tokens: 10, toolMs: 100 },
    ]);
  });
});
