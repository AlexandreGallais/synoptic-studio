import { describe, expect, it } from "vitest";

import { lastLine, watchStep } from "./watch-task";

/** Stall after 120 s; the watch ends on a line starting with "deployments: none". */
const RULES = { stallMs: 120_000, until: /^deployments: none/u };

/** State after a first look at 0 s, 10 bytes read. */
const FRESH = { changedAt: 0, isStalled: false, size: 10 };

describe("watchdog of a background task", () => {
  it("reads the last non-empty line", () => {
    expect(lastLine("a\nb\n\n")).toBe("b");
    expect(lastLine("")).toBe("");
  });

  it("stays quiet while the output grows", () => {
    const step = watchStep(FRESH, { now: 5000, text: "runs: batch 1\n" }, RULES);

    expect(step).toEqual({
      isDone: false,
      message: "",
      watch: { changedAt: 5000, isStalled: false, size: 14 },
    });
  });

  it("reports a stall once, then the output coming back", () => {
    const text = "0123456789";
    const stalled = watchStep(FRESH, { now: 125_000, text }, RULES);
    const again = watchStep(stalled.watch, { now: 130_000, text }, RULES);
    const back = watchStep(again.watch, { now: 135_000, text: `${text}\nbatch 2\n` }, RULES);

    expect(stalled.message).toBe("stalled: no output for 125 s — last line: 0123456789");
    expect(again.message).toBe("");
    expect(back.message).toBe("resumed: batch 2");
  });

  it("ends on any new line matching, or when the task exited", () => {
    const text = "0123456789\ndeployments: none left\n\n[exited with code 0]\n";
    const exited = watchStep(
      FRESH,
      { now: 5000, text: "0123456789\n[exited with code 1]\n" },
      RULES,
    );

    expect(watchStep(FRESH, { now: 5000, text }, RULES).message).toBe(
      "done: deployments: none left",
    );

    expect(exited.message).toBe("done: [exited with code 1]");
  });

  it("ends on the end pattern", () => {
    const step = watchStep(
      FRESH,
      { now: 5000, text: "0123456789\ndeployments: none left\n" },
      RULES,
    );

    expect(step).toMatchObject({ isDone: true, message: "done: deployments: none left" });
  });
});
