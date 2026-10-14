import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { quarterTurn } from "./quarter-turn";
import { quarterTurns } from "./quarter-turns";

describe("quarterTurns (DERIV-turned-shape)", () => {
  it("turns (1, 0) to (0, 1), (−1, 0), (0, −1), then back", () => {
    // −0 + 0 is +0: the signs of zero aside.
    const turned = [1, 2, 3, 4].map((count) => quarterTurns({ x: 1, y: 0 }, count));

    expect(turned.map((v) => ({ x: v.x + 0, y: v.y + 0 }))).toEqual([
      { x: 0, y: 1 },
      { x: -1, y: 0 },
      { x: 0, y: -1 },
      { x: 1, y: 0 },
    ]);

    expect(turned[0]).toEqual(quarterTurn({ x: 1, y: 0 }));
  });

  test.prop({ count: fc.nat(8), v: integerPoint(1e4) })(
    "is the same after 4 more turns",
    ({ count, v }) => {
      expect(quarterTurns(v, count + 4)).toEqual(quarterTurns(v, count));
    },
  );
});
