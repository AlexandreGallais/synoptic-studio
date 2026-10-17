import { describe, expect, it } from "vitest";

import { leftBranches } from "./epic-check";
import { checkAllFinding, failedCalls, failingChecks, lostCalls } from "./epic-check-findings";

/** One hour and one second, in milliseconds: a running call's file older than an hour. */
const OVER_AN_HOUR = 3_601_000;

/**
 * Calls of one day, the first one failed.
 *
 * @param count - how many calls
 * @returns the records of the metrics log
 */
function oneFailedOf(count: number): readonly { error: string; time: string; tool: string }[] {
  return [
    { error: "Exit code 1", time: "2026-10-13T10:00:00Z", tool: "Bash" },
    ...Array.from({ length: count - 1 }, () => ({
      error: "",
      time: "2026-10-13T10:00:00Z",
      tool: "Bash",
    })),
  ];
}

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
      detail:
        "3 of 4; 2 × Bash: Exit code 1; 1 × Read: File not found; apart: 0 refused by the worktree isolation",
      isOk: false,
      what: "calls failed during the epic (2 % at most)",
    });

    expect(failedCalls([], "2026-10-13").isOk).toBe(true);
  });

  it("[F18.AC11] counts the worktree isolation's refusals apart from the failed calls", () => {
    const refusal = "This agent is isolated in the worktree /repo/.claude/worktrees/us-061";
    const records = [
      ...Array.from({ length: 60 }, () => ({
        error: "",
        time: "2026-10-13T10:00:00Z",
        tool: "Bash",
      })),
      { error: refusal, time: "2026-10-13T10:00:00Z", tool: "Bash" },
      { error: refusal, time: "2026-10-13T11:00:00Z", tool: "Monitor" },
      { error: "Exit code 1", time: "2026-10-13T12:00:00Z", tool: "Bash" },
    ];

    // 1 failed of 61 calls, the 2 refusals left out of both: 1.6 %, within 2 %.
    expect(failedCalls(records, "2026-10-13")).toEqual({
      detail: "1 of 61; 1 × Bash: Exit code 1; apart: 2 refused by the worktree isolation",
      isOk: true,
      what: "calls failed during the epic (2 % at most)",
    });
  });

  it("accepts exactly 2 % of failed calls, and a call exactly an hour old as not yet lost", () => {
    // 1 of 50 is 2 %, the limit itself: accepted; 1 of 49 is 2.04 %: over it.
    expect(failedCalls(oneFailedOf(50), "2026-10-13").isOk).toBe(true);
    expect(failedCalls(oneFailedOf(49), "2026-10-13").isOk).toBe(false);

    // 3 600 000 ms is one hour exactly: not over an hour, so not lost.
    const anHour = { ageMs: 3_600_000, record: { start: 0, summary: "ls", tool: "Bash" } };

    expect(lostCalls([anHour]).detail).toBe("none");
  });

  it("[F18.AC11] names each failing check, its lane and its failing tests", () => {
    const output = [
      "✓ lint 23.9 s",
      "✓ format:check 2.0 s",
      "✗ test 26.2 s",
      "",
      " FAIL  scripts/epic-check.test.ts > epic-check > lists the branches",
      "AssertionError: expected 1 to be 2",
      "\u{1B}[31m FAIL \u{1B}[39m src/math/unit.test.ts > unit > keeps a zero vector",
      " FAIL  scripts/epic-check.test.ts > epic-check > lists the branches",
      "✗ check:rust 4.1 s",
      "test heavy_lock_waits ... FAILED",
      "test heavy_lock_takes ... ok",
      "✗ playground:build 3.0 s",
      "error during build",
      "check:all failed in 30.1 s: test, check:rust, playground:build",
    ].join("\n");

    expect(failingChecks(output)).toEqual([
      "test (lane test): scripts/epic-check.test.ts > epic-check > lists the branches, src/math/unit.test.ts > unit > keeps a zero vector",
      "check:rust (lane check:rust): heavy_lock_waits",
      "playground:build (lane docs:build → playground:build)",
    ]);
  });

  it("[F18.AC11] puts the failing checks in check:all's line, its last line when it passed", () => {
    expect(checkAllFinding({ isOk: false, out: "✗ lint 1.0 s\nerror\ncheck:all failed" })).toEqual({
      detail: "lint (lane lint)",
      isOk: false,
      what: "check:all (code, site, playground, dependencies)",
    });

    // No check named (the lock failed before any check ran): the output's end.
    expect(checkAllFinding({ isOk: false, out: "heavy lock: cannot start" }).detail).toBe(
      "heavy lock: cannot start",
    );

    expect(checkAllFinding({ isOk: true, out: "✓ lint 1.0 s\ncheck:all passed in 1.0 s" })).toEqual(
      {
        detail: "check:all passed in 1.0 s",
        isOk: true,
        what: "check:all (code, site, playground, dependencies)",
      },
    );
  });

  it("names no check when check:all passed, and at most ten tests of one check", () => {
    expect(failingChecks("✓ lint 1.0 s\ncheck:all passed in 1.0 s")).toEqual([]);
    expect(failingChecks("")).toEqual([]);

    const many = Array.from(
      Array.from({ length: 12 }).keys(),
      (index) => ` FAIL  a.test.ts > t${String(index)}`,
    );

    expect(failingChecks(["✗ test 1.0 s", ...many].join("\n"))).toEqual([
      "test (lane test): a.test.ts > t0, a.test.ts > t1, a.test.ts > t2, a.test.ts > t3, a.test.ts > t4, a.test.ts > t5, a.test.ts > t6, a.test.ts > t7, a.test.ts > t8, a.test.ts > t9, 2 more",
    ]);
  });

  it("[F18.AC11] reports the calls started over an hour ago and never ended as lost", () => {
    const calls = [
      {
        ageMs: OVER_AN_HOUR,
        record: {
          agent: "main",
          start: 1_792_112_400_000,
          summary: "npm run check:all",
          tool: "Bash",
        },
      },
      { ageMs: 1000, record: { start: 1_792_190_000_000, summary: "ls", tool: "Bash" } },
      {
        ageMs: OVER_AN_HOUR * 2,
        record: { start: 1_792_108_800_000, summary: "docs/a.md", tool: "Read" },
      },
    ];

    // 2026-10-16T00:00Z is 20 742 days × 86 400 000 ms = 1 792 108 800 000 ms; one hour later, 3 600 000 ms more.
    expect(lostCalls(calls)).toEqual({
      detail: "2 lost: 2026-10-16T00:00 Read docs/a.md; 2026-10-16T01:00 Bash npm run check:all",
      isOk: true,
      what: "calls lost: started, never ended (a force-quit), over an hour old",
    });

    expect(lostCalls([]).detail).toBe("none");
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
