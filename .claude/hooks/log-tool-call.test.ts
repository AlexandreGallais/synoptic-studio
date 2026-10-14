import { describe, expect, it } from "vitest";

import { sessionMoment, toolCallRecord, toolSummary } from "./log-tool-call";

// Hook inputs are external JSON (snake_case keys): parsed as the hook parses them.

/** Fields of a hook input for a shell command that succeeded, as documented by `REF-CC-HOOKS`. */
const BASH = `"cwd": "/repo", "duration_ms": 4187, "session_id": "abc123", "tool_name": "Bash",
  "tool_input": {"command": "npm run check", "description": "Run the checks"},
  "tool_use_id": "toolu_01"`;

/** Where and when the call ends: the hook ran at 2026-10-12T21:00:00Z, no snapshot. */
const CONTEXT = {
  branch: "develop",
  snapshot: undefined,
  startedAt: 0,
  step: "",
  time: 1_791_838_800_000,
};

/**
 * Parsed hook input.
 *
 * @param fields - JSON members, without the braces
 * @returns the parsed object
 */
function hookInput(fields: string): unknown {
  return JSON.parse(`{${fields}}`);
}

describe("log-tool-call hook", () => {
  it("records a successful command with its duration", () => {
    const input = hookInput(`${BASH}, "hook_event_name": "PostToolUse"`);

    expect(toolCallRecord(input, CONTEXT)).toEqual({
      ...sessionMoment(undefined, 0),
      agent: "main",
      agentId: "",
      branch: "develop",
      description: "Run the checks",
      durationMs: 4187,
      error: "",
      // {"command":"npm run check","description":"Run the checks"}: 58 characters.
      inputChars: 58,
      isBackground: false,
      isOk: true,
      outputChars: 0,
      permissionMode: "",
      session: "abc123",
      startedAt: 0,
      step: "",
      summary: "npm run check",
      time: "2026-10-12T21:00:00Z",
      tool: "Bash",
      toolUseId: "toolu_01",
    });
  });

  it("copies the session's state from the status line's snapshot, its age and the turn's time", () => {
    const snapshot = {
      contextPercent: 42,
      model: "Opus 5.5",
      pendingAgents: 1,
      pendingCommands: 2,
      time: 9000,
      turnOutputTokens: 300,
      turnStart: 4000,
      turnTokens: 5000,
    };

    expect(sessionMoment(snapshot, 10_000)).toEqual({
      contextPercent: 42,
      model: "Opus 5.5",
      pendingAgents: 1,
      pendingCommands: 2,
      snapshotAgeMs: 1000,
      turnElapsedMs: 6000,
      turnOutputTokens: 300,
      turnStart: 4000,
      turnTokens: 5000,
    });

    expect(sessionMoment(undefined, 10_000)).toMatchObject({ snapshotAgeMs: 0, turnElapsedMs: 0 });
  });

  it("records a call in the background, its start, step, result size and subagent id", () => {
    const input = hookInput(
      `"tool_name": "Agent", "agent_id": "ag1", "permission_mode": "auto",
      "tool_input": {"subagent_type": "auditor"}, "tool_response": "ok"`,
    );

    expect(toolCallRecord(input, { ...CONTEXT, startedAt: 7, step: "RET-002" })).toMatchObject({
      agentId: "ag1",
      isBackground: true,
      // "ok" written as JSON: 4 characters.
      outputChars: 4,
      permissionMode: "auto",
      startedAt: 7,
      step: "RET-002",
    });
  });

  it("records a failure with the first line of its error, and the subagent that ran it", () => {
    const input = hookInput(
      String.raw`${BASH}, "hook_event_name": "PostToolUseFailure", "agent_type": "auditor",
      "error": "Exit code 1\nError: Cannot find module"`,
    );

    expect(toolCallRecord(input, CONTEXT)).toMatchObject({
      agent: "auditor",
      error: "Exit code 1",
      isOk: false,
    });
  });

  it("leaves out a missing duration", () => {
    const input = hookInput('"tool_name": "Bash", "tool_input": {"command": "ls"}');

    expect(toolCallRecord(input, CONTEXT).durationMs).toBeUndefined();
  });

  it("summarizes each kind of tool by what it acted on", () => {
    expect(toolSummary("Read", hookInput('"file_path": "/repo/src/a.ts"'), "/repo")).toBe(
      "src/a.ts",
    );

    expect(toolSummary("Grep", hookInput('"pattern": "foo"'), "/repo")).toBe("foo");

    expect(toolSummary("WebFetch", hookInput('"url": "https://x.org"'), "/repo")).toBe(
      "https://x.org",
    );

    expect(
      toolSummary("Agent", hookInput('"description": "audit", "subagent_type": "auditor"'), ""),
    ).toBe("auditor: audit");

    expect(toolSummary("Other", hookInput(""), "/repo")).toBe("");
  });

  it("cuts a long command and keeps its length", () => {
    const summary = toolSummary("Bash", hookInput(`"command": "echo ${"a".repeat(400)}"`), "/repo");

    expect(summary).toMatch(/^echo a{295}… \(405 characters\)$/u);
  });
});
