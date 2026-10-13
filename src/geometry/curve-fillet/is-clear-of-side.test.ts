import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";
import { orientedSides } from "../fillet-side";

import { isClearOfSide } from "./is-clear-of-side";

describe("isClearOfSide (ADR-0047)", () => {
  it("keeps the fillet's circle outside the side, and refuses one the side runs through", () => {
    const [, after] = orientedSides(...LINE_AND_THIRDS);

    expect(isClearOfSide(after, { x: 90, y: 10 }, 10)).toBe(true);
    expect(isClearOfSide(after, { x: 95, y: 10 }, 10)).toBe(false);
  });

  test.prop({ radius: fc.integer({ max: 50, min: 1 }) })(
    "takes any circle tangent to the straight side from the outside",
    ({ radius }) => {
      const [, after] = orientedSides(...LINE_AND_THIRDS);

      expect(isClearOfSide(after, { x: 100 - radius, y: 25 }, radius)).toBe(true);
    },
  );
});
