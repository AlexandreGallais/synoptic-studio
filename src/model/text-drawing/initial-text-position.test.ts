import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { initialTextPosition } from "./initial-text-position";

describe("initialTextPosition (REF-SVG2-TEXT §11.10.1.1)", () => {
  it("writes a left-to-right line at its left edge", () => {
    expect(initialTextPosition(2.66015625, 36.4609375, "ltr")).toBe(2.66015625);
  });

  it("writes a right-to-left line at its right edge, where its start is drawn", () => {
    // 2.66015625 + 36.4609375 = 39.12109375.
    expect(initialTextPosition(2.66015625, 36.4609375, "rtl")).toBe(39.12109375);
  });

  it("writes an empty line at its left edge either way", () => {
    expect(initialTextPosition(10, 0, "rtl")).toBe(10);
  });

  const coordinate = fc.double({ max: 100_000, min: -100_000, noNaN: true });

  test.prop({ left: coordinate, width: coordinate })(
    "puts a right-to-left line one width to the right of a left-to-right one",
    ({ left, width }) => {
      expect(
        initialTextPosition(left, width, "rtl") - initialTextPosition(left, width, "ltr"),
      ).toBeCloseTo(width, 6);
    },
  );
});
