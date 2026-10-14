import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { degreesInTurn } from "./degrees-in-turn";
import { explement } from "./explement";

describe("explement (REF-WIKI-ANGLE)", () => {
  it.each([
    { angle: 0, expected: 0 },
    { angle: 30, expected: 330 },
    { angle: 90, expected: 270 },
    { angle: 359, expected: 1 },
    { angle: -90, expected: 90 },
  ])("of $angle° is $expected°", ({ angle, expected }) => {
    expect(explement(angle)).toBe(expected);
  });

  test.prop({ angle: fc.integer({ max: 720, min: -720 }) })(
    "adds to the angle to make whole turns",
    ({ angle }) => {
      expect(degreesInTurn(explement(angle) + angle)).toBe(0);
    },
  );
});
