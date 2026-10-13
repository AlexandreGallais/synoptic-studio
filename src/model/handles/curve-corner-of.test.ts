import { describe, expect, it } from "vitest";

import { curveCornerOf } from "./curve-corner-of";
import { NO_HANDLES } from "./no-handles";

describe("curveCornerOf", () => {
  it("[F12.AC2] gives a corner its handles off, in mode angle and length, both (0, 0)", () => {
    expect(curveCornerOf({ point: { x: 100, y: 0 }, radius: 30 })).toEqual({
      handles: {
        after: { x: 0, y: 0 },
        before: { x: 0, y: 0 },
        isOn: false,
        mode: "angle-and-length",
      },
      point: { x: 100, y: 0 },
      radius: 30,
    });

    expect(curveCornerOf({ point: { x: 0, y: 0 }, radius: 0 }).handles).toBe(NO_HANDLES);
  });
});
