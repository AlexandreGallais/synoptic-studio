import { describe, expect, it } from "vitest";

import { isValidRegularPolygon } from "./is-valid-regular-polygon";

import type { Alignment, Anchor } from "../geometry";

/** F02's placement: centered on both axes (F07.AC1). */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

describe("isValidRegularPolygon", () => {
  it("[F02.AC1] accepts 3 and 12 corners, the limits of Q19", () => {
    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 12,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 100,
      }),
    ).toBe(true);
  });

  it("[F02.AC1] refuses 2 corners, 13 corners and a fractional number of corners", () => {
    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 2,
        height: 100,
        radii: [0, 0],
        width: 100,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 13,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 100,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 4.5,
        height: 100,
        radii: Array.from({ length: 4.5 }, () => 0),
        width: 100,
      }),
    ).toBe(false);
  });

  it("[F02.AC1] accepts a size of 0 and refuses a negative or fractional size (Q15)", () => {
    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 6,
        height: 0,
        radii: [0, 0, 0, 0, 0, 0],
        width: 0,
      }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        width: -1,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 6,
        height: 0.5,
        radii: [0, 0, 0, 0, 0, 0],
        width: 100,
      }),
    ).toBe(false);
  });

  it("accepts a radius larger than the polygon and refuses a negative one (ADR-0007)", () => {
    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 6,
        height: 100,
        radii: [1000, 1000, 1000, 1000, 1000, 1000],
        width: 100,
      }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 6,
        height: 100,
        radii: [-1, -1, -1, -1, -1, -1],
        width: 100,
      }),
    ).toBe(false);
  });

  it("[F07.AC1] accepts the nine anchors", () => {
    const alignments = ["min", "mid", "max"] as const;

    for (const horizontal of alignments) {
      for (const vertical of alignments) {
        const anchor = { horizontal, vertical };

        expect(
          isValidRegularPolygon({
            angle: 0,
            x: 0,
            y: 0,
            anchor,
            corners: 3,
            height: 100,
            radii: [0, 0, 0],
            width: 100,
          }),
        ).toBe(true);
      }
    }
  });

  it("[F07.AC1] refuses an alignment other than min, mid and max", () => {
    // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- data read from outside may hold any string
    const left = "left" as Alignment;

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: { horizontal: left, vertical: "mid" },
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: { horizontal: "mid", vertical: left },
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
    ).toBe(false);
  });

  it("[F03.AC1] accepts one radius per corner, different ones included, and refuses another count", () => {
    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 3,
        height: 100,
        radii: [10, 0, 5],
        width: 100,
      }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 3,
        height: 100,
        radii: [10, 0],
        width: 100,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        corners: 3,
        height: 100,
        radii: [10, 0, -1],
        width: 100,
      }),
    ).toBe(false);
  });

  it.each([
    { angle: 360, isValid: false, x: 0, y: 0 },
    { angle: 2.5, isValid: false, x: 0, y: 0 },
    { angle: 0, isValid: false, x: Infinity, y: 0 },
    // A fractional position is allowed: an edit of a turned shape leaves one (ADR-0035).
    { angle: 30, isValid: true, x: 12.5, y: -7.25 },
  ])("[F04.AC3] checks the angle $angle and the position ($x, $y)", ({ isValid, ...placement }) => {
    expect(
      isValidRegularPolygon({
        ...placement,
        anchor: CENTER,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        width: 100,
      }),
    ).toBe(isValid);
  });
});
