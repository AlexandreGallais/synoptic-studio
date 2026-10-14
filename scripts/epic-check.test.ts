import { describe, expect, it } from "vitest";

import { failedCalls, leftBranches } from "./epic-check";

describe("epic-check", () => {
  it("counts the calls failed since the epic's first day, the frequent errors first", () => {
    const records = [
      { error: "", time: "2026-10-12T10:00:00Z", tool: "Bash" },
      { error: "Exit code 1", time: "2026-10-13T10:00:00Z", tool: "Bash" },
      { error: "Exit code 1", time: "2026-10-13T11:00:00Z", tool: "Bash" },
      { error: "File not found", time: "2026-10-14T10:00:00Z", tool: "Read" },
      { error: "", time: "2026-10-14T11:00:00Z", tool: "Bash" },
    ];

    // 3 of the 4 calls since 2026-10-13 failed: far over 2 %.
    expect(failedCalls(records, "2026-10-13")).toEqual({
      detail: "3 of 4; 2 × Bash: Exit code 1; 1 × Read: File not found",
      isOk: false,
      what: "calls failed during the epic (2 % at most)",
    });

    expect(failedCalls([], "2026-10-13").isOk).toBe(true);
  });

  it("lists the branches left besides main, develop and the one checked out", () => {
    const branches = [
      "develop",
      "main",
      "docs/ret-002",
      "origin/docs/ret-002",
      "origin/HEAD",
      "feat/old",
    ];

    expect(leftBranches(branches, "docs/ret-002")).toEqual({
      detail: "feat/old",
      isOk: false,
      what: "branches left",
    });
  });
});
