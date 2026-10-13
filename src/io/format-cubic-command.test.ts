import { describe, expect, it } from "vitest";

import { formatCubicCommand } from "./format-cubic-command";

describe("formatCubicCommand (REF-SVG2-PATHS §9.3.6)", () => {
  it("[F12.AC3] writes C, the two control points and the end, at fixed precision", () => {
    // (125.98076211353316, −15.000000000000004) is written 125.98076 −15 (SVG_DECIMALS = 5).
    expect(
      formatCubicCommand({
        end: { x: 200, y: 0 },
        first: { x: 125.98076211353316, y: -15.000000000000004 },
        kind: "cubic",
        second: { x: 200, y: 0 },
        start: { x: 100, y: 0 },
      }),
    ).toBe("C125.98076 -15 200 0 200 0");
  });
});
