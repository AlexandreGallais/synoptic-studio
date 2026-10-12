import { describe, expect, it } from "vitest";

import { placeOf, readRunningCall, runningCall, runningLine } from "./status-place";

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
  // Hook inputs are external JSON (snake_case keys): parsed as the hook parses them.
  const input: unknown = JSON.parse(
    '{"tool_name": "Bash", "cwd": "/repo", "tool_input": {"command": "npm run check", "description": "Run the checks"}}',
  );

  it("records a call starting and reads it back", () => {
    const call = runningCall(input, 1000);

    expect(call).toEqual({
      description: "Run the checks",
      start: 1000,
      summary: "npm run check",
      tool: "Bash",
    });

    const written: unknown = structuredClone(call);

    expect(readRunningCall(written)).toEqual(call);
  });

  it("shows the oldest call, its description or else its command, and how long it runs", () => {
    const calls = [
      { description: "", start: 5000, summary: "ls", tool: "Bash" },
      { description: "Run the checks", start: 1000, summary: "npm run check", tool: "Bash" },
    ];

    expect(runningLine(calls, 13_000, seconds)).toBe("▶ Bash · Run the checks — 12 s");
    expect(runningLine(calls.slice(0, 1), 6000, seconds)).toBe("▶ Bash · ls — 1 s");
    expect(runningLine([], 0, seconds)).toBe("");
  });
});
