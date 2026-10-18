import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { alignedOffset } from "./aligned-offset";

describe("alignedOffset (DERIV-regular-polygon-fit step 5)", () => {
  it.each([
    // A room of 40: none before at min, 20 at mid, 40 at max.
    { alignment: "min" as const, expected: 0, room: 40 },
    { alignment: "mid" as const, expected: 20, room: 40 },
    { alignment: "max" as const, expected: 40, room: 40 },
    // A span 10 longer than its room: half of the overflow before it at mid.
    { alignment: "mid" as const, expected: -5, room: -10 },
  ])("places a room of $room at $alignment $expected from its start", (row) => {
    expect(alignedOffset(row.room, row.alignment)).toBe(row.expected);
  });

  test.prop({ room: fc.integer({ max: 10_000, min: -10_000 }) })(
    "leaves at mid the same room on both sides",
    ({ room }) => {
      const before = alignedOffset(room, "mid");

      // Before the span at mid, and after it: room − before.
      expect(before).toBe(room - before);
      expect(alignedOffset(room, "min") + alignedOffset(room, "max")).toBe(room);
    },
  );
});
