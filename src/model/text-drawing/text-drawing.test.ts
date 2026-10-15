import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, DECORATED_METRICS, HELLO_TEXT } from "../../../tests";

import { textDrawing } from "./text-drawing";

import type { Text } from "../text";

/** "Hello World" in Arial 16 without kerning, in a box 60 × 60 at (0, 0). */
const IN_BOX: Text = { ...HELLO_TEXT, boxHeight: 60, boxWidth: 60, isInBox: true };

describe("textDrawing (ADR-0052, DERIV-text-width steps 4 to 6)", () => {
  it.each([
    { content: "Hello World", name: "no space" },
    { content: "Hello World ", name: "a space" },
    { content: "Hello World  ", name: "two spaces" },
  ])(
    "[F17.AC6] ends each band of 'Hello World' with $name at the end with its word, in a box 50",
    ({ content }) => {
      const text = { ...IN_BOX, boxWidth: 50, content, hasBackground: true };

      // "Hello" 36.4609375 and "World" 41.78125 wide, never 46.2265625 with a space (569 × 16 /
      // 2048 = 4.4453125), nor 50.671875 with two, past the box.
      expect(textDrawing(ARIAL_BOX_METRICS, text).backgrounds.map((band) => band.width)).toEqual([
        36.4609375, 41.78125,
      ]);
    },
  );

  it("[F17.AC6] places 'World ' right-aligned at the box's edge, its space ignored", () => {
    const text = {
      ...IN_BOX,
      alignment: "right" as const,
      boxWidth: 50,
      content: "Hello World ",
      hasBackground: true,
    };
    const drawn = textDrawing(ARIAL_BOX_METRICS, text);

    // 50 − 41.78125 = 8.21875, not 50 − 46.2265625 = 3.7734375.
    expect(drawn.lines[1]?.x).toBe(8.21875);
    expect(drawn.backgrounds[1]).toMatchObject({ width: 41.78125, x: 8.21875 });
  });

  it("[F05.AC4] underlines each line that has a length, an empty line in between", () => {
    const text = { ...HELLO_TEXT, content: "Hello\n\nWorld", isUnderlined: true };
    const drawn = textDrawing(DECORATED_METRICS, text);

    // Baselines 14.484375 and 14.484375 + 2 × 18.3984375 = 51.28125; underlines 2 below them.
    expect(drawn.lines).toHaveLength(3);

    expect(drawn.decorations).toEqual([
      { color: "#000000", height: 1, opacity: 100, width: 36.4609375, x: 0, y: 16.484375 },
      { color: "#000000", height: 1, opacity: 100, width: 41.78125, x: 0, y: 53.28125 },
    ]);
  });

  it("[F05.AC4] keeps a trailing empty line: a second baseline, no stroke, two lines tall", () => {
    const drawn = textDrawing(DECORATED_METRICS, {
      ...HELLO_TEXT,
      content: "Hello\n",
      isUnderlined: true,
    });

    // 14.484375 + 18.3984375 = 32.8828125; block 2 × 18.3984375 = 36.796875 tall.
    expect(drawn.lines.map((line) => line.y)).toEqual([14.484375, 32.8828125]);
    expect(drawn.decorations).toHaveLength(1);
    expect(drawn.block).toEqual({ height: 36.796875, width: 36.4609375 });
  });

  it("[F05.AC4] starts each stroke of a right-to-left text at its line's left edge", () => {
    const drawn = textDrawing(DECORATED_METRICS, {
      ...HELLO_TEXT,
      alignment: "right",
      content: "Hello\nWorld",
      direction: "rtl",
      isUnderlined: true,
    });

    // 'Hello' from 41.78125 − 36.4609375 = 5.3203125 to 41.78125, where its x is written.
    expect(drawn.lines.map((line) => line.x)).toEqual([41.78125, 41.78125]);
    expect(drawn.decorations.map((stroke) => stroke.x)).toEqual([5.3203125, 0]);
  });

  it("[F05.AC5] turns about the middle of the measured block, its position added", () => {
    const drawn = textDrawing(DECORATED_METRICS, {
      ...HELLO_TEXT,
      angle: 90,
      content: "Hello\nWorld",
      x: 10,
      y: 20,
    });

    // Block 41.78125 × 2 × 18.3984375 = 36.796875: middle (10 + 20.890625, 20 + 18.3984375).
    expect(drawn.center).toEqual({ x: 30.890625, y: 38.3984375 });
  });

  test.prop({
    content: fc.string({ maxLength: 12 }),
    x: fc.integer({ max: 100_000, min: -100_000 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("[F05.AC5] keeps the middle half a block from the position", ({ content, x, y }) => {
    const { block, center } = textDrawing(DECORATED_METRICS, { ...HELLO_TEXT, content, x, y });

    expect(center.x - x).toBeCloseTo(block.width / 2, 9);
    expect(center.y - y).toBeCloseTo(block.height / 2, 9);
  });

  it.each([
    // h(2) = 36.796875 in a box 60 tall: room 23.203125. Middle: 11.6015625 + 14.484375 =
    // 26.0859375, then 18.3984375 lower; bottom: 23.203125 + 14.484375 = 37.6875.
    { baselines: [14.484375, 32.8828125], isTrimmed: false, verticalAlignment: "top" },
    { baselines: [26.0859375, 44.484375], isTrimmed: false, verticalAlignment: "middle" },
    { baselines: [37.6875, 56.0859375], isTrimmed: false, verticalAlignment: "bottom" },
    // Trimmed, h(2) = 29.859375: (60 − 29.859375) / 2 + 11.4609375 = 26.53125.
    { baselines: [26.53125, 44.9296875], isTrimmed: true, verticalAlignment: "middle" },
  ] as const)(
    "[F14.AC3] places 'Hello' over 'World' in a box 50 × 60 at $verticalAlignment, trimmed: $isTrimmed",
    ({ baselines, isTrimmed, verticalAlignment }) => {
      // "Hello World" wraps into 2 lines in a box 50 wide: n = min(3, ∞, 2) = 2.
      const text = { ...IN_BOX, boxWidth: 50, isTrimmed, verticalAlignment };

      expect(textDrawing(ARIAL_BOX_METRICS, text).lines.map((line) => line.y)).toEqual(baselines);
    },
  );

  it.each([
    // "Hello" 36.4609375 and "World" 41.78125 wide across 60: room 23.5390625 and 18.21875.
    { alignment: "left", xs: [10, 10] },
    { alignment: "center", xs: [21.76953125, 19.109375] },
    { alignment: "right", xs: [33.5390625, 28.21875] },
  ] as const)(
    "[F14.AC3] aligns the lines $alignment across a box 60 wide at x = 10",
    ({ alignment, xs }) => {
      const text = { ...IN_BOX, alignment, content: "Hello\nWorld", x: 10 };

      expect(textDrawing(ARIAL_BOX_METRICS, text).lines.map((line) => line.x)).toEqual(xs);
    },
  );

  it("[F14.AC3] turns a text in a box 60 × 40 at (0, 0) about (30, 20), its box's middle", () => {
    const text = { ...IN_BOX, angle: 90, boxHeight: 40 };

    expect(textDrawing(ARIAL_BOX_METRICS, text).center).toEqual({ x: 30, y: 20 });
  });

  it("[F14.AC6] leaves a text that fits its content at the top of its block", () => {
    // The vertical alignment is read in a box only: the first baseline stays 14.484375.
    const text = { ...HELLO_TEXT, boxHeight: 60, verticalAlignment: "bottom" } as const;

    expect(textDrawing(ARIAL_BOX_METRICS, text).lines.map((line) => line.y)).toEqual([14.484375]);
  });

  test.prop({
    boxHeight: fc.integer({ max: 1000, min: 0 }),
    boxWidth: fc.integer({ max: 1000, min: 0 }),
    x: fc.integer({ max: 100_000, min: -100_000 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("[F14.AC3] turns a text in a box about its box's middle, whatever its lines", (box) => {
    const { center } = textDrawing(ARIAL_BOX_METRICS, { ...IN_BOX, ...box });

    expect(center.x).toBeCloseTo(box.x + box.boxWidth / 2, 9);
    expect(center.y).toBeCloseTo(box.y + box.boxHeight / 2, 9);
  });

  it("[F14.AC4] puts a background behind 'Hello' over 'World', ascender to descender", () => {
    const text = { ...HELLO_TEXT, content: "Hello\nWorld", hasBackground: true };

    // Baselines 14.484375 and 32.8828125, ascent 1854 × 16 / 2048 = 14.484375, height
    // (1854 + 434) / 128 = 17.875 (DERIV-text-box, checks).
    expect(textDrawing(ARIAL_BOX_METRICS, text).backgrounds).toEqual([
      { height: 17.875, width: 36.4609375, x: 0, y: 0 },
      { height: 17.875, width: 41.78125, x: 0, y: 18.3984375 },
    ]);
  });

  it.each([
    // Line height 24: half the leading (24 − 17.875) / 2 = 3.0625, so the first baseline
    // 3.0625 + 14.484375 = 17.546875 and the band's top 3.0625; the second 24 lower, 27.0625.
    { isTrimmed: false, lineHeight: 24, tops: [3.0625, 27.0625] },
    // Trimmed: the first baseline at the cap height 1467 × 16 / 2048 = 11.4609375, the band's
    // top 11.4609375 − 14.484375 = −3.0234375, above the block; the second 18.3984375 lower.
    { isTrimmed: true, lineHeight: 0, tops: [-3.0234375, 15.375] },
  ])(
    "[F14.AC4] keeps each band from ascender to descender, line height $lineHeight, trimmed: $isTrimmed",
    ({ isTrimmed, lineHeight, tops }) => {
      const text = {
        ...HELLO_TEXT,
        content: "Hello\nWorld",
        hasBackground: true,
        isTrimmed,
        lineHeight,
      };

      expect(textDrawing(ARIAL_BOX_METRICS, text).backgrounds).toEqual([
        { height: 17.875, width: 36.4609375, x: 0, y: tops[0] },
        { height: 17.875, width: 41.78125, x: 0, y: tops[1] },
      ]);
    },
  );

  it("[F14.AC4] runs a cut line's background to the end of its ellipsis", () => {
    // "Hello World" in a box 60 wide, 1 line: "Hello…", (4667 + 2048) / 128 = 52.4609375 wide.
    const text = { ...IN_BOX, hasBackground: true, maximumLines: 1 };
    const drawn = textDrawing(ARIAL_BOX_METRICS, text);

    expect(drawn.lines.map((line) => line.content)).toEqual(["Hello…"]);
    expect(drawn.backgrounds).toEqual([{ height: 17.875, width: 52.4609375, x: 0, y: 0 }]);
  });

  it("[F14.AC4] gives an empty line no background", () => {
    const text = { ...HELLO_TEXT, content: "Hello\n\nWorld", hasBackground: true };

    // The third line's baseline 14.484375 + 2 × 18.3984375 = 51.28125, its top 36.796875.
    expect(
      textDrawing(ARIAL_BOX_METRICS, text).backgrounds.map((rectangle) => rectangle.y),
    ).toEqual([0, 36.796875]);
  });

  it("[F14.AC6] draws no background for a text without one", () => {
    expect(textDrawing(ARIAL_BOX_METRICS, IN_BOX).backgrounds).toEqual([]);
  });

  it("draws an empty text as nothing, its middle at its position", () => {
    expect(
      textDrawing(DECORATED_METRICS, {
        ...HELLO_TEXT,
        content: "",
        hasBackground: true,
        isUnderlined: true,
        x: 3,
        y: 4,
      }),
    ).toEqual({
      backgrounds: [],
      block: { height: 0, width: 0 },
      center: { x: 3, y: 4 },
      decorations: [],
      lines: [],
    });
  });
});
