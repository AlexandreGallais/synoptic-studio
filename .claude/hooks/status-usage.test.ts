import { describe, expect, it } from "vitest";

import { usageSnapshot } from "./status-usage";

// The status line's input is external JSON (snake_case keys), written here as Claude Code sends it.

/**
 * The status line's input, parsed.
 *
 * @param json - its text
 * @returns the parsed value
 */
function input(json: string): unknown {
  return JSON.parse(json);
}

describe("usageSnapshot", () => {
  it("keeps the weekly and five-hour windows with their reset times", () => {
    const limits = input(
      `{"rate_limits": {"five_hour": {"used_percentage": 23.5, "resets_at": 1738425600}, "seven_day": {"used_percentage": 91, "resets_at": 1738857600}}}`,
    );

    expect(usageSnapshot(limits, 5)).toStrictEqual({
      fiveHour: { resetsAt: "2025-02-01T16:00:00Z", usedPercent: 23.5 },
      sevenDay: { resetsAt: "2025-02-06T16:00:00Z", usedPercent: 91 },
      time: 5,
    });
  });

  it("leaves a missing window out", () => {
    const limits = input(`{"rate_limits": {"seven_day": {"used_percentage": 40, "resets_at": 0}}}`);

    expect(usageSnapshot(limits, 5)).toStrictEqual({
      sevenDay: { resetsAt: "1970-01-01T00:00:00Z", usedPercent: 40 },
      time: 5,
    });
  });

  it("returns nothing without rate limits, so that the last snapshot is kept", () => {
    expect(usageSnapshot(input(`{"model": {"display_name": "Opus 5.5"}}`), 5)).toBeUndefined();
  });

  it("takes a window without its reset time for no window", () => {
    const limits = input(`{"rate_limits": {"five_hour": {"used_percentage": 10}}}`);

    expect(usageSnapshot(limits, 5)).toBeUndefined();
  });
});
