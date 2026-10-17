import { describe, expect, it } from "vitest";

import {
  backgroundText,
  clockDuration,
  isTurnDone,
  sessionSnapshot,
  shortDuration,
  statusText,
} from "./status-line";
import { backgroundLines } from "./status-place";
import { isRunOpen } from "./status-run";
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

/**
 * A `tool_use` block of an `Agent` call.
 *
 * @param id - the call's id, also its description
 * @returns one JSON block
 */
function agent(id: string): string {
  return `{"type": "tool_use", "id": "${id}", "name": "Agent", "input": {"subagent_type": "Explore", "description": "${id}"}}`;
}

/** What the status line shows, before a test changes some of it. */
const STATUS = {
  background: [],
  contextPercent: 10,
  durationMs: 60_000,
  isRunningLong: false,
  model: "Opus 5.5",
  place: "E14 › F12",
  run: "",
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
      '{"type": "user", "message": {"content": [{"type": "tool_result", "tool_use_id": "toolu_A", "content": [{"type": "text", "text": "Async agent launched successfully."}]}, {"type": "tool_result", "tool_use_id": "toolu_C"}]}}';
    const ended = addTokens(
      EMPTY_TALLY,
      `${prompt("2026-10-13T08:00:00Z")}\n${launch}\n${results}\n{"timestamp": "2026-10-13T08:02:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}\n`,
    );

    // A subagent and a command in the background run on; their immediate results end nothing.
    expect(ended.pending.map((task) => task.id)).toEqual(["toolu_A", "toolu_B"]);
    expect(ended.finished).toEqual(["toolu_A", "toolu_C"]);
    expect(isTurnDone(ended)).toBe(false);

    expect(statusText({ ...STATUS, tally: ended })).toContain(
      "◌ chat free · 1 agent, 1 command in the background",
    );

    const notified = addTokens(
      ended,
      '{"type": "queue-operation", "timestamp": "2026-10-13T08:30:00Z", "content": "<task-notification>\\n<tool-use-id>toolu_A</tool-use-id>\\n<tool-use-id>toolu_B</tool-use-id>"}\n',
    );

    expect(notified.pending).toEqual([]);
    expect(isTurnDone(notified)).toBe(true);
  });

  it("[F18.AC2] ends a subagent run in the foreground with its result, never one launched in the background", () => {
    const launch = `{"type": "assistant", "timestamp": "2026-10-17T01:00:00Z", "message": {"id": "a", "content": [${["fg", "bg", "text", "flag"].map((id) => agent(id)).join(", ")}, {"type": "tool_use", "id": "cmd", "name": "Bash", "input": {"command": "npm run test", "run_in_background": true}}]}}`;
    // A foreground subagent's result: `status: "completed"` (the hooks reference), never notified.
    const completed =
      '{"type": "user", "message": {"content": [{"tool_use_id": "fg", "type": "tool_result", "content": [{"type": "text", "text": "The report."}]}]}, "toolUseResult": {"status": "completed", "content": []}}';
    // As the transcripts write a launch in the background (2026-10-09): text and flags.
    const launched =
      '{"type": "user", "message": {"content": [{"tool_use_id": "bg", "type": "tool_result", "content": [{"type": "text", "text": "Async agent launched successfully. (This tool result is internal metadata.)"}]}]}, "toolUseResult": {"isAsync": true, "status": "async_launched"}}';
    // The launch text without `toolUseResult` (a line of 2026-10-16), and the flags without the text.
    const textOnly =
      '{"type": "user", "message": {"content": [{"tool_use_id": "text", "type": "tool_result", "content": [{"type": "text", "text": "Async agent launched successfully."}]}]}}';
    const flagsOnly =
      '{"type": "user", "message": {"content": [{"tool_use_id": "flag", "type": "tool_result", "content": "started"}]}, "toolUseResult": {"isAsync": true}}';
    // A command in the background answers at once too: only its notification ends it.
    const command =
      '{"type": "user", "message": {"content": [{"tool_use_id": "cmd", "type": "tool_result", "content": "Command running in background with ID: b1"}]}}';
    const tally = addTokens(
      EMPTY_TALLY,
      `${[launch, completed, launched, textOnly, flagsOnly, command].join("\n")}\n`,
    );

    expect(tally.pending.map((task) => task.id)).toEqual(["bg", "text", "flag", "cmd"]);
  });

  it("ends a background call answered with an error: refused before it started, it never notifies", () => {
    // The line that kept the status line waiting for three hours (2026-10-14): a push in the
    // background refused because the safety check gave no verdict.
    const launch =
      '{"type": "assistant", "timestamp": "2026-10-14T16:33:05Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_P", "name": "Bash", "input": {"command": "git push", "description": "Push", "run_in_background": true}}]}}';
    const refused =
      '{"type": "user", "message": {"content": [{"type": "tool_result", "is_error": true, "tool_use_id": "toolu_P", "content": "The server-side auto mode classifier gave no verdict"}]}}';
    const started = addTokens(EMPTY_TALLY, `${prompt("2026-10-14T16:33:00Z")}\n${launch}\n`);

    expect(started.pending).toEqual([
      { id: "toolu_P", kind: "command", label: "Push", start: 1_791_995_585_000 },
    ]);

    expect(addTokens(started, `${refused}\n${END}\n`).pending).toEqual([]);
  });

  it("[F18.AC9] counts a subagent resumed by a message until its notification, a queued message as nothing", () => {
    // The lines that showed "your turn" for 15 minutes while a worker ran (2026-10-15, 12:07 to
    // 12:22 UTC): a message resumed the stopped EN-039 worker, whose end was notified under the
    // message's call id; no Agent call started it, so the status line saw nothing running.
    const send =
      '{"type": "assistant", "timestamp": "2026-10-15T12:07:38Z", "message": {"id": "m", "content": [{"type": "tool_use", "id": "toolu_013f", "name": "SendMessage", "input": {"to": "a93498ccc604c403b", "summary": "EN-039 audit findings to fix", "message": "EN-039 audit findings: fix …"}}]}}';
    const resumed = String.raw`{"type": "user", "timestamp": "2026-10-15T12:07:40Z", "message": {"content": [{"tool_use_id": "toolu_013f", "type": "tool_result", "content": [{"type": "text", "text": "{\"success\":true,\"message\":\"Resuming agent a93498c\",\"resumedAgentId\":\"a93498ccc604c403b\"}"}]}]}}`;
    const end =
      '{"timestamp": "2026-10-15T12:07:46Z", "message": {"id": "z", "stop_reason": "end_turn"}}';
    const running = addTokens(
      EMPTY_TALLY,
      `${prompt("2026-10-15T12:07:30Z")}\n${send}\n${resumed}\n${end}\n`,
    );

    expect(running.pending).toEqual([
      {
        id: "toolu_013f",
        kind: "agent",
        label: "resumed: EN-039 audit findings to fix",
        start: 1_792_066_058_000,
      },
    ]);

    expect(isTurnDone(running)).toBe(false);

    const notified = addTokens(
      running,
      '{"type": "queue-operation", "timestamp": "2026-10-15T12:22:26Z", "content": "<task-notification>\\n<task-id>a93498ccc604c403b</task-id>\\n<tool-use-id>toolu_013f</tool-use-id>\\n<status>completed</status>"}\n',
    );

    expect(notified.pending).toEqual([]);
    expect(isTurnDone(notified)).toBe(true);

    // 13:54:35 UTC the same day: a message to a subagent still running is only queued.
    const queued = send.replaceAll("toolu_013f", "toolu_01Wc");
    const queuedResult = String.raw`{"type": "user", "message": {"content": [{"tool_use_id": "toolu_01Wc", "type": "tool_result", "content": [{"type": "text", "text": "{\"success\":true,\"message\":\"Message queued for delivery to a6ab5a436c6a3dc67 at its next tool round.\"}"}]}]}}`;
    const asked = addTokens(EMPTY_TALLY, `${queued}\n`);

    expect(asked.pending.map((task) => task.kind)).toEqual(["message"]);
    expect(addTokens(asked, `${queuedResult}\n`).pending).toEqual([]);
  });

  it("counts a monitor and a subagent as background tasks, with what they do", () => {
    const launch =
      '{"type": "assistant", "timestamp": "2026-10-14T16:00:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_M", "name": "Monitor", "input": {"description": "push F10", "command": "until …"}}, {"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {"subagent_type": "auditor", "description": "Review EN-030"}}, {"type": "tool_use", "id": "toolu_B", "name": "Bash", "input": {"command": "npm run check:all", "run_in_background": true}}]}}';
    const tally = addTokens(EMPTY_TALLY, `${launch}\n`);

    expect(tally.pending.map((task) => [task.kind, task.label])).toEqual([
      ["command", "push F10"],
      ["agent", "auditor: Review EN-030"],
      ["command", "npm run check:all"],
    ]);

    expect(backgroundText(tally)).toBe("1 agent, 2 commands");
    expect(backgroundText(EMPTY_TALLY)).toBe("");
  });

  it("[F18.AC9] shows a worker in the background and its duration, never waiting nor your turn", () => {
    const worker = {
      id: "w",
      kind: "agent",
      label: "resumed: EN-039 audit findings to fix",
    } as const;
    const tally = {
      ...EMPTY_TALLY,
      pending: [
        { ...worker, start: 0 },
        { id: "m", kind: "message", label: "resumed: other", start: 0 },
      ],
      start: 1,
    } as const;
    const background = backgroundLines(tally.pending, [], {
      duration: shortDuration,
      now: 900_000,
    });
    const text = statusText({ ...STATUS, background, tally });

    // 900 000 ms = 15 min, the time the resumed EN-039 worker ran; a message still unanswered
    // is neither a line nor counted.
    expect(text.split("\n")).toEqual([
      "\u{1B}[1mE14 › F12\u{1B}[0m",
      expect.stringContaining("◌ chat free · 1 agent in the background"),
      "\u{1B}[2m↳ agent resumed: EN-039 audit findings to fix — 15 min\u{1B}[0m",
    ]);

    expect(text).not.toMatch(/waiting|your turn/u);

    expect(sessionSnapshot({ ...STATUS, background, tally }, 0)).toMatchObject({
      pendingAgents: 1,
      pendingCommands: 0,
    });
  });

  it("shows each background task, a subagent with the call it makes now", () => {
    const tasks = [
      { id: "a", kind: "agent", label: "auditor: Review EN-030", start: 0 },
      { id: "b", kind: "command", label: "Push", start: 60_000 },
    ] as const;
    const call = { description: "", start: 0, summary: "", tool: "Read" };
    const calls = [
      { ...call, agent: "auditor", start: 100, summary: "src/a.ts" },
      { ...call, agent: "auditor", description: "List the files", start: 200, tool: "Bash" },
      { ...call, agent: "main", start: 300, tool: "Edit" },
    ];

    expect(backgroundLines(tasks, calls, { duration: shortDuration, now: 240_000 })).toEqual([
      "↳ agent auditor: Review EN-030 — 4 min · now Bash · List the files",
      "↳ command Push — 3 min",
    ]);
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
      pending: [{ id: "toolu_A", kind: "command", label: "", start: 4 }],
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
      background: [],
      contextPercent: 34.4,
      durationMs: 60_000,
      isRunningLong: false,
      model: "Opus 5.5",
      place: "E01 › RET-001 retrospective e01",
      run: "",
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
    expect(active).toContain("● working");
    expect(active).toContain("⏱ turn 1 min 00 s");
    expect(active).toContain("5,678,901 tokens this turn (1,234 written)");
    expect(active).toContain("conversation memory 34 % full");
    expect(active.split("\n")).toHaveLength(2);
    expect(silent.split("\n", 1)[0]).toBe("\u{1B}[1mE01 › RET-001 retrospective e01\u{1B}[0m");
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

    expect(
      statusText({
        ...base,
        background: ["↳ command Push — 3 min"],
        silentMs: 1000,
        step: "",
        tally: {
          ...base.tally,
          isWaiting: false,
          pending: [{ id: "b", kind: "command", label: "Push", start: 0 }],
        },
      }).split("\n"),
    ).toEqual([
      expect.stringContaining("E01"),
      expect.stringContaining("● working · 1 command in the background"),
      "\u{1B}[2m↳ command Push — 3 min\u{1B}[0m",
    ]);

    expect(silent.split("\n", 3)[2]).toBe("\u{1B}[31m▶ Bash · Run the checks — 12 s\u{1B}[0m");
  });

  it("shows the open run beside the turn, and nothing of a run outside one (EN-092)", () => {
    const run = "run 2 h 05 · 43,100,000 tokens";
    const turn = statusText({ ...STATUS, run }).split("\n", 2)[1] ?? "";

    expect(turn).toContain(
      `tokens this turn (0 written)\u{1B}[2m · \u{1B}[0m\u{1B}[35m${run}\u{1B}[0m`,
    );

    expect(statusText(STATUS)).not.toContain("run ");
  });

  it("tells an open run's marker from a closed or missing one (EN-092)", () => {
    expect(isRunOpen('{"start":5,"end":0,"sessions":["a"]}')).toBe(true);
    expect(isRunOpen('{"start":5,"end":9,"sessions":["a"]}')).toBe(false);
    expect(isRunOpen('{"end":0}')).toBe(false);
    expect(isRunOpen("")).toBe(false);
  });
});
