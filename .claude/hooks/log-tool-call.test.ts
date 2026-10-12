import { describe, expect, it } from "vitest";

import { toolCallRecord, toolSummary } from "./log-tool-call";

// Hook inputs are external JSON (snake_case keys): parsed as the hook parses them.

/** Fields of a hook input for a shell command that succeeded, as documented by `REF-CC-HOOKS`. */
const BASH = `"cwd": "/repo", "duration_ms": 4187, "session_id": "abc123", "tool_name": "Bash",
  "tool_input": {"command": "npm run check", "description": "Run the checks"},
  "tool_use_id": "toolu_01"`;

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

    expect(toolCallRecord(input, "2026-10-12T21:00:00Z", "develop")).toEqual({
      agent: "main",
      branch: "develop",
      description: "Run the checks",
      durationMs: 4187,
      error: "",
      isOk: true,
      session: "abc123",
      summary: "npm run check",
      time: "2026-10-12T21:00:00Z",
      tool: "Bash",
      toolUseId: "toolu_01",
    });
  });

  it("records a failure with the first line of its error, and the subagent that ran it", () => {
    const input = hookInput(
      String.raw`${BASH}, "hook_event_name": "PostToolUseFailure", "agent_type": "auditor",
      "error": "Exit code 1\nError: Cannot find module"`,
    );

    expect(toolCallRecord(input, "t", "b")).toMatchObject({
      agent: "auditor",
      error: "Exit code 1",
      isOk: false,
    });
  });

  it("leaves out a missing duration", () => {
    const input = hookInput('"tool_name": "Bash", "tool_input": {"command": "ls"}');

    expect(toolCallRecord(input, "t", "b").durationMs).toBeUndefined();
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
