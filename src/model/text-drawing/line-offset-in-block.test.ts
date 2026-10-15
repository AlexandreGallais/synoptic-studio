import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { lineOffsetInBlock } from "./line-offset-in-block";

describe("lineOffsetInBlock (REF-CSS-TEXT-3 §6.1, DERIV-text-width step 5)", () => {
  it("[F05.AC4] places 'Hello' in the block of 'Hello' and 'World', Arial 16", () => {
    // Block 41.78125, line 36.4609375: room 5.3203125; none of it, half, all of it before.
    expect(lineOffsetInBlock(41.78125, 36.4609375, "left")).toBe(0);
    expect(lineOffsetInBlock(41.78125, 36.4609375, "center")).toBe(2.66015625);
    expect(lineOffsetInBlock(41.78125, 36.4609375, "right")).toBe(5.3203125);
  });

  it("gives the widest line no offset, whatever the alignment", () => {
    for (const alignment of ["left", "center", "right"] as const) {
      expect(lineOffsetInBlock(41.78125, 41.78125, alignment)).toBe(0);
    }
  });

  it("centers a line narrower than nothing in a block of its widest line, 10", () => {
    // Lines 10 and −8: block 10; room 18 and 0, half of each before the line.
    expect(lineOffsetInBlock(10, -8, "center")).toBe(9);
    expect(lineOffsetInBlock(10, 10, "center")).toBe(0);
  });

  it("moves a line narrower than nothing past the right edge when aligned right", () => {
    // Block 0 (every line negative), line −8: room 8.
    expect(lineOffsetInBlock(0, -8, "right")).toBe(8);
  });

  const width = fc.double({ max: 100_000, min: 0, noNaN: true });

  test.prop({ line: width, room: width })(
    "ends a right-aligned line on the block's right edge, a centered one halfway",
    ({ line, room }) => {
      // The block is at least as wide as each of its lines.
      const block = line + room;
      const left = lineOffsetInBlock(block, line, "left");
      const right = lineOffsetInBlock(block, line, "right");

      expect(left).toBeCloseTo(0, 9);
      expect(right + line).toBeCloseTo(block, 6);
      expect(lineOffsetInBlock(block, line, "center")).toBeCloseTo(right / 2, 6);
    },
  );
});
