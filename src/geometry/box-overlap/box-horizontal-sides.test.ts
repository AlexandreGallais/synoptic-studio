import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boxHorizontalSides } from "./box-horizontal-sides";

describe("boxHorizontalSides (DERIV-box-overlap step 4)", () => {
  it("gives the top side, then the bottom side, each from left to right", () => {
    expect(boxHorizontalSides({ maxX: 60, maxY: -25, minX: 40, minY: -35 })).toEqual([
      { end: { x: 60, y: -35 }, kind: "segment", start: { x: 40, y: -35 } },
      { end: { x: 60, y: -25 }, kind: "segment", start: { x: 40, y: -25 } },
    ]);
  });
});

describe("boxHorizontalSides (properties)", () => {
  test.prop({ maxX: fc.integer(), maxY: fc.integer(), minX: fc.integer(), minY: fc.integer() })(
    "keeps each side flat, the top at minY and the bottom at maxY, both from minX to maxX",
    (box) => {
      const [top, bottom] = boxHorizontalSides(box);

      expect([top.start.y, top.end.y, bottom.start.y, bottom.end.y]).toEqual([
        box.minY,
        box.minY,
        box.maxY,
        box.maxY,
      ]);

      expect([top.start.x, top.end.x, bottom.start.x, bottom.end.x]).toEqual([
        box.minX,
        box.maxX,
        box.minX,
        box.maxX,
      ]);
    },
  );
});
