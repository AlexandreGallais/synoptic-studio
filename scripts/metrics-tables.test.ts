import { describe, expect, it } from "vitest";

import { callStats, commandKey, formatDuration, markdownTable, median } from "./metrics-tables";

describe("commandKey", () => {
  it("names a command by its words, without directory change, variables, flags or quotes", () => {
    expect(commandKey('cd "/repo" && npm run check:all 2>&1 | tail')).toBe("npm run check:all");
    expect(commandKey("S=/tmp/x; git commit -q -m 'x'")).toBe("git commit");
    expect(commandKey("gh pr checks 12 --watch")).toBe("gh pr checks");
    expect(commandKey("npm install -D eslint")).toBe("npm install");
    expect(commandKey("grep -n foo bar.ts")).toBe("grep");
  });

  it("is empty when nothing runs", () => {
    expect(commandKey('cd "/repo"')).toBe("");
  });
});

describe("median", () => {
  it("takes the middle value, or the mean of the two middle ones", () => {
    expect(median([5, 1, 3])).toBe(3);
    // Sorted 1, 2, 4, 9: (2 + 4) / 2 = 3.
    expect(median([9, 1, 4, 2])).toBe(3);
    expect(median([])).toBe(0);
  });
});

describe("formatDuration", () => {
  it("writes milliseconds, seconds, then minutes and seconds", () => {
    expect(formatDuration(850)).toBe("850 ms");
    expect(formatDuration(12_400)).toBe("12.4 s");
    // 185 s = 3 min 5 s.
    expect(formatDuration(185_000)).toBe("3 min 05 s");
  });
});

describe("callStats", () => {
  it("counts, sums and ranks the calls of each group", () => {
    const calls = [
      { durationMs: 1000, isOk: true, summary: "npm run check", tool: "Bash" },
      { durationMs: 3000, isOk: false, summary: "npm run check", tool: "Bash" },
      { durationMs: 200, isOk: true, summary: "ls", tool: "Bash" },
    ];

    expect(callStats(calls, (call) => commandKey(call.summary))).toEqual([
      { count: 2, failures: 1, key: "npm run check", maxMs: 3000, medianMs: 2000, totalMs: 4000 },
      { count: 1, failures: 0, key: "ls", maxMs: 200, medianMs: 200, totalMs: 200 },
    ]);
  });
});

describe("markdownTable", () => {
  it("writes a header row, a separator and escapes pipes", () => {
    expect(markdownTable(["A", "B"], [["x | y", "1"]])).toBe(
      ["| A | B |", "| --- | --- |", String.raw`| x \| y | 1 |`].join("\n"),
    );
  });
});
