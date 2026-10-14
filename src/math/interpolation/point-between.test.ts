import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { midpoint } from "./midpoint";
import { pointBetween } from "./point-between";

describe("pointBetween (REF-WIKI-DE-CASTELJAU)", () => {
  it("gives the ends at 0 and 1, and a fifth of the way at 0.2", () => {
    const [start, end] = [
      { x: 0, y: 10 },
      { x: 50, y: -40 },
    ];

    expect([pointBetween(start, end, 0), pointBetween(start, end, 1)]).toEqual([start, end]);
    expect(pointBetween(start, end, 0.2)).toEqual({ x: 10, y: 0 });
  });

  test.prop({ ax: fc.integer(), ay: fc.integer(), bx: fc.integer(), by: fc.integer() })(
    "is the midpoint bit for bit at one half",
    ({ ax, ay, bx, by }) => {
      const [start, end] = [
        { x: ax, y: ay },
        { x: bx, y: by },
      ];

      expect(pointBetween(start, end, 1 / 2)).toEqual(midpoint(start, end));
    },
  );
});
