import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { heightFromBottom } from "./height-from-bottom";

describe("heightFromBottom (DERIV-color-spaces step 10)", () => {
  it("[F16.AC2] puts y = 25 at 75 above the bottom of a plane 100 tall, the ends swapped", () => {
    // 100 − 25 = 75; 100 − 0 = 100; 100 − 100 = 0.
    expect([
      heightFromBottom(25, 100),
      heightFromBottom(0, 100),
      heightFromBottom(100, 100),
    ]).toEqual([75, 100, 0]);
  });

  test.prop({ height: fc.integer({ max: 2000, min: 0 }), y: fc.integer({ max: 2000, min: 0 }) })(
    "gives y back when applied twice",
    ({ height, y }) => {
      expect(heightFromBottom(heightFromBottom(y, height), height)).toBe(y);
    },
  );
});
