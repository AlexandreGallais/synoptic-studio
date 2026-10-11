import { describe, expect, it } from "vitest";

import { isValidRegularPolygon } from "./is-valid-regular-polygon";

import type { Alignment, Anchor } from "../geometry";

/** F02's placement: centered on both axes (F07.AC1). */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

describe("isValidRegularPolygon", () => {
  it("[F02.AC1] accepts 3 and 12 corners, the limits of Q19", () => {
    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 3, height: 100, radius: 0, width: 100 }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 12, height: 100, radius: 0, width: 100 }),
    ).toBe(true);
  });

  it("[F02.AC1] refuses 2 corners, 13 corners and a fractional number of corners", () => {
    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 2, height: 100, radius: 0, width: 100 }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 13, height: 100, radius: 0, width: 100 }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 4.5, height: 100, radius: 0, width: 100 }),
    ).toBe(false);
  });

  it("[F02.AC1] accepts a size of 0 and refuses a negative or fractional size (Q15)", () => {
    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 6, height: 0, radius: 0, width: 0 }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 6, height: 100, radius: 0, width: -1 }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 6, height: 0.5, radius: 0, width: 100 }),
    ).toBe(false);
  });

  it("accepts a radius larger than the polygon and refuses a negative one (ADR-0007)", () => {
    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 6, height: 100, radius: 1000, width: 100 }),
    ).toBe(true);

    expect(
      isValidRegularPolygon({ anchor: CENTER, corners: 6, height: 100, radius: -1, width: 100 }),
    ).toBe(false);
  });

  it("[F07.AC1] accepts the nine anchors", () => {
    const alignments = ["min", "mid", "max"] as const;

    for (const horizontal of alignments) {
      for (const vertical of alignments) {
        const anchor = { horizontal, vertical };

        expect(
          isValidRegularPolygon({ anchor, corners: 3, height: 100, radius: 0, width: 100 }),
        ).toBe(true);
      }
    }
  });

  it("[F07.AC1] refuses an alignment other than min, mid and max", () => {
    // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- data read from outside may hold any string
    const left = "left" as Alignment;

    expect(
      isValidRegularPolygon({
        anchor: { horizontal: left, vertical: "mid" },
        corners: 3,
        height: 100,
        radius: 0,
        width: 100,
      }),
    ).toBe(false);

    expect(
      isValidRegularPolygon({
        anchor: { horizontal: "mid", vertical: left },
        corners: 3,
        height: 100,
        radius: 0,
        width: 100,
      }),
    ).toBe(false);
  });
});
