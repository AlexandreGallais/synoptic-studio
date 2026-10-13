import { describe, expect, it } from "vitest";

import { clockDuration, isTurnDone, shortDuration, statusText } from "./status-line";
import { EMPTY_TALLY, addTokens, cachedTally } from "./status-tokens";

// Transcript lines are external JSON (snake_case keys), written here as the transcript holds them.

/**
 * Transcript line of an assistant message.
 *
 * @param id - message id
 * @param output - output tokens
 * @returns one JSON line, 1 input, 10 cache writes and 100 cache reads
 */
function line(id: string, output: number): string {
  return `{"message": {"id": "${id}", "usage": {"input_tokens": 1, "cache_creation_input_tokens": 10, "cache_read_input_tokens": 100, "output_tokens": ${String(output)}}}}`;
}

/**
 * Transcript line of a prompt typed by the person.
 *
 * @param time - ISO 8601 timestamp
 * @returns one JSON line
 */
function prompt(time: string): string {
  return `{"type": "user", "timestamp": "${time}", "message": {"content": "go on"}}`;
}

/** What the status line shows, before a test changes some of it. */
const STATUS = {
  contextPercent: 10,
  durationMs: 60_000,
  isRunningLong: false,
  model: "Opus 5.5",
  place: "E14 › F12",
  running: "",
  silentMs: 1000,
  step: "",
  tally: EMPTY_TALLY,
};

/** Transcript line of the agent ending its turn. */
const END = '{"message": {"id": "z", "stop_reason": "end_turn"}}';

describe("status line", () => {
  it("counts a request written on two lines once, and moves past the text read", () => {
    const text = `${line("a", 5)}\n${line("a", 5)}\n{"type": "user"}\n${line("b", 7)}\n`;

    // a: 1 + 10 + 100 + 5 = 116; b: 1 + 10 + 100 + 7 = 118.
    expect(addTokens(EMPTY_TALLY, text)).toEqual({
      ...EMPTY_TALLY,
      lastId: "b",
      offset: Buffer.byteLength(text),
      output: 12,
      total: 234,
    });
  });

  it("continues a tally, without counting again the request it ended on", () => {
    const tally = addTokens(EMPTY_TALLY, `${line("a", 5)}\n`);

    expect(addTokens(tally, `${line("a", 5)}\n${line("c", 1)}\n`).total).toBe(116 + 112);
  });

  it("counts the current turn only, from the prompt after the agent ended its turn", () => {
    const first = `${prompt("2026-10-13T08:00:00Z")}\n${line("a", 5)}\n${END}\n`;
    const second = `${prompt("2026-10-13T09:00:00Z")}\n${line("b", 7)}\n${prompt("2026-10-13T09:05:00Z")}\n${line("c", 1)}\n`;
    const tally = addTokens(addTokens(EMPTY_TALLY, first), second);

    // b and c only: 118 + 112; the prompt sent while the agent works stays in the turn.
    expect(tally).toMatchObject({
      isWaiting: false,
      output: 8,
      start: Temporal.Instant.from("2026-10-13T09:00:00Z").epochMilliseconds,
      total: 230,
    });
  });

  it("stops the turn's clock at the agent's last answer, and restarts it at the next prompt", () => {
    const ended = addTokens(
      EMPTY_TALLY,
      `${prompt("2026-10-13T08:00:00Z")}\n${line("a", 5)}\n{"timestamp": "2026-10-13T08:03:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}\n`,
    );

    expect(ended).toMatchObject({
      end: Temporal.Instant.from("2026-10-13T08:03:00Z").epochMilliseconds,
      isWaiting: true,
    });

    expect(addTokens(ended, `${prompt("2026-10-13T09:00:00Z")}\n`)).toMatchObject({
      end: 0,
      isWaiting: false,
    });
  });

  it("resumes the turn, its clock and tokens running, when the agent answers a notification after its end", () => {
    const ended = addTokens(
      EMPTY_TALLY,
      `${prompt("2026-10-13T08:00:00Z")}\n${line("a", 5)}\n{"timestamp": "2026-10-13T08:03:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}\n`,
    );
    const notification =
      '{"type": "user", "promptSource": "system", "message": {"content": "done"}}';
    const answer = `{"type": "assistant", "message": {"id": "b", "usage": {"input_tokens": 1, "output_tokens": 7}}}`;

    expect(addTokens(ended, `${notification}\n${answer}\n`)).toMatchObject({
      end: 0,
      isWaiting: false,
      output: 12,
      start: Temporal.Instant.from("2026-10-13T08:00:00Z").epochMilliseconds,
    });
  });

  it("does not start a turn at a compaction summary nor at a task notification", () => {
    const summary = '{"type": "user", "isCompactSummary": true, "message": {"content": "summary"}}';
    const notification =
      '{"type": "user", "promptSource": "system", "message": {"content": "done"}}';

    expect(addTokens(EMPTY_TALLY, `${END}\n${summary}\n${notification}\n`)).toMatchObject({
      isWaiting: true,
      start: 0,
    });
  });

  it("waits for background tasks after the end of the turn, the person's turn coming once they end", () => {
    const launch =
      '{"type": "assistant", "timestamp": "2026-10-13T08:01:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {}}, {"type": "tool_use", "id": "toolu_B", "name": "Bash", "input": {"run_in_background": true}}, {"type": "tool_use", "id": "toolu_C", "name": "Bash", "input": {}}]}}';
    const results =
      '{"type": "user", "message": {"content": [{"type": "tool_result", "tool_use_id": "toolu_A"}, {"type": "tool_result", "tool_use_id": "toolu_C"}]}}';
    const ended = addTokens(
      EMPTY_TALLY,
      `${prompt("2026-10-13T08:00:00Z")}\n${launch}\n${results}\n{"timestamp": "2026-10-13T08:02:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}\n`,
    );

    // A subagent and a command in the background run on; their immediate results end nothing.
    expect(ended.pending.map((task) => task.id)).toEqual(["toolu_A", "toolu_B"]);
    expect(ended.finished).toEqual(["toolu_A", "toolu_C"]);
    expect(isTurnDone(ended)).toBe(false);
    expect(statusText({ ...STATUS, tally: ended })).toContain("waiting for 2 background tasks");

    const notified = addTokens(
      ended,
      '{"type": "queue-operation", "timestamp": "2026-10-13T08:30:00Z", "content": "<task-notification>\\n<tool-use-id>toolu_A</tool-use-id>\\n<tool-use-id>toolu_B</tool-use-id>"}\n',
    );

    expect(notified.pending).toEqual([]);
    expect(isTurnDone(notified)).toBe(true);
  });

  it("drops a background task whose end never came after three hours", () => {
    const launch =
      '{"type": "assistant", "timestamp": "2026-10-13T08:00:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {}}]}}';

    expect(
      addTokens(EMPTY_TALLY, `${launch}\n${prompt("2026-10-13T11:00:01Z")}\n`).pending,
    ).toEqual([]);

    expect(
      addTokens(EMPTY_TALLY, `${launch}\n${prompt("2026-10-13T10:59:59Z")}\n`).pending,
    ).toHaveLength(1);
  });

  it("keeps a cached tally only for the same transcript", () => {
    const cache: unknown = JSON.parse(
      '{"transcript": "t.jsonl", "tally": {"end": 7, "isWaiting": false, "lastId": "a", "offset": 9, "output": 2, "pending": [{"id": "toolu_A", "start": 4}], "start": 5, "total": 3}}',
    );

    expect(cachedTally(cache, "t.jsonl")).toEqual({
      end: 7,
      finished: [],
      isWaiting: false,
      lastId: "a",
      offset: 9,
      output: 2,
      pending: [{ id: "toolu_A", start: 4 }],
      start: 5,
      total: 3,
    });

    expect(cachedTally(cache, "other.jsonl")).toEqual(EMPTY_TALLY);
  });

  it("writes durations in seconds, minutes, then hours", () => {
    expect(shortDuration(45_000)).toBe("45 s");
    expect(shortDuration(12 * 60_000)).toBe("12 min");
    expect(shortDuration(125 * 60_000)).toBe("2 h 05");
  });

  it("writes the turn's clock to the second", () => {
    expect(clockDuration(45_900)).toBe("45 s");
    expect(clockDuration(12 * 60_000 + 5000)).toBe("12 min 05 s");
    expect(clockDuration(125 * 60_000 + 9000)).toBe("2 h 05 min 09 s");
  });

  it("shows the place and step, the turn, and the call running", () => {
    const base = {
      contextPercent: 34.4,
      durationMs: 60_000,
      isRunningLong: false,
      model: "Opus 5.5",
      place: "E01 › RET-001 retrospective e01",
      running: "",
      tally: { ...EMPTY_TALLY, output: 1234, total: 5_678_901 },
    };
    const active = statusText({ ...base, silentMs: 1000, step: "RET-001 · metrics" });
    const silent = statusText({
      ...base,
      isRunningLong: true,
      running: "▶ Bash · Run the checks — 12 s",
      silentMs: 180_000,
      step: "",
    });

    expect(active).toContain("E01 › RET-001 retrospective e01");
    expect(active).toContain("RET-001 · metrics");
    expect(active).toContain("● active");
    expect(active).toContain("⏱ turn 1 min 00 s");
    expect(active).toContain("5,678,901 tokens this turn (1,234 written)");
    expect(active).toContain("conversation memory 34 % full");
    expect(active.split("\n")).toHaveLength(2);
    expect(silent).toContain("waiting");
    expect(silent).toContain("◌ silent for 3 min");

    expect(
      statusText({
        ...base,
        silentMs: 1000,
        step: "",
        tally: { ...base.tally, isWaiting: true, start: 1 },
      }),
    ).toContain("◌ your turn");

    expect(
      statusText({
        ...base,
        silentMs: 1000,
        step: "",
        tally: { ...base.tally, isWaiting: true, start: 1 },
      }),
    ).toContain("⏱ last turn 1 min 00 s");

    expect(silent.split("\n", 3)[2]).toBe("\u{1B}[31m▶ Bash · Run the checks — 12 s\u{1B}[0m");
  });
});
