import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { offsetOnLargerAxis } from "./offset-on-larger-axis";

describe("offsetOnLargerAxis (US-072)", () => {
  it("[F39.AC2] keeps (30.6, 19.4) across, the larger, so that A lands at (41, 20)", () => {
    expect(offsetOnLargerAxis({ x: 30.6, y: 19.4 })).toEqual({ x: 30.6, y: 0 });
  });

  it("[F39.AC2] keeps (−3, 12.5) down, the larger by its size whatever its sign", () => {
    expect(offsetOnLargerAxis({ x: -3, y: 12.5 })).toEqual({ x: 0, y: 12.5 });
    expect(offsetOnLargerAxis({ x: 3, y: -12.5 })).toEqual({ x: 0, y: -12.5 });
  });

  it("[F39.AC2] keeps a tie, (20, −20), across (agent's choice)", () => {
    expect(offsetOnLargerAxis({ x: 20, y: -20 })).toEqual({ x: 20, y: 0 });
  });

  it("keeps no offset as none", () => {
    expect(offsetOnLargerAxis({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
  });

  test.prop({
    x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
  })("keeps one axis as it is and sets the other to 0, the one kept the larger", ({ x, y }) => {
    const kept = offsetOnLargerAxis({ x, y });
    const isAcross = kept.y === 0 && kept.x === x;
    const isDown = kept.x === 0 && kept.y === y;

    expect(isAcross || isDown).toBe(true);
    expect(Math.abs(kept.x) + Math.abs(kept.y)).toBe(Math.max(Math.abs(x), Math.abs(y)));
  });
});
