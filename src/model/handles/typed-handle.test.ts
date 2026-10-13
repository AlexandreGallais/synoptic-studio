import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints, integerPoint } from "../../../tests";

import { fieldValuesOfHandle } from "./field-values-of-handle";
import { HANDLE_WAYS } from "./handle-ways";
import { typedHandle } from "./typed-handle";

/** A handle after the point (100, 0), its side going right to (200, 0). */
const AFTER = { isBefore: false, point: { x: 100, y: 0 }, side: { x: 100, y: 0 } };

/** The handle before the point (200, 0), its side going back left to (100, 0). */
const BEFORE = { isBefore: true, point: { x: 200, y: 0 }, side: { x: -100, y: 0 } };

describe("typedHandle and fieldValuesOfHandle (DERIV-handle-angle)", () => {
  it("[F12.AC4] takes x and y on the page, or an angle and a length from the side", () => {
    expect(typedHandle("page", [130, -15], AFTER)).toEqual({ x: 30, y: -15 });

    // 330° from a side going right, length 30: (30 cos 330°, 30 sin 330°) = (25.98, −15).
    expectPoints(
      [typedHandle("side", [330, 30], AFTER) ?? { x: NaN, y: NaN }],
      [{ x: 25.98076, y: -15 }],
      5,
    );
  });

  it("[F12.AC4] mirrors the handle before: the same values at both ends make an arch", () => {
    // 330°, 30 at both ends: after (25.98, −15), before (−25.98, −15), both above the side.
    expectPoints(
      [
        typedHandle("side", [330, 30], AFTER) ?? { x: NaN, y: NaN },
        typedHandle("side", [330, 30], BEFORE) ?? { x: NaN, y: NaN },
      ],
      [
        { x: 25.98076, y: -15 },
        { x: -25.98076, y: -15 },
      ],
      5,
    );
  });

  it("[F12.AC4] brings an angle into 0 to 359, and refuses a negative length or a side of length 0", () => {
    // 690° is 330°; −30° is 330° too.
    expect(typedHandle("side", [690, 30], AFTER)?.y).toBeCloseTo(-15, 9);
    expect(typedHandle("side", [-30, 30], AFTER)?.y).toBeCloseTo(-15, 9);
    expect(typedHandle("side", [0, -1], AFTER)).toBeUndefined();
    expect(typedHandle("side", [0, 10], { ...AFTER, side: { x: 0, y: 0 } })).toBeUndefined();
  });

  it("shows a handle in each way", () => {
    const handle = { x: 30, y: -15 };

    expect(fieldValuesOfHandle("page", handle, AFTER)).toEqual([130, -15]);

    expect(fieldValuesOfHandle("side", { x: -25.980762113533157, y: -15 }, BEFORE)[0]).toBeCloseTo(
      330,
      9,
    );
  });

  test.prop({
    handle: integerPoint(500),
    side: integerPoint(500),
    isBefore: fc.boolean(),
    way: fc.constantFrom(...HANDLE_WAYS),
  })("gives back the handle shown, in every way", ({ handle, isBefore, side, way }) => {
    fc.pre(side.x !== 0 || side.y !== 0);

    const frame = { isBefore, point: { x: 7, y: -3 }, side };
    const back = typedHandle(way, fieldValuesOfHandle(way, handle, frame), frame);

    expect(back?.x).toBeCloseTo(handle.x, 6);
    expect(back?.y).toBeCloseTo(handle.y, 6);
  });

  it("[F12.AC4] accepts a length of 0, and reads −90° as 270° and 370° as 10°", () => {
    expect(typedHandle("side", [330, 0], AFTER)).toEqual({ x: 0, y: 0 });

    expect(
      fieldValuesOfHandle(
        "side",
        typedHandle("side", [-90, 30], AFTER) ?? { x: 0, y: 0 },
        AFTER,
      )[0],
    ).toBeCloseTo(270, 9);

    expect(
      fieldValuesOfHandle(
        "side",
        typedHandle("side", [370, 30], AFTER) ?? { x: 0, y: 0 },
        AFTER,
      )[0],
    ).toBeCloseTo(10, 9);

    expect(typedHandle("side", [370, 30], AFTER)?.x).toBeCloseTo(29.54423, 5);
  });

  it("counts the page way from the point on both axes, and reads a handle before along its side as 0", () => {
    const raised = { isBefore: false, point: { x: 100, y: 20 }, side: { x: 100, y: 0 } };

    expect(typedHandle("page", [130, -15], raised)).toEqual({ x: 30, y: -35 });
    expect(fieldValuesOfHandle("page", { x: 30, y: -15 }, raised)).toEqual([130, 5]);
    expect(fieldValuesOfHandle("side", { x: -10, y: 0 }, BEFORE)).toEqual([0, 10]);
  });
});
