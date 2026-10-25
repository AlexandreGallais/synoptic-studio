import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textPickArea } from "./text-pick-area";

/** A text at (10, 20), straight, fitting its content. */
const TEXT = { ...HELLO_TEXT, angle: 0, isInBox: false, x: 10, y: 20 };

/** Its measured block. */
const BLOCK = { height: 40, width: 80 };

describe("textPickArea", () => {
  it("gives its block's corners and four sides", () => {
    const area = textPickArea(TEXT, BLOCK);

    expect(area.corners).toEqual([
      { x: 10, y: 20 },
      { x: 90, y: 20 },
      { x: 90, y: 60 },
      { x: 10, y: 60 },
    ]);

    expect(area.pieces).toEqual([
      { end: { x: 90, y: 20 }, kind: "segment", start: { x: 10, y: 20 } },
      { end: { x: 90, y: 60 }, kind: "segment", start: { x: 90, y: 20 } },
      { end: { x: 10, y: 60 }, kind: "segment", start: { x: 90, y: 60 } },
      { end: { x: 10, y: 20 }, kind: "segment", start: { x: 10, y: 60 } },
    ]);

    expect(area.isFlat).toBe(false);
  });

  it("gives its box when it sits in one, not its block", () => {
    // Box 200 × 100 at (10, 20): corners to (210, 120).
    const area = textPickArea({ ...TEXT, boxHeight: 100, boxWidth: 200, isInBox: true }, BLOCK);

    expect(area.corners[2]).toEqual({ x: 210, y: 120 });
  });

  it("marks a block 0 wide flat, an empty text", () => {
    expect(textPickArea(TEXT, { height: 40, width: 0 }).isFlat).toBe(true);
  });

  it("marks a box 0 high flat, though its block has an area", () => {
    const area = textPickArea({ ...TEXT, boxHeight: 0, boxWidth: 200, isInBox: true }, BLOCK);

    expect(area.isFlat).toBe(true);
  });
});
