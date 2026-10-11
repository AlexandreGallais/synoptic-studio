import { describe, expect, it } from "vitest";

import { isAnchor } from "./is-anchor";

import type { Alignment } from "../geometry";

describe("isAnchor", () => {
  it("[F07.AC1] accepts min, mid and max on both axes", () => {
    expect(isAnchor({ horizontal: "min", vertical: "max" })).toBe(true);
    expect(isAnchor({ horizontal: "mid", vertical: "mid" })).toBe(true);
    expect(isAnchor({ horizontal: "max", vertical: "min" })).toBe(true);
  });

  it("[F07.AC1] refuses any other value on either axis", () => {
    // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- data read from outside may hold any string
    const bottom = "bottom" as Alignment;

    expect(isAnchor({ horizontal: bottom, vertical: "mid" })).toBe(false);
    expect(isAnchor({ horizontal: "mid", vertical: bottom })).toBe(false);
  });
});
