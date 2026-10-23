import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shownHistoryWindow } from "./shown-history-window";

describe("shownHistoryWindow (US-077)", () => {
  it.each([
    // 14 rows beside the present, at most 7 after it: 7 asked, the 3 past states leave 11 after.
    { after: 11, before: 3, future: 20, past: 3, rows: 15 },
    // Nothing to come: 14 before.
    { after: 0, before: 14, future: 0, past: 100, rows: 15 },
    // 3 to come, all shown: 11 before.
    { after: 3, before: 11, future: 3, past: 100, rows: 15 },
    // Both sides long: 7 and 7.
    { after: 7, before: 7, future: 100, past: 100, rows: 15 },
    // 13 rows beside the present: at most 6 after, 7 before.
    { after: 6, before: 7, future: 100, past: 100, rows: 14 },
    // An empty history: the present alone.
    { after: 0, before: 0, future: 0, past: 0, rows: 15 },
    // One row, or none: the present alone.
    { after: 0, before: 0, future: 5, past: 5, rows: 1 },
    { after: 0, before: 0, future: 5, past: 5, rows: 0 },
  ])(
    "[F41.AC1] shows $before past and $after to come of $past and $future in $rows rows",
    ({ after, before, future, past, rows }) => {
      expect(shownHistoryWindow(past, future, rows)).toEqual({ after, before });
    },
  );

  test.prop({
    future: fc.nat({ max: 50 }),
    past: fc.nat({ max: 50 }),
    rows: fc.nat({ max: 30 }),
  })(
    "[F41.AC1] shows as many states as fit beside the present, at most half the rows after it",
    ({ future, past, rows }) => {
      const { after, before } = shownHistoryWindow(past, future, rows);
      const beside = Math.max(rows - 1, 0);

      expect(before).toBeLessThanOrEqual(past);
      expect(after).toBeLessThanOrEqual(future);
      expect(before + after).toBe(Math.min(beside, past + future));
      expect(after).toBeGreaterThanOrEqual(Math.min(future, Math.floor(beside / 2)));
    },
  );
});
