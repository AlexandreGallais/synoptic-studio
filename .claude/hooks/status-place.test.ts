import { describe, expect, it } from "vitest";

import { placeOf, readRunningCall, runningLine } from "./status-place";

/** Backlog files, as the status line lists them. */
const FILES = [
  "E01-F04-US-018-turn-a-shape.md",
  "E01-RET-001-retrospective-e01.md",
  "E14-F11-code-base-ready-for-curves.md",
];

describe("placeOf", () => {
  it("names the epic, feature and story of a branch", () => {
    expect(placeOf("feat/us-018-turn-a-shape", FILES)).toBe("E01 › F04 › US-018 turn a shape");

    expect(placeOf("docs/ret-001-retrospective-e01", FILES)).toBe(
      "E01 › RET-001 retrospective e01",
    );

    expect(placeOf("feature/f11-clean-up", FILES)).toBe(
      "E14 › F11 › F11 code base ready for curves",
    );
  });

  it("keeps the branch when it names no backlog item", () => {
    expect(placeOf("develop", FILES)).toBe("develop");
    expect(placeOf("feat/us-099-unknown", FILES)).toBe("feat/us-099-unknown");
  });
});

/**
 * A duration in seconds, for the tests.
 *
 * @param milliseconds - length of time
 * @returns `12 s`
 */
function seconds(milliseconds: number): string {
  return `${String(milliseconds / 1000)} s`;
}

describe("running calls", () => {
  it("reads back a call starting as the track-tool-call hook writes it", () => {
    const written: unknown = JSON.parse(
      '{"agent":"main","description":"Run the checks","start":1000,"summary":"npm run check","tool":"Bash"}',
    );

    expect(readRunningCall(written)).toEqual({
      agent: "main",
      description: "Run the checks",
      start: 1000,
      summary: "npm run check",
      tool: "Bash",
    });
  });

  it("shows the oldest call, its description or else its command, and how long it runs", () => {
    const calls = [
      { agent: "main", description: "", start: 5000, summary: "ls", tool: "Bash" },
      {
        agent: "main",
        description: "Run the checks",
        start: 1000,
        summary: "npm run check",
        tool: "Bash",
      },
    ];

    expect(runningLine(calls, 13_000, seconds)).toBe("▶ Bash · Run the checks — 12 s");
    expect(runningLine(calls.slice(0, 1), 6000, seconds)).toBe("▶ Bash · ls — 1 s");
    expect(runningLine([], 0, seconds)).toBe("");
  });
});
