import { describe, expect, it } from "vitest";

import { shortDuration, statusText } from "./status-line";
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

  it("does not start a turn at the summary written when the memory is compacted", () => {
    const summary = '{"type": "user", "isCompactSummary": true, "message": {"content": "summary"}}';

    expect(addTokens(EMPTY_TALLY, `${END}\n${summary}\n`)).toMatchObject({
      isWaiting: true,
      start: 0,
    });
  });

  it("keeps a cached tally only for the same transcript", () => {
    const cache: unknown = JSON.parse(
      '{"transcript": "t.jsonl", "tally": {"isWaiting": false, "lastId": "a", "offset": 9, "output": 2, "start": 5, "total": 3}}',
    );

    expect(cachedTally(cache, "t.jsonl")).toEqual({
      isWaiting: false,
      lastId: "a",
      offset: 9,
      output: 2,
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

  it("shows the place and step, the turn, and the call running", () => {
    const base = {
      contextPercent: 34.4,
      durationMs: 60_000,
      model: "Opus 5.5",
      place: "E01 › RET-001 retrospective e01",
      running: "",
      tally: { ...EMPTY_TALLY, output: 1234, total: 5_678_901 },
    };
    const active = statusText({ ...base, silentMs: 1000, step: "RET-001 · metrics" });
    const silent = statusText({
      ...base,
      running: "▶ Bash · Run the checks — 12 s",
      silentMs: 180_000,
      step: "",
    });

    expect(active).toContain("E01 › RET-001 retrospective e01");
    expect(active).toContain("RET-001 · metrics");
    expect(active).toContain("● active");
    expect(active).toContain("⏱ turn 1 min");
    expect(active).toContain("5,678,901 tokens this turn (1,234 written)");
    expect(active).toContain("conversation memory 34 % full");
    expect(active.split("\n")).toHaveLength(2);
    expect(silent).toContain("waiting");
    expect(silent).toContain("◌ silent for 3 min");
    expect(silent.split("\n", 3)[2]).toContain("▶ Bash · Run the checks — 12 s");
  });
});
