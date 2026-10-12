import { describe, expect, it } from "vitest";

import { callSummary, transcriptEvents, turnStartTime } from "./metrics-events";

// Transcript lines are external JSON (snake_case keys): parsed as the script parses them.

/** Tokens reported by a request. */
const USAGE: unknown = JSON.parse(
  '{"cache_creation_input_tokens": 20, "cache_read_input_tokens": 100, "input_tokens": 3, "output_tokens": 7}',
);

/** Result of a tool call that failed. */
const RESULT: unknown = JSON.parse(
  '{"is_error": true, "tool_use_id": "toolu_1", "type": "tool_result"}',
);

/** A transcript: one request written on two lines (text, then a tool call), its result, noise. */
const TRANSCRIPT = [
  JSON.stringify({
    gitBranch: "main",
    message: {
      content: [{ text: "Checking.", type: "text" }],
      id: "msg_1",
      usage: USAGE,
    },
    sessionId: "s1",
    timestamp: "2026-10-12T10:00:00.000Z",
    type: "assistant",
  }),
  JSON.stringify({
    gitBranch: "main",
    message: {
      content: [
        { id: "toolu_1", input: { command: "npm  run\ncheck" }, name: "Bash", type: "tool_use" },
      ],
      id: "msg_1",
      usage: USAGE,
    },
    sessionId: "s1",
    timestamp: "2026-10-12T10:00:01.000Z",
    type: "assistant",
  }),
  JSON.stringify({
    gitBranch: "main",
    message: { content: [RESULT] },
    sessionId: "s1",
    timestamp: "2026-10-12T10:00:31.000Z",
    type: "user",
  }),
  JSON.stringify({ sessionId: "s1", type: "last-prompt" }),
  "",
].join("\n");

describe("transcriptEvents", () => {
  const events = transcriptEvents(TRANSCRIPT);
  const start = Temporal.Instant.from("2026-10-12T10:00:01Z").epochMilliseconds;

  it("counts a request written on several lines once, with its tokens", () => {
    expect(events.requests).toEqual([
      {
        branch: "main",
        id: "msg_1",
        session: "s1",
        time: start,
        tokens: { cacheRead: 100, cacheWrite: 20, input: 3, output: 7 },
      },
    ]);
  });

  it("times a tool call from its request to its result, and keeps its failure", () => {
    expect(events.calls).toEqual([
      {
        branch: "main",
        end: start + 30_000,
        id: "toolu_1",
        isOk: false,
        session: "s1",
        start,
        summary: "npm run check",
        tool: "Bash",
      },
    ]);
  });
});

describe("callSummary", () => {
  it("takes the first field naming what the tool acted on", () => {
    expect(callSummary(JSON.parse('{"file_path": "a.ts", "pattern": "x"}'))).toBe("a.ts");
    expect(callSummary(undefined)).toBe("");
  });
});

describe("turnStartTime", () => {
  it("starts the turn at the first prompt after the agent ended its last turn", () => {
    const lines = [
      '{"type": "user", "timestamp": "2026-10-12T10:00:00Z", "message": {"content": "first"}}',
      '{"type": "assistant", "timestamp": "2026-10-12T10:01:00Z", "message": {"stop_reason": "end_turn"}}',
      '{"type": "user", "timestamp": "2026-10-12T11:00:00Z", "message": {"content": "second"}}',
      '{"type": "user", "timestamp": "2026-10-12T11:05:00Z", "isMeta": true, "message": {"content": "skill"}}',
      '{"type": "user", "timestamp": "2026-10-12T11:30:00Z", "message": {"content": [{"type": "text", "text": "while working"}]}}',
    ].join("\n");

    expect(turnStartTime(lines)).toBe(
      Temporal.Instant.from("2026-10-12T11:00:00Z").epochMilliseconds,
    );

    expect(turnStartTime("")).toBe(0);
  });
});
